use crate::email;
use crate::sms;
use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::Query;

#[get("/")]
pub async fn check() -> impl Responder {
    HttpResponse::Ok().body("Phone mailer running :)")
}

#[post("/email")]
pub async fn send_email(sms_data: Query<sms::SmsData>) -> impl Responder {

    let response = match email::send_sms_email(sms_data.into_inner()).await {
      Ok(result) => println!("Email sent {:?}", result),
      Err(error) => panic!("Could not send email, {:?}", error),
    };

    HttpResponse::Ok().body(response)
}


#[post("/sms")]
pub async fn send_sms(req_body: String) -> impl Responder {

    let response = sms::send_sms(req_body).await;

    HttpResponse::Ok().body(response)
}
