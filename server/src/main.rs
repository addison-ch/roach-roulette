#![allow(non_snake_case, non_camel_case_types, dead_code)]

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running");

    HttpServer::new(|| App::new().service(hello))
        .bind(("localhost", 8080))?
        .run()
        .await
}
