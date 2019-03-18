extern crate pretty_env_logger;
#[macro_use] extern crate log;

extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
extern crate serde_json;
// extern crate checkmail;

extern crate tokio_postgres;

// extern crate rand;
// extern crate base64;

use actix::prelude::*;
use actix_web::{
	http::StatusCode, server, App, AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json
};
use futures::Future;
// use rand::{thread_rng, Rng, ThreadRng};

// mod utils;
// use utils::{generate_random_token, NewEmail, ValidationToken};

use serde::{Deserialize, Serialize};


use tokio_postgres::{Client, Statement};

pub struct PgConnection {
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
						client.prepare("insert into emails (email, validation_token) values ($1, $2)")
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
				}).wait(ctx);

			act
		})
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct NewEmail {
	email: String,
	service: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct ValidationToken {
// 	token: String,
// }

// tokio_postgres::error::SqlState::INTEGRITY_CONSTRAINT_VIOLATION

impl Message for NewEmail {
	type Result = Result<(), tokio_postgres::error::Error>;
}

impl Handler<NewEmail> for PgConnection {
	type Result = ResponseFuture<(), tokio_postgres::error::Error>;

	fn handle(&mut self, n: NewEmail, _: &mut Self::Context) -> Self::Result {
		Box::new(
			self.client
				.as_mut().unwrap()
				.execute(self.insert_new_email.as_ref().unwrap(), &[&n.email, &n.service])
				.and_then(|rows| {
					println!("{:?}", rows);
					Ok(())
				})
		)
	}
}

struct State {
	db: Addr<PgConnection>,
}

fn new_email(req: &HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = actix_web::error::Error> {
	let db = req.state().db.clone();
	req.json()
		.from_err()
		.and_then(move |v: NewEmail| {
			db.send(v)
				.then(|res| match res {
					Ok(_) => Ok(HttpResponse::with_body(StatusCode::NO_CONTENT, actix_web::Body::Empty)),
					Err(_) => Ok(HttpResponse::with_body(StatusCode::INTERNAL_SERVER_ERROR, actix_web::Body::Empty)),
				})
		}).responder()
}


fn main() {
	pretty_env_logger::init();

	let sys = System::new("micro_email_collector");
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
