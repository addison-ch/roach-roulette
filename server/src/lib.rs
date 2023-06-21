#![allow(non_snake_case, non_camel_case_types, dead_code)]

mod ws;
mod utils;
mod lobby;
use lobby::Lobby;
use actix_web::dev::Server;
mod messages;
mod start_connection;
use start_connection::start_connection as start_connection_route;
use actix::Actor;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use utils::generate_room_code;

// Struct that keeps track of application state, to keep things organized
struct AppState {
    active_rooms: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDBs")
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
    while active_rooms.contains_key(&room_code) {
        room_code = generate_room_code();
    }

    active_rooms.insert(room_code.clone(), Vec::new());
    for (room, players) in active_rooms.iter() {
        println!("{}", room);
        println!("{:?}", players)
    }

    HttpResponse::Ok().json(room_code)
}

// async fn actors(){
//     let instance = Lobby {
//         sessions: HashMap::new(),
//         rooms: HashMap::new(),
//     };
    
//     let roach_server = instance.start();
//     let result: Result<_, _> = roach_server.send(Connect).await;

//     match result {
//         Ok(res) => println!("Got result: {}", res.unwrap()),
//         Err(err) => println!("Got error: {}", err),
//     }
// }

// #[actix_rt::main]
pub fn run() -> Result<Server, std::io::Error> {
    let active_rooms = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let app_state = web::Data::new(AppState { active_rooms });

    // actors();
    // start up the lobby
    let instance = Lobby {
        sessions: HashMap::new(),
        rooms: HashMap::new(),
    };
    
    let roach_server = instance.start();
    println!("order");
    // let anotherThing = roach_server.clone();
    // let port = _listener.local_addr().unwrap().port();
    let server = HttpServer::new(move || {
        // let app_state = app_state.clone();
        App::new()
            .route("/", web::get().to(hello))
            .route("/health_check", web::get().to(health_check))
            // .app_data(app_state)
            .route("/create_room", web::get().to(create_room))
            .app_data(roach_server.clone())
            .service(start_connection_route)
    })
    .bind("127.0.0.1:3005")?
    .run();

    println!("Server running on http://127.0.0.1:3005");
    // let anotherValue = anotherThing.clone();
    Ok(server)
}
