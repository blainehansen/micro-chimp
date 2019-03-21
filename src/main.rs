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
use futures::{Future, future};

mod utils;
use utils::generate_random_token;

use serde::{Deserialize, Serialize};

// use tokio_postgres::{Client, Statement, types::{ToSql as TokioToSql, FromSql as TokioFromSql}};
use tokio_postgres::{Client, Statement};
// use postgres::{types::{ToSql, FromSql, IsNull, Type}};


struct PgConnection {
	client: Option<Client>,
	insert_new_email: Option<Statement>,
}

impl Actor for PgConnection {
	type Context = Context<Self>;
}

impl PgConnection {
	pub fn connect(db_url: &str) -> Addr<PgConnection> {
		let hs = tokio_postgres::connect(db_url, tokio_postgres::tls::NoTls);

		PgConnection::create(move |ctx| {
			let act = PgConnection {
				client: None,
				insert_new_email: None,
			};

			hs.map_err(|_| panic!("can not connect to postgresql"))
				.into_actor(&act)
				.and_then(|(mut client, conn), act, ctx| {
					ctx.wait(
						client.prepare("insert into emails (email, site_name, validation_token) values ($1, $2, $3)")
							.map_err(|_| ())
							.into_actor(act)
							.and_then(|statement, act, _| {
								act.insert_new_email = Some(statement);
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
			GenericError::NoContent
		}
		else if c == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
			GenericError::BadRequest
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


// #[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
// #[postgres(name = "site_name_type")]
// enum SiteName {
// 	#[postgres(name = "crowdsell")]
// 	Crowdsell,
// 	#[postgres(name = "blog")]
// 	Blog,
// }


// impl TokioToSql for SiteName {
// 	fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
// 		self.to_sql(ty, out)
// 	}

// 	fn accepts(ty: &Type) -> bool {
// 		SiteName::accepts(ty)
// 	}

// 	fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
// 		self.to_sql_checked(ty, out)
// 	}
// }

// impl<'a> TokioFromSql<'a> for SiteName {
// 	fn from_sql(ty: &Type,  raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
// 		SiteName::from_sql(ty, raw)
// 	}

// 	fn accepts(ty: &Type) -> bool {
// 		SiteName::accepts(ty)
// 	}
// }


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
	// fn make_insert_query() -> &'static str {
	// 	"insert into emails (email, validation_token) values ($1, $2)"
	// }

	fn into_insert(self) -> Result<NewEmailInsert, GenericError> {
		let validation_token = generate_random_token().ok_or(GenericError::InternalServer)?;

		Ok(NewEmailInsert {
			email: self.email,
			site_name: self.site_name,
			validation_token,
		})
	}

	// if site_names are enabled
	// fn make_insert_query(&self) -> &'static str {
	// 	"insert into emails (email, site_name, validation_token) values ($1, $2, $3)"
	// }

	// fn make_insert_args(&self, validation_token: &str) -> [&str; 2] {
	// 	[&self.email, validation_token]
	// }

	// if site_names are enabled
	// fn make_insert_args(&self, validation_token: &str) -> [&str; 3] {
	// 	[&self.email, &self.site_name, validation_token]
	// }
}


// #[derive(Debug, Serialize, Deserialize)]
// struct ValidationToken {
// 	token: String,
// }


impl Message for NewEmailMessage {
	type Result = Result<NewEmailInsert, GenericError>;
}

impl Handler<NewEmailMessage> for PgConnection {
	type Result = ResponseFuture<NewEmailInsert, GenericError>;

	fn handle(&mut self, msg: NewEmailMessage, _: &mut Self::Context) -> Self::Result {
		let insert_row = match dbg!(msg).into_insert() {
			Ok(i) => i,
			Err(e) => {
				return Box::new(future::err(e));
			},
		};

		Box::new(
			self.client
				.as_mut().unwrap()
				// .execute(self.insert_new_email.as_ref().unwrap(), &(insert_row.make_insert_args()))
				.execute(self.insert_new_email.as_ref().unwrap(), &[&insert_row.email, &insert_row.site_name, &insert_row.validation_token])
				.map_err(|e| dbg!(e))
				.from_err()
				.and_then(move |rows| {
					println!("rows: {:?}", rows);
					if rows <= 0 {
						return future::err(GenericError::NoContent);
					}
					if rows >= 2 {
						return future::err(GenericError::InternalServer);
					}
					future::ok(insert_row)
				})
		)
	}
}

struct State {
	db: Addr<PgConnection>,
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


// struct MailgunForm<'f> {
// 	from: &'static str,
// 	to: &'f str,
// 	subject: &'static str,
// 	text: &'f str,
// }

fn new_email(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = GenericError> {
	let db = req.state().db.clone();
	req.json()
		.map_err(|e| match e {
			actix_web::error::JsonPayloadError::Deserialize(_) => GenericError::Unprocessable,
			_ => GenericError::BadRequest,
		})
		.and_then(move |msg: NewEmailMessage| {
			validate_email(msg)
				.and_then(move |msg| {
					db.send(msg)
						.from_err()
						.and_then(|msgr| msgr.and_then(send_mail))
				})
		})
		.responder()
}


fn send_mail(msg: NewEmailInsert) -> impl Future<Item = HttpResponse, Error = GenericError> {
	// have to format "api:api_key" into url?
	let request: actix_web::client::ClientRequest = http_client::post("https://api.mailgun.net/v3/YOUR_DOMAIN_NAME/messages")
		.form(MailgunForm {
			from: "<no-reply@crowdsell.io>".to_string(),
			to: msg.email,
			subject: "Crowdsell - Validation Email".to_string(),
			text: format!("Hello! Thank you for signing up to join the Crowdsell private beta.\n\nClick this link to validate your email:\nhttps://crowdsell.io/validate-email?t={}", msg.validation_token),
		})
		.unwrap();

	request
		.send()
		// .map_err(|e| { dbg!(e); GenericError::InternalServer })
		.and_then(|r| {
			dbg!(r);
			future::ok(empty_status(StatusCode::NO_CONTENT))
		})
}

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
	})
		// .backlog(8192)
		.bind("127.0.0.1:5050")
		.unwrap()
		.start();

	info!("Started http server: 127.0.0.1:5050");
	let _ = sys.run();
}


// algorithms:

// new email:
// parse incoming email and system/site
// validate email format
// generate a validation_token
// insert into emails (email, validation_token) values ($1, $2)
// handle constraint error
// create validation url
// create validation email body with that url in it
// send to mailgun

// validate:
// parse input validation_token
// update emails set validation_token = null where validation_token = $1
// check to see if correct number of rows were changed
// do errors and stuff
