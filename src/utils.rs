use rand::Rng;
use rand::rngs::OsRng;

pub fn base64_encode(buf: &[u8]) -> String {
	base64::encode_config(buf, base64::URL_SAFE)
}

pub fn base64_decode(string: String) -> Option<String> {
	let buf = base64::decode_config(string.as_bytes(), base64::URL_SAFE).ok()?;
	String::from_utf8(buf).ok()
}

pub fn generate_random_token() -> Option<String> {
	let mut r = OsRng::new().ok()?;
	let mut buf: [u8; 64] = [0; 64];
	r.fill(&mut buf);

	Some(base64_encode(&buf[..]))
}



#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate serde;
extern crate serde_yaml;
extern crate sodiumoxide;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
  key: PublicKey,
}

fn as_base64<T, S>(key: &T, serializer: &mut S) -> Result<(), S::Error>
	where T: AsRef<[u8]>, S: Serializer
{
	serializer.serialize_str(&base64::encode(key.as_ref()))
}

fn from_base64<D>(deserializer: &mut D) -> Result<PublicKey, D::Error>
    where D: Deserializer
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| PublicKey::from_slice(&bytes))
        .and_then(|opt| opt.ok_or_else(|| Error::custom("failed to deserialize public key")))
}

fn main() {
    let config = Config {
        key: PublicKey::from_slice(&[1; PUBLICKEYBYTES]).unwrap(),
    };
    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("{}", yaml);
    let _: Config = serde_yaml::from_str(&yaml).unwrap();
}
