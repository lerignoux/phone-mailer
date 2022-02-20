use phone_mailer::api;
use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::check)
            .service(api::send_email)
            .service(api::send_sms)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
