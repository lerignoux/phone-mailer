use dotenv;
use std::env as env;

use sendgrid::error::SendgridError;
use sendgrid::v3::*;


pub async fn send_email(content:String) -> Result<(), SendgridError> {

    dotenv::dotenv().expect("Failed to read .env file");
    let api_key = env::var("SENDGRID_API_KEY").expect("Failed to parse Sendgrid API Key");
    let email = env::var("SENDGRID_EMAIL").expect("Failed to parse Sendgrid Email");
    let to = env::var("SMS_RECIPIENT").expect("Failed to parse 'to' number");

    let personalization = Personalization::new(Email::new(to));

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
