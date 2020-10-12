// https://actix.rs/docs/http2/
use actix_web::{get, web::{self, Path}, Responder};
type Pool = web::Data<sqlx::PgPool>;


fn get_env(env_var: &'static str) -> std::io::Result<String> {
	std::env::var(env_var)
		.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, format!("{} isn't set", env_var)))
		.map(|v| v.trim().to_string())
}

// macro_rules! query_one {
// 	($query: expr, $($args: expr),+) => {
// 		//
// 	};
// }

#[get("/{id}/{name}")]
async fn index(
	Path((id, name)): Path<(i64, String)>,
	pool: Pool,
) -> actix_web::Result<String> {
// ) -> impl Responder {
	let row = sqlx::query!(
			r#"select $1::bigint as "yo!", $2::text as "dude!""#,
			id, name,
		)
		.fetch_one(&**pool)
		.await
		.map_err(|_| actix_web::error::ErrorInternalServerError("database"))?;

	Ok(format!("id: {}; Hello {}! ", row.yo, row.dude))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&get_env("DATABASE_URL")?).await
		.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

	actix_web::HttpServer::new(move || {
		actix_web::App::new().data(pool.clone())
			.service(index)
	})
		.bind("127.0.0.1:8080")?
		.run().await
}




// extern crate base64;

// extern crate rand;
// use rand::Rng;
// use rand::rngs::OsRng;

// mod sites;
// use sites::SiteName;


// fn base64_encode(s: &[u8]) -> String {
// 	base64::encode_config(s, base64::URL_SAFE)
// }

// pub fn generate_random_token() -> Option<String> {
// 	let mut r = OsRng::new().ok()?;
// 	let mut buf: [u8; 64] = [0; 64];
// 	r.fill(&mut buf);

// 	Some(base64_encode(&buf[..]))
// }


// const NEW_EMAIL_QUERY: &'static str = "insert into subscription (email, site_name, validation_token) values ($1, $2::site_name_enum, $3)";
// const VERIFY_QUERY: &'static str = "update subscription set validation_token = null where validation_token = $1";
// const UNSUBSCRIBE_QUERY: &'static str = "update subscription set unsubscribed_with = $1 where email = $2 and site_name = $3::site_name_enum"


// fn empty_status(code: StatusCode) -> HttpResponse {
// 	HttpResponse::with_body(code, actix_web::Body::Empty)
// }

// fn respond_success() -> Result<HttpResponse, GenericError> {
// 	Ok(empty_status(StatusCode::NO_CONTENT))
// }


// #[derive(Debug, Error)]
// pub enum GenericError {
// 	NoContent,
// 	BadRequest,
// 	Unprocessable,
// 	InternalServer,
// }

// impl ResponseError for GenericError {
// 	fn error_response(&self) -> HttpResponse {
// 		match *self {
// 			GenericError::NoContent => empty_status(StatusCode::NO_CONTENT),
// 			GenericError::BadRequest => empty_status(StatusCode::BAD_REQUEST),
// 			GenericError::Unprocessable => empty_status(StatusCode::UNPROCESSABLE_ENTITY),
// 			GenericError::InternalServer => empty_status(StatusCode::INTERNAL_SERVER_ERROR),
// 		}
// 	}
// }


// impl From<tokio_postgres::Error> for GenericError {
// 	fn from(error: tokio_postgres::Error) -> Self {
// 		let c = error.code();
// 		if c == Some(&tokio_postgres::error::SqlState::INTEGRITY_CONSTRAINT_VIOLATION) {
// 			GenericError::BadRequest
// 		}
// 		else if c == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
// 			GenericError::NoContent
// 		}
// 		else {
// 			GenericError::InternalServer
// 		}
// 	}
// }

// impl From<actix::MailboxError> for GenericError {
// 	fn from(_: actix::MailboxError) -> Self {
// 		GenericError::InternalServer
// 	}
// }

// impl From<actix_web::error::JsonPayloadError> for GenericError {
// 	fn from(error: actix_web::error::JsonPayloadError) -> Self {
// 		match dbg!(error) {
// 			actix_web::error::JsonPayloadError::Deserialize(_) => GenericError::Unprocessable,
// 			_ => GenericError::BadRequest,
// 		}
// 	}
// }




// #[derive(Debug, Serialize)]
// pub struct MailgunForm {
// 	to: String,
// 	text: String,
// 	from: &'static str,
// 	subject: &'static str,
// }


// #[derive(Debug, Validate, Deserialize)]
// struct NewEmailJsonInput {
// 	#[validate(email)]
// 	email: String,
// 	site_name: SiteName,
// }

// #[derive(Debug)]
// struct NewEmailMessage {
// 	site_name: &'static str,
// 	validation_token: String,
// 	mailgun_url: &'static str,
// 	mailgun_form: MailgunForm,
// }


// fn from_base64<'d, D>(deserializer: D) -> Result<String, D::Error>
// 	where D: Deserializer<'d>
// {
// 	use serde::de::Error;
// 	let de = String::deserialize(deserializer)?;
// 	let buf = base64::decode_config(&de, base64::URL_SAFE).map_err(|_| Error::custom(""))?;
// 	String::from_utf8(buf).map_err(|_| Error::custom(""))
// }

// #[derive(Debug, Deserialize)]
// struct VerifyEmailMessage {
// 	validation_token: String,
// }


// http_client::post(msg.mailgun_url)
// 	.header(actix_web::http::header::AUTHORIZATION, MAILGUN_API_KEY.to_owned())
// 	.form(msg.mailgun_form)


// #[derive(Debug, Deserialize)]
// struct UnsubscribeMessage {
// 	#[serde(deserialize_with = "from_base64")]
// 	email: String,
// 	site_name: SiteName,
// 	unsubscribed_with: String,
// }


// lazy_static! {
// 	static ref MAILGUN_API_KEY: HeaderValue = {
// 		let contents = get_env("MAILGUN_API_KEY");
// 		let auth = base64_encode(contents.trim().as_bytes());
// 		HeaderValue::from_bytes(format!("Basic {}", auth).as_bytes()).expect("couldn't construct valid header")
// 	};
// }


// fn main() {
// 	std::thread::sleep(std::time::Duration::from_secs(5));

// 	assert!(MAILGUN_API_KEY.to_owned() != "");

// 	std::env::set_var("RUST_LOG", "micro_chimp=info");
// 	pretty_env_logger::init();

// 	let user = "rust_server_user";
// 	let pass = get_env("SERVER_POSTGRES_PASSWORD");

// 	let db_url = format!("postgres://{}:{}@database/database", user, pass);

// 	// start http server
// 	let sys = System::new("micro_chimp");
// 	server::new(move || {
// 		let addr = PgConnection::connect(db_url.as_str());

// 		App::with_state(State { db: addr })
// 			.resource("/subscribe", |r| r.post().a(new_email))
// 			.resource("/verify", |r| r.post().a(verify_email))
// 			.resource("/unsubscribe", |r| r.post().a(unsubscribe))
// 	})
// 		// .backlog(8192)
// 		.bind("0.0.0.0:5050")
// 		.unwrap()
// 		.start();

// 	info!("Started http server: 0.0.0.0:5050");
// 	let _ = sys.run();
// }
