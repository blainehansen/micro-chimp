// https://gmosx.ninja/posts/2020/09/21/how-to-deploy-a-rust-service-to-google-cloud-run
// https://cloud.google.com/run/docs/quickstarts/build-and-deploy/other
// http://opreview.blogspot.com/2017/03/how-to-upload-to-google-cloud-storage.html

use actix_web::{web, http::{self, StatusCode}, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

fn from_base64<'d, D>(deserializer: D) -> Result<String, D::Error>
	where D: serde::Deserializer<'d>
{
	use serde::de::Error;
	let de = String::deserialize(deserializer)?;
	let buf = base64::decode_config(&de, base64::URL_SAFE).map_err(|_| Error::custom("unable to decode as base64"))?;
	String::from_utf8(buf).map_err(|_| Error::custom(""))
}

fn base64_encode(s: &[u8]) -> String {
	base64::encode_config(s, base64::URL_SAFE)
}

// use rand::{RngCore, OsRng};
// fn generate_random_token() -> Option<String> {
// 	let mut r = OsRng::new().ok()?;
// 	let mut buf: [u8; 64] = [0; 64];
// 	r.fill(&mut buf);

// 	Some(base64_encode(&buf[..]))
// }


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
// 	// Unprocessable,
// 	InternalServer,
// }

// impl actix_web::ResponseError for GenericError {
// 	fn error_response(&self) -> HttpResponse {
// 		match *self {
// 			GenericError::NoContent => empty_status(StatusCode::NO_CONTENT),
// 			GenericError::BadRequest => empty_status(StatusCode::BAD_REQUEST),
// 			// GenericError::Unprocessable => empty_status(StatusCode::UNPROCESSABLE_ENTITY),
// 			GenericError::InternalServer => empty_status(StatusCode::INTERNAL_SERVER_ERROR),
// 		}
// 	}
// }

// impl From<actix_web::MailboxError> for GenericError {
// 	fn from(_: actix_web::MailboxError) -> Self {
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


// const NEW_EMAIL_QUERY: &'static str = "insert into subscription (email, validation_token) values ($1, $2::site_name_enum, $3)";
// to signup a new person, we create a new cloud storage (or google drive) file with their base64 encoded email as the filename. the file is a bincoded struct representing their current status

// const VERIFY_QUERY: &'static str = "update subscription set validation_token = null where validation_token = $1";
// when a new person signs up, we need to create a secure random token they can use to verify their email. the verify route receives this token along with

// const UNSUBSCRIBE_QUERY: &'static str = "update subscription set unsubscribed_with = $1 where email = $2"


#[derive(Debug, Serialize)]
pub struct MailgunForm {
	to: String,
	text: String,
	from: &'static str,
	subject: &'static str,
}


// #[derive(Debug)]
// struct NewEmailMessage {
// 	validation_token: String,
// 	mailgun_url: &'static str,
// 	mailgun_form: MailgunForm,
// }

#[derive(Debug, Deserialize, validator::Validate)]
struct SubscribePayload {
	#[validate(email)]
	email: String,
}

const TEXT_BODY: &'static str = "Yo here's an email {}.";

#[actix_web::post("/subscribe")]
async fn subscribe(payload: web::Json<SubscribePayload>, data: web::Data<AppState>) -> impl Responder {
	// send a verification email
	let response = actix_web::client::Client::default()
		.post("unknown")
		.header(http::header::AUTHORIZATION, data.mailgun_header)
		.send_form(&MailgunForm {
			to: payload.email,
			text: format!(TEXT_BODY, validation_url),
			from: "no-reply@example.com",
			subject: "verify",
		})
		.await?;

	// store the status struct

	// type Status = Vec<u8>;
	// let target: Status = vec![1, 2, 3];
	// let encoded = bincode::serialize(&target).unwrap();
	// let decoded: Status = bincode::deserialize(&dbg!(encoded)[..]).unwrap();
	// assert_eq!(target, decoded);

	Ok("yo")
}


#[derive(Debug, Deserialize)]
struct VerifyPayload {
	validation_token: String,
}

#[actix_web::post("/verify")]
async fn verify(payload: web::Json<VerifyPayload>) -> impl Responder {
	"yo"
}


#[derive(Debug, Deserialize)]
struct UnsubscribePayload {
	#[serde(deserialize_with = "from_base64")]
	email: String,
	unsubscribed_with: String,
}

#[actix_web::post("/unsubscribe")]
async fn unsubscribe(payload: web::Json<UnsubscribePayload>) -> impl Responder {
	"yo"
}


fn std_err(message: String) -> std::io::Error {
	std::io::Error::new(std::io::ErrorKind::Other, message)
}

fn get_env(env_var: &'static str) -> std::io::Result<String> {
	std::env::var(env_var)
		.map_err(|_| std_err(format!("{} isn't set", env_var)))
		.map(|v| v.trim().to_string())
}

struct AppState {
	mailgun_header: http::HeaderValue
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mailgun_header = http::HeaderValue::from_bytes(
		format!(
			"Basic {}",
			base64_encode(get_env("MAILGUN_API_KEY")?.trim().as_bytes())
		).as_bytes()
	).map_err(|_| std_err("unable to construct header value".into()))?;


	// std::env::set_var("RUST_LOG", "micro_chimp=info");
	// pretty_env_logger::init();

	actix_web::HttpServer::new(move || actix_web::App::new()
		.data(AppState { mailgun_header: mailgun_header.clone() })
		.service(subscribe)
		.service(verify)
		.service(unsubscribe)
	)
		.bind("0.0.0.0:5050")?
		.run()
		.await

	// info!("Started http server: 0.0.0.0:5050");
}
