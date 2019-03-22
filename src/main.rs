extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[macro_use] extern crate derive_error;

extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
extern crate serde_json;
extern crate checkmail;

extern crate tokio_postgres;

// #[macro_use] extern crate postgres;
// #[macro_use] extern crate postgres_derive;

extern crate rand;
extern crate base64;

use actix::prelude::*;
use actix_web::{
	client as http_client, http::StatusCode, server, App, AsyncResponder,
	HttpMessage, HttpRequest, HttpResponse, error::ResponseError,
};
use futures::{Future, future, IntoFuture};

mod utils;
use utils::{generate_random_token, base64_encode, base64_decode};

mod args;

use serde::{Deserialize, Serialize};

// use tokio_postgres::{Client, Statement, types::{ToSql as TokioToSql, FromSql as TokioFromSql}};
use tokio_postgres::{Client, Statement};
// use postgres::{types::{ToSql, FromSql, IsNull, Type}};


struct PgConnection {
	client: Option<Client>,
	insert_new_email: Option<Statement>,
	verify_existing: Option<Statement>,
}

impl Actor for PgConnection {
	type Context = Context<Self>;
}

struct State {
	db: Addr<PgConnection>,
}

impl PgConnection {
	pub fn connect(db_url: &str) -> Addr<PgConnection> {
		let hs = tokio_postgres::connect(db_url, tokio_postgres::tls::NoTls);

		PgConnection::create(move |ctx| {
			let act = PgConnection {
				client: None,
				insert_new_email: None,
				verify_existing: None,
			};

			// let (client, [insert_new_email, verify_existing]) = hs
			// 	.map_err(|_| panic!("{:?}", ))
			// 	.and_then(|mut client, conn| {
			// 		Arbiter::spawn(conn.map_err(|e| panic!("{}", e)));

			// 		future::join_all([
			// 			client.prepare(args::NEW_EMAIL_QUERY)
			// 				.map_err(|_| panic!("{:?}", )),
			// 			client.prepare(args::VERIFY_QUERY)
			// 				.map_err(|_| panic!("{:?}", )),
			// 		])
			// 			.map_err(|_| panic!("{:?}", ))
			// 			.and_then(move |statements| {
			// 				fut::ok((client, statements))
			// 			})
			// 	})
			// 	.wait(ctx);

			// PgConnection { client, insert_new_email, verify_existing }

			hs.map_err(|_| panic!("can not connect to postgresql"))
				.into_actor(&act)
				.and_then(|(mut client, conn), act, ctx| {
					ctx.wait(
						client.prepare(args::NEW_EMAIL_QUERY)
							.map_err(|_| ())
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.insert_new_email = Some(statement);
								fut::ok(())
							})
					);

					ctx.wait(
						client.prepare(args::VERIFY_QUERY)
							.map_err(|_| ())
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.verify_existing = Some(statement);
								fut::ok(())
							})
					);

					act.client = Some(client);
					Arbiter::spawn(conn.map_err(|e| panic!("{}", e)));
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
enum GenericError {
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


#[derive(Debug, Serialize, Deserialize)]
struct NewEmailMessage {
	email: String,
	site_name: String,
}

#[derive(Debug)]
struct NewEmailInsert {
	email: String,
	site_name: String,
	validation_token: String,
	// unsubscribe_token: String,
}


impl NewEmailMessage {
	fn into_insert(self) -> Result<NewEmailInsert, GenericError> {
		let validation_token = generate_random_token().ok_or(GenericError::InternalServer)?;

		Ok(NewEmailInsert {
			email: self.email,
			site_name: self.site_name,
			validation_token,
		})
	}

	// fn make_insert_query(&self) -> &'static str {
	// 	"insert into emails (email, site_name, validation_token) values ($1, $2, $3)"
	// }

	// fn make_insert_args(&self, validation_token: &str) -> [&str; 3] {
	// 	[&self.email, &self.site_name, validation_token]
	// }
}



impl Message for NewEmailMessage {
	type Result = Result<NewEmailInsert, GenericError>;
}

impl Handler<NewEmailMessage> for PgConnection {
	type Result = ResponseFuture<NewEmailInsert, GenericError>;

	fn handle(&mut self, msg: NewEmailMessage, _: &mut Self::Context) -> Self::Result {
		let insert_row = match msg.into_insert() {
			Ok(i) => i,
			Err(e) => {
				return Box::new(future::err(e));
			},
		};

		Box::new(
			self.client
				.as_mut().unwrap()
				// .execute(self.insert_new_email.as_ref().unwrap(), insert_row.make_insert_args().as_ref())
				.execute(self.insert_new_email.as_ref().unwrap(), &[&insert_row.email, &insert_row.site_name, &insert_row.validation_token])
				.into_future()
				.from_err()
				.and_then(move |rows| match rows {
					1 => Ok(insert_row),
					0 => Err(GenericError::NoContent),
					_ => Err(GenericError::InternalServer),
				})
		)
	}
}


// server_domain = "crowdsell.io"
// mail_private_api_key
// mail_public_key
#[derive(Debug, Serialize, Deserialize)]
struct MailgunForm {
	from: String,
	to: String,
	subject: String,
	text: String,
}


// NewEmailJsonInput


fn new_email(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = GenericError> {
	let db = req.state().db.clone();
	req.json()
		.from_err()
		.and_then(move |json_input: NewEmailJsonInput| {
			json_input.translate()
				.into_future()
				.from_err()
				.and_then(|msg| {
					db.send(msg)
						.from_err()
						.and_then(|msg_res| {
							msg_res
								.into_future()
								.from_err()
								.and_then(|msg| {
									http_client::post("http://api.mailgun.net/v3/YOUR_DOMAIN_NAME/messages")
										.form(MailgunForm {
											from: "<no-reply@crowdsell.io>".to_string(),
											to: msg.email,
											subject: "Crowdsell - Validation Email".to_string(),
											text: format!("Hello! Thank you for signing up to join the Crowdsell private beta.\n\nClick this link to validate your email:\nhttps://crowdsell.io/validate-email?t={}", msg.validation_token),
										})
										.unwrap()
										.send()
										.map_err(|e| { dbg!(e); GenericError::InternalServer })
										.and_then(|_| respond_success())
									})
						})
				})
		})
		.responder()

		// .and_then(move |msg: NewEmailMessage| {
		// 	validate_email(msg)
		// 		.and_then(move |msg| {
		// 			db.send(msg)
		// 				.from_err()
		// 				.and_then(|msg_res| {
		// 					msg_res
		// 						.into_future()
		// 						.from_err()
		// 						.and_then(|msg| {
		// 						})
		// 				})
		// 		})
		// })
}



#[derive(Debug, Serialize, Deserialize)]
struct VerifyEmailJsonInput {
	encoded_validation_token: String,
}

impl VerifyEmailJsonInput {
	fn decode(self) -> Result<VerifyEmailMessage, GenericError> {
		let validation_token = base64_decode(self.encoded_validation_token).ok_or(GenericError::BadRequest)?;
		Ok(VerifyEmailMessage { validation_token })
	}
}

#[derive(Debug)]
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
				// .execute(self.verify_existing.as_ref().unwrap(), &(insert_row.make_insert_args()))
				.execute(self.verify_existing.as_ref().unwrap(), &[&msg.validation_token])
				.into_future()
				.from_err()
				.and_then(move |rows| match rows {
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
		.and_then(move |msg: VerifyEmailJsonInput| {
			msg.decode()
				.into_future()
				.and_then(move |msg| {
					db.send(msg)
						.from_err()
						.and_then(|msg_res| {
							msg_res
								.into_future()
								.from_err()
								.and_then(|_| respond_success())
						})
					})
		})
		.responder()
}

// // fn send_mail(msg: NewEmailInsert) -> impl Future<Item = HttpResponse, Error = GenericError> {
// fn send_mail(msg: NewEmailInsert) -> impl Future<Item = actix_web::client::ClientResponse, Error = actix_web::client::SendRequestError> {
// 	// have to format "api:api_key" into url?

// }

fn validate_email(msg: NewEmailMessage) -> impl Future<Item = NewEmailMessage, Error = GenericError> {
	match checkmail::validate_email(&msg.email) {
		true => future::ok(msg),
		false => future::err(GenericError::BadRequest),
	}
}

fn main() {
	std::env::set_var("RUST_LOG", "micro_chimp=info");
	pretty_env_logger::init();

	let sys = System::new("micro_chimp");
	let db_url = "postgres://user:asdf@localhost/database";

	// start http server
	server::new(move || {
		let addr = PgConnection::connect(db_url);

		App::with_state(State { db: addr })
			.resource("/new-email", |r| r.post().a(new_email))
			.resource("/verify-email", |r| r.post().a(verify_email))
	})
		// .backlog(8192)
		.bind("127.0.0.1:5050")
		.unwrap()
		.start();

	info!("Started http server: 127.0.0.1:5050");
	let _ = sys.run();
}
