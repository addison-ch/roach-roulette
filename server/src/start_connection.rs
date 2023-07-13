use crate::lobby::Lobby;
use crate::ws::WsConn;
use actix::Addr;
use actix_web::Responder;
use actix_web::{web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    info: Path<(Uuid, u32)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("hello");
    // added this to fix error, not sure if will work
    let group_id = info.into_inner().0;
    // random room generation code
    let ws = WsConn::new(group_id, srv.get_ref().clone());

    let resp: HttpResponse = ws::start(ws, &req, stream)?;
    Ok(resp)
}

pub async fn supersimple(_info: Path<(String,)>) -> impl Responder {
    HttpResponse::Ok()
}
