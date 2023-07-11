use crate::lobby::Lobby;
use crate::ws::WsConn;
use crate::AppState;
use actix::Addr;
use actix_web::Responder;
use actix_web::{web, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    info: Path<(String,)>,
    srv: Data<Addr<Lobby>>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let room_code_by_id = state.room_code_by_id.lock().unwrap();
    let room_code_client_facing = info.into_inner().0;
    let room_uuid_lookup = room_code_by_id.get(&room_code_client_facing);

    // might fail
    let room_uuid = match room_uuid_lookup {
        Some(uuid) => {
            println!(
                "/start_connection route reached, room id {}, room code {}",
                uuid, room_code_client_facing
            );
            *uuid
        }
        None => {
            println!("Room not found");
            return Ok(HttpResponse::InternalServerError().body("Room not found"));
        }
    };

    let ws = WsConn::new(room_uuid, srv.get_ref().clone());

    let resp: HttpResponse = ws::start(ws, &req, stream)?;
    Ok(resp)
}

pub async fn supersimple(_info: Path<(String,)>) -> impl Responder {
    HttpResponse::Ok()
}
