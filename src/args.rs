pub const NEW_EMAIL_QUERY = "insert into emails (email, validation_token, site_name) values ($1, $2, $3)";
pub const VERIFY_QUERY = "update emails set validation_token = null where validation_token = $1";

// pub const MAILGUN_URL = "https://api.mailgun.net/v3/crowdsell.io/messages";

#[derive(Debug, Serialize, Deserialize)]
pub enum SiteName {
	crowdsell,
	blog,
}

impl SiteName {
	fn into_mailgun(self, to: String) -> (String, MailgunForm) {
		use SiteName::*;
		match self {
			crowdsell => ("https://api.mailgun.net/v3/crowdsell.io/messages", MailgunForm {
				from: "<no-reply@crowdsell.io>",
				to,
				subject: "Crowdsell - Validation Email",
				text: "Hello! Thank you for signing up to join the Crowdsell private beta.\n\nClick this link to validate your email:\nhttps://crowdsell.io/validate-email?t={verification_token}",
			}),
			blog => ("https://api.mailgun.net/v3/blainehansen.co/messages", MailgunForm {
				from: "<no-reply@blainehansen.co>",
				to,
				subject: "blainehansen.co - verification email",
				text: "Hello! Click this link to validate your email:\nhttps://blainehansen.co/validate-email?t={verification_token}",
			}),
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
struct MailgunForm {
	from: String,
	to: String,
	subject: String,
	text: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewEmailMessage {
	email: String,
	site_name: SiteName,
}

impl NewEmailMessage {
	fn into_insert(self) -> Result<NewEmailInsert, GenericError> {
		let validation_token = generate_random_token().ok_or(GenericError::InternalServer)?;

		Ok(NewEmailInsert {
			email: self.email,
			site_name: ,
			validation_token,
		})
	}
}

#[derive(Debug)]
struct NewEmailInsert {
	email: String,
	site_name: u8,
	validation_token: String,
	// unsubscribe_token: String,
}

impl NewEmailInsert {
	fn make_insert_args(arg: Type) -> RetType {
		unimplemented!();
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailInput {
	encoded_validation_token: String,
}


// need to have static args for port

// some function or something for getting the
