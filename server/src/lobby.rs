use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Recipient type lets you send messages
type Socket = Recipient<WsMessage>;

// keeps track of all rooms and websocket connections
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>, //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id map to list of users id
}

// implements Default trait to Lobby, so that lobby has default values when you call default function
impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid){
        if let Some(socket_recipient) = self.sessions.get(id_to)  {
            let _ = socket.recipient.do_send(WsMessage(message.to_owned()));
        }
    }
}

// makes lobby an actor, so that it can receive messages
impl Actor for Lobby {
    type Context = Context<Self>;
}