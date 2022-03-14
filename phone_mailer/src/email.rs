use dotenv;
use std::env as env;

use sendgrid::error::SendgridError;
use sendgrid::v3::*;

use crate::sms;


pub async fn send_sms_email(content:sms::SmsData) -> Result<(), SendgridError> {

    let content = f!("<p>From: {content.from}</p><pre>{content.body}</pre>");
    let result = _send_email(content).await;
    Ok(())
}

pub async fn send_raw_email(content:String) -> Result<(), SendgridError> {

    let result =_send_email(content).await;
    Ok(())
}


pub async fn _send_email(content:String) -> Result<(), SendgridError> {

    dotenv::dotenv().expect("Failed to read .env file");
    let api_key = env::var("SENDGRID_API_KEY").expect("Failed to parse Sendgrid API Key");
    let email = env::var("SENDGRID_EMAIL").expect("Failed to parse Sendgrid Email");
    let to = env::var("EMAIL_RECIPIENT").expect("Failed to parse 'to' number");

    let personalization = Personalization::new(Email::new(to));

    let template = f!("<h2>Emailed content:</h2><br/>\n<br/>\n{content}");

    let message = Message::new(Email::new(email))
        .set_subject("Phone mailer SMS email")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(content),
        )
        .add_personalization(personalization);

    let sender = Sender::new(api_key);
    let response = sender.send(&message).await?;
    println!("Email sending status: {}", response.status());

    Ok(())
}
