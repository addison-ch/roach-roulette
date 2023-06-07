#![allow(non_snake_case, non_camel_case_types, dead_code)]

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

// Endpoint to be called to check on the status of the API
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
