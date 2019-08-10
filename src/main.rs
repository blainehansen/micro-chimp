extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[macro_use] extern crate derive_error;

extern crate actix;
extern crate actix_web;
extern crate futures;
use actix::prelude::*;
use actix_web::{
	client as http_client, http::{StatusCode, header::HeaderValue}, server, App, AsyncResponder,
	HttpMessage, HttpRequest, HttpResponse, error::ResponseError,
};
use futures::{Future, IntoFuture};

extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize, Deserializer};

extern crate tokio_postgres;
use tokio_postgres::{Client, Statement};

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[macro_use] extern crate lazy_static;

extern crate base64;

extern crate rand;
use rand::Rng;
use rand::rngs::OsRng;

mod sites;
use sites::SiteName;


fn base64_encode(s: &[u8]) -> String {
	base64::encode_config(s, base64::URL_SAFE)
}

pub fn generate_random_token() -> Option<String> {
	let mut r = OsRng::new().ok()?;
	let mut buf: [u8; 64] = [0; 64];
	r.fill(&mut buf);

	Some(base64_encode(&buf[..]))
}


struct PgConnection {
	client: Option<Client>,
	new_email_query: Option<Statement>,
	verify_query: Option<Statement>,
	unsubscribe_query: Option<Statement>,
}

impl Actor for PgConnection {
	type Context = Context<Self>;
}

const NEW_EMAIL_QUERY: &'static str = "insert into subscription (email, site_name, validation_token) values ($1, $2::site_name_enum, $3)";
const VERIFY_QUERY: &'static str = "update subscription set validation_token = null where validation_token = $1";
const UNSUBSCRIBE_QUERY: &'static str = "update subscription set unsubscribed_with = $1 where email = $2 and site_name = $3::site_name_enum";

impl PgConnection {
	pub fn connect(db_url: &str) -> Addr<PgConnection> {
		let connection_attempt = tokio_postgres::connect(db_url, tokio_postgres::tls::NoTls);

		PgConnection::create(move |ctx| {
			let act = PgConnection { client: None, new_email_query: None, verify_query: None, unsubscribe_query: None };

			connection_attempt.map_err(|_| panic!("can not connect to postgresql"))
				.into_actor(&act)
				.and_then(|(mut client, connection), act, ctx| {
					Arbiter::spawn(connection.map_err(|e| panic!("{}", e)));

					use tokio_postgres::types::{Type as Pg};
					ctx.wait(
						client.prepare_typed(NEW_EMAIL_QUERY, &[Pg::TEXT, Pg::TEXT, Pg::TEXT])
							.map_err(|_| panic!("couldn't prepare NEW_EMAIL_QUERY"))
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.new_email_query = Some(statement);
								fut::ok(())
							})
					);

					ctx.wait(
						client.prepare_typed(VERIFY_QUERY, &[Pg::TEXT])
							.map_err(|_| panic!("couldn't prepare VERIFY_QUERY"))
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.verify_query = Some(statement);
								fut::ok(())
							})
					);

					ctx.wait(
						client.prepare_typed(UNSUBSCRIBE_QUERY, &[Pg::TEXT, Pg::TEXT, Pg::TEXT])
							.map_err(|_| panic!("couldn't prepare UNSUBSCRIBE_QUERY"))
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.unsubscribe_query = Some(statement);
								fut::ok(())
							})
					);

					act.client = Some(client);
					fut::ok(())
				})
				.wait(ctx);

			act
		})
	}
}


fn empty_status(code: StatusCode) -> HttpResponse {
	HttpResponse::with_body(code, actix_web::Body::Empty)
}

fn respond_success() -> Result<HttpResponse, GenericError> {
	Ok(empty_status(StatusCode::NO_CONTENT))
}


#[derive(Debug, Error)]
pub enum GenericError {
	NoContent,
	BadRequest,
	Unprocessable,
	InternalServer,
}

impl ResponseError for GenericError {
	fn error_response(&self) -> HttpResponse {
		match *self {
			GenericError::NoContent => empty_status(StatusCode::NO_CONTENT),
			GenericError::BadRequest => empty_status(StatusCode::BAD_REQUEST),
			GenericError::Unprocessable => empty_status(StatusCode::UNPROCESSABLE_ENTITY),
			GenericError::InternalServer => empty_status(StatusCode::INTERNAL_SERVER_ERROR),
		}
	}
}


impl From<tokio_postgres::Error> for GenericError {
	fn from(error: tokio_postgres::Error) -> Self {
		let c = error.code();
		if c == Some(&tokio_postgres::error::SqlState::INTEGRITY_CONSTRAINT_VIOLATION) {
			GenericError::BadRequest
		}
		else if c == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
			GenericError::NoContent
		}
		else {
			GenericError::InternalServer
		}
	}
}

impl From<actix::MailboxError> for GenericError {
	fn from(_: actix::MailboxError) -> Self {
		GenericError::InternalServer
	}
}

impl From<actix_web::error::JsonPayloadError> for GenericError {
	fn from(error: actix_web::error::JsonPayloadError) -> Self {
		match error {
			actix_web::error::JsonPayloadError::Deserialize(_) => GenericError::Unprocessable,
			_ => GenericError::BadRequest,
		}
	}
}


#[derive(Debug, Serialize)]
pub struct MailgunForm {
	to: String,
	text: String,
	from: &'static str,
	subject: &'static str,
}


#[derive(Debug, Validate, Deserialize)]
struct NewEmailJsonInput {
	#[validate(email)]
	email: String,
	site_name: SiteName,
}

#[derive(Debug)]
struct NewEmailMessage {
	site_name: &'static str,
	validation_token: String,
	mailgun_url: &'static str,
	mailgun_form: MailgunForm,
}

impl NewEmailJsonInput {
	fn into_message(self) -> Result<NewEmailMessage, GenericError> {
		self.validate().ok().ok_or(GenericError::BadRequest)?;
		let validation_token = generate_random_token().ok_or(GenericError::InternalServer)?;

		let (mailgun_url, string_site_name, mailgun_form) = self.site_name.get_site_information(self.email, &validation_token);

		let msg = NewEmailMessage {
			site_name: string_site_name,
			validation_token,
			mailgun_url,
			mailgun_form,
		};

		Ok(msg)
	}
}

impl Message for NewEmailMessage {
	type Result = Result<NewEmailMessage, GenericError>;
}

impl Handler<NewEmailMessage> for PgConnection {
	type Result = ResponseFuture<NewEmailMessage, GenericError>;

	fn handle(&mut self, msg: NewEmailMessage, _: &mut Self::Context) -> Self::Result {
		Box::new(
			self.client
				.as_mut().unwrap()
				// .execute(self.new_email_query.as_ref().unwrap(), msg.make_insert_args().as_ref())
				.execute(self.new_email_query.as_ref().unwrap(), &[&msg.mailgun_form.to, &msg.site_name, &msg.validation_token])
				.into_future()
				.from_err()
				.and_then(move |rows| match rows {
					1 => Ok(msg),
					0 => Err(GenericError::NoContent),
					_ => Err(GenericError::InternalServer),
				})
		)
	}
}

fn new_email(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = GenericError> {
	let db = req.state().db.clone();
	req.json()
		.from_err()
		.and_then(move |json_input: NewEmailJsonInput| {
			json_input.into_message()
				.into_future()
				.from_err()
				.and_then(move |msg| {
					db.send(msg)
						.from_err()
						.and_then(|msg_res| {
							msg_res
								.into_future()
								.from_err()
								.and_then(|msg| {
									http_client::post(msg.mailgun_url)
										.header(actix_web::http::header::AUTHORIZATION, MAILGUN_AUTH.to_owned())
										.form(msg.mailgun_form)
										.unwrap()
										.send()
										.then(|res| match dbg!(res) {
											Ok(ref r) if r.status().is_success() => respond_success(),
											_ => Err(GenericError::InternalServer)
										})
							})
					})
			})
		})
		.responder()
}


fn from_base64<'d, D>(deserializer: D) -> Result<String, D::Error>
	where D: Deserializer<'d>
{
	use serde::de::Error;
	let de = String::deserialize(deserializer)?;
	let buf = base64::decode_config(&de, base64::URL_SAFE).map_err(|_| Error::custom(""))?;
	String::from_utf8(buf).map_err(|_| Error::custom(""))
}

#[derive(Debug, Deserialize)]
struct VerifyEmailMessage {
	validation_token: String,
}

impl Message for VerifyEmailMessage {
	type Result = Result<(), GenericError>;
}

impl Handler<VerifyEmailMessage> for PgConnection {
	type Result = ResponseFuture<(), GenericError>;

	fn handle(&mut self, msg: VerifyEmailMessage, _: &mut Self::Context) -> Self::Result {
		Box::new(
			self.client
				.as_mut().unwrap()
				.execute(self.verify_query.as_ref().unwrap(), &[&msg.validation_token])
				.into_future()
				.from_err()
				.and_then(|rows| match rows {
					1 => Ok(()),
					0 => Err(GenericError::NoContent),
					_ => Err(GenericError::InternalServer),
				})
		)
	}
}

fn verify_email(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = GenericError> {
	let db = req.state().db.clone();
	req.json()
		.from_err()
		.and_then(move |msg: VerifyEmailMessage| {
			db.send(msg)
				.from_err()
				.and_then(|msg_res| {
					msg_res
						.into_future()
						.from_err()
						.and_then(|_| respond_success())
				})
		})
		.responder()
}



#[derive(Debug, Deserialize)]
struct UnsubscribeMessage {
	#[serde(deserialize_with = "from_base64")]
	email: String,
	site_name: SiteName,
	unsubscribed_with: String,
}

impl Message for UnsubscribeMessage {
	type Result = Result<(), GenericError>;
}

impl Handler<UnsubscribeMessage> for PgConnection {
	type Result = ResponseFuture<(), GenericError>;

	fn handle(&mut self, msg: UnsubscribeMessage, _: &mut Self::Context) -> Self::Result {
		let (_, site_name, _) = msg.site_name.get_site_information("".to_string(), "");
		Box::new(
			self.client
				.as_mut().unwrap()
				.execute(self.unsubscribe_query.as_ref().unwrap(), &[&msg.unsubscribed_with, &msg.email, &site_name])
				.into_future()
				.from_err()
				.and_then(|rows| match rows {
					1 => Ok(()),
					0 => Err(GenericError::NoContent),
					_ => Err(GenericError::InternalServer),
				})
		)
	}
}


fn unsubscribe(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = GenericError> {
	let db = req.state().db.clone();
	req.json()
		.from_err()
		.and_then(move |msg: UnsubscribeMessage| {
			db.send(msg)
				.from_err()
				.and_then(|msg_res| {
					msg_res
						.into_future()
						.from_err()
						.and_then(|_| respond_success())
				})
		})
		.responder()
}


struct State {
	db: Addr<PgConnection>,
}


// fn env_to_file_to_string(env_var: &'static str) -> String {
// 	use std::fs::File;
// 	use std::io::Read;

// 	let file_name = std::env::var(env_var).expect(format!("{} isn't set", env_var).as_str());
// 	let mut file = File::open(&file_name).expect(format!("there was a problem finding {}", file_name).as_str());
// 	let mut contents = String::new();
// 	file.read_to_string(&mut contents).expect(format!("couldn't read contents of {}", file_name).as_str());
// 	contents.trim().to_string()
// }

fn get_env(env_var: &'static str) -> String {
	std::env::var(env_var)
		.expect(format!("{} isn't set", env_var).as_str())
		.trim()
		.to_string()
}

lazy_static! {
	static ref MAILGUN_AUTH: HeaderValue = {
		// let contents = env_to_file_to_string("MAILGUN_AUTH_FILE");
		let contents = get_env("MAILGUN_AUTH");
		let auth = base64_encode(contents.trim().as_bytes());
		HeaderValue::from_bytes(format!("Basic {}", auth).as_bytes()).expect("couldn't construct valid header")
	};
}

fn main() {
	std::thread::sleep(std::time::Duration::from_secs(5));

	assert!(MAILGUN_AUTH.to_owned() != "");

	std::env::set_var("RUST_LOG", "micro_chimp=info");
	pretty_env_logger::init();

	let user = "rust_server_user";
	// let pass = env_to_file_to_string("POSTGRES_PASSWORD_FILE");
	let pass = get_env("SERVER_POSTGRES_PASSWORD");

	let db_url = format!("postgres://{}:{}@database/database", user, pass);

	// start http server
	let sys = System::new("micro_chimp");
	server::new(move || {
		let addr = PgConnection::connect(db_url.as_str());

		App::with_state(State { db: addr })
			.resource("/new-email", |r| r.post().a(new_email))
			.resource("/verify-email", |r| r.post().a(verify_email))
			.resource("/unsubscribe", |r| r.post().a(unsubscribe))
	})
		// .backlog(8192)
		.bind("127.0.0.1:5050")
		.unwrap()
		.start();

	info!("Started http server: 127.0.0.1:5050");
	let _ = sys.run();
}
