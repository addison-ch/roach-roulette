use actix::prelude::{Message, Recipient};
use serde::Serialize;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub room_id: Uuid,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerListMessage {
    pub id: Uuid,
    pub room_id: Uuid,
}
#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct OutboundPlayerListMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    pub users: Vec<Uuid>,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct OutboundWelcomeMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct OutboundJoinMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct OutboundDisconnectMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    pub msg: String,
    pub room_id: Uuid,
}
