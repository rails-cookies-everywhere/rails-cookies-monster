use base64::prelude::*;
use rails_cookie_parser::HashDigest;
use rails_cookie_parser::ParseCookieError;
use rails_cookie_parser::RailsCookieParser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RailsMessage {
  #[serde(rename = "message")]
  pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RailsCookie {
  #[serde(rename = "_rails")]
  pub rails: RailsMessage,
}

pub fn decipher_cookie(rails_version: &str, cookie: &str) -> Result<String, ParseCookieError> {
  let cookie_parser = match rails_version {
    "7.0.0" => RailsCookieParser::new(
      &std::env::var("SECRET_KEY_BASE").unwrap(),
      "authenticated encrypted cookie",
      1000,
      HashDigest::Sha256,
    ),
    _ => RailsCookieParser::default(),
  };

  let decoded = cookie_parser.decipher_cookie(cookie).unwrap();
  let rails_message = decoded
    .split('"')
    .nth(5)
    .expect("No rails message")
    .to_string();
  // Be safer, bro!
  //
  // let rails_cookie: RailsCookie = serde_json::from_str(&decoded).expect("Incorrect JSON");
  // let rails_message = rails_cookie.rails.message
  let b64 = BASE64_STANDARD.decode(rails_message).expect("Wrong base64");

  Ok(String::from_utf8(b64).expect("Wrong UTF8"))
}
