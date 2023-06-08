#![allow(non_snake_case, non_camel_case_types, dead_code)]

use actix_web::dev::Server;
use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

mod utils;
use utils::generate_room_code;

// Struct that keeps track of application state, to keep things organized
struct AppState {
    active_rooms: Arc<Mutex<HashSet<String>>>,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

// Endpoint to be called to check on the status of the API
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// web::Data is the shared application state, of type AppState struct
// the impl Responder return type is a type that represents stuff like HttpResponse, or String etc.
async fn create_room(state: web::Data<AppState>) -> impl Responder {
    // locks the data so that only one thread can access it at a time, unwrap is used to handle errors, it assumes the lock is successful
    let mut active_rooms = state.active_rooms.lock().unwrap();

    let mut room_code = generate_room_code();
    while active_rooms.contains(&room_code) {
        room_code = generate_room_code();
    }

    active_rooms.insert(room_code.clone());

    HttpResponse::Ok().json(room_code)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let active_rooms = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let app_state = web::Data::new(AppState { active_rooms });

    let port = listener.local_addr().unwrap().port();
    println!("Server running on http://127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        let app_state = app_state.clone();
        App::new()
            .route("/", web::get().to(hello))
            .route("/health_check", web::get().to(health_check))
            .app_data(app_state)
            .route("create_room", web::get().to(create_room))
    })
    .listen(listener)?
    .run();

    Ok(server)
}