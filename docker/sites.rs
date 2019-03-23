use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum SiteName {
	CrowdsellIo
}

use super::MailgunForm;

impl SiteName {
	pub fn get_site_information(self, to: String) -> (String, String, MailgunForm) {
		use SiteName::*;
		match self {
			CrowdsellIo => (
				"https://api.mailgun.net/v3/crowdsell.io/messages".to_string(),
				"CROWDSELL_IO".to_string(),
				MailgunForm {
					to,
					from: "<no-reply@crowdsell.io>".to_string(),
					subject: "Crowdsell - Validation Email".to_string(),
					text: "Hello! Thank you for signing up to join the Crowdsell private beta.\n\nClick this link to validate your email:\nhttps://crowdsell.io/validate-email?t={verification_token}\n".to_string(),
				},
			),
		}
	}
}
