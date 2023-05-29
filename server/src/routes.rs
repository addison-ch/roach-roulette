use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// planning to separate endpoints into this file, currently doesn't do anything
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

