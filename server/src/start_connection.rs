use crate::ws::WsConn;
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    info: Path<(Uuid, u32)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    // added this to fix error, not sure if will work
    println!("hello");
    let group_id = info.into_inner().0;
    let ws = WsConn::new(
        group_id,
        srv.get_ref().clone(),
    );

    let resp: HttpResponse = ws::start(ws, &req, stream)?;
    Ok(resp)
}