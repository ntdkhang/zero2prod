use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde::Deserialize;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
    App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(form: web::Form<SubscriptionData>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct SubscriptionData {
    name: String,
    email: String,
}
