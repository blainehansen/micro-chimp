// use serde::Deserialize;
// // use tokio_postgres::types::ToSql;

// // #[derive(Debug, ToSql, Deserialize)]
// #[derive(Debug, Deserialize)]
// // #[postgres(name = "site_name_enum")]
// pub enum SiteName {
// 	// #[postgres(name = "crowdsell")]
// 	CROWDSELL,
// 	// #[postgres(name = "blog")]
// 	BLOG,
// }

// use super::MailgunForm;

// impl SiteName {
// 	pub fn get_site_information(self, to: String) -> (String, String, MailgunForm) {
// 		use SiteName::*;
// 		match self {
// 			CROWDSELL => (
// 				"https://api.mailgun.net/v3/crowdsell.io/messages".to_string(),
// 				"crowdsell".to_string(),
// 				MailgunForm {
// 					from: "<no-reply@crowdsell.io>".to_string(),
// 					to,
// 					subject: "Crowdsell - Validation Email".to_string(),
// 					text: "Hello! Thank you for signing up to join the Crowdsell private beta.\n\nClick this link to validate your email:\nhttps://crowdsell.io/validate-email?t={verification_token}".to_string(),
// 				},
// 			),
// 			BLOG => ("https://api.mailgun.net/v3/blainehansen.co/messages".to_string(), "blog".to_string(), MailgunForm {
// 				from: "<no-reply@blainehansen.co>".to_string(),
// 				to,
// 				subject: "blainehansen.co - verification email".to_string(),
// 				text: "Hello! Click this link to validate your email:\nhttps://blainehansen.co/validate-email?t={verification_token}".to_string(),
// 			}),
// 		}
// 	}
// }

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
