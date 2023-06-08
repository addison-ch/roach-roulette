#![allow(non_snake_case,non_camel_case_types,dead_code)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashSet;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};


mod routes;
// The following fuctions are used to provide the basic nescessities of the game with the main function at the bottom.


// Struct that keeps track of application state, to keep things organized
struct AppState {
    active_rooms: Arc<Mutex<HashSet<String>>>,
}


fn shuffle(deck: &[u8; 64]) -> [Vec<u8>; 4] {
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();
    let mut deck3 = Vec::new();
    let mut deck4 = Vec::new();

    for a in 0..16 {
        deck4.push(deck[(a * 2) + 3]);
        deck3.push(deck[(a * 2) + 2]);
        deck2.push(deck[(a * 2) + 1]);
        deck1.push(deck[a * 2]);
    }
    println!("Deck1 : {:?}    Deck2 : {:?}    Deck3 : {:?}      Deck4 : {:?}", deck1,deck2,deck3,deck4);
    [deck1, deck2, deck3, deck4]

}


fn deal(shuf: &[u8; 64]) -> [u8; 64]
{
    println!("Running-----------------------------------------");

    let mut value = shuf.clone();
    let both = shuffle(shuf, );
    let deck1 = & both[0];
    let deck2 = & both[1];
    let deck3 = & both[2];
    let deck4 = & both[3];

    *shuf
}

fn generate_room_code() -> String {
    let mut rng = thread_rng();
    let code: String = rng
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    code
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

// web::Data is the shared application state, of type AppState struct
// the impl Responder return type is a type that represents stuff like HttpResponse, or String etc.
#[get("/create-room")]
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let active_rooms = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let app_state = web::Data::new(AppState { active_rooms});

    println!("{}", generate_room_code());
    println!("Server running");

    HttpServer::new(move ||{ let app_state = app_state.clone();
        App::new().service(hello).app_data(app_state).service(create_room)})
        .bind(("localhost", 8080))?
        .run()
        .await

}