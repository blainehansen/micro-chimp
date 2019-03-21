use rand::Rng;
use rand::rngs::OsRng;

pub fn base64_encode(buf: &[u8]) -> String {
	base64::encode_config(buf, base64::URL_SAFE)
}

pub fn generate_random_token() -> Option<String> {
	let mut r = OsRng::new().ok()?;
	let mut buf: [u8; 64] = [0; 64];
	r.fill(&mut buf);

	Some(base64_encode(&buf[..]))
}

