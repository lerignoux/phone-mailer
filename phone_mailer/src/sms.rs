use dotenv;
use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env as env;
use serde::Deserialize;
use serde;

mod sms_test;

pub struct SmsDataError;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmsData {
    pub body: String,
    pub from: String,
    pub to: String,
    pub sms_status: String,
    pub sms_sid: String,
    pub sms_message_sid: String,
    pub message_sid: String,
    pub account_sid: String,
    num_segments: u32
}

pub async fn send_sms(content:String) {
  // Securely import sensitive credentials and values from `.env` instead of inlining their values.
  dotenv::dotenv().expect("Failed to read .env file");
  let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Failed to parse Account SID");
  let api_key = env::var("TWILIO_API_KEY").expect("Failed to parse API Key");
  let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Failed to parse API Key Secret");
  let from = env::var("TWILIO_PHONE_NUMBER").expect("Failed to parse 'from' number");
  let to = env::var("SMS_RECIPIENT").expect("Failed to parse 'to' number");

  // Create a new, default client configuration.
  let mut twilio_config = Configuration::default();
  // Apply your Basic Auth credentials to the client configuration.
  twilio_config.basic_auth = Some((api_key, Some(api_key_secret)));

  let message = twilio_api::create_message(
    &twilio_config,
    &account_sid,
    &to,
    None,
    None,
    None,
    Some(&content),
    None,
    None,
    Some(&from),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None, None, None
  )
  .await;

  let _result = match message {
    Ok(result) => println!("Sms sent {:?}", result),
    Err(error) => panic!("Could not send sms, {:?}", error),
  };
  
}
