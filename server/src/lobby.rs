use crate::messages::{
    ClientActorMessage, Connect, Disconnect, OutboundDisconnectMessage, OutboundJoinMessage,
    OutboundPlayerListMessage, OutboundWelcomeMessage, PlayerListMessage, WsMessage, GeneralMessage
};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Recipient type lets you send messages
type Socket = Recipient<WsMessage>;

// implements Default trait to Lobby, so that lobby has default values when you call default function
#[derive(Default)]
// keeps track of all rooms and websocket connections
pub struct Lobby {
    pub sessions: HashMap<Uuid, Socket>,     //self id to self
    pub rooms: HashMap<Uuid, HashSet<Uuid>>, //room id map to list of users id
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    fn send_player_list(&self, room_id: &Uuid, id_to: &Uuid) {
        if let Some(users) = self.get_users_in_room(room_id) {
            let users: Vec<Uuid> = users.iter().cloned().collect();
            let message = OutboundPlayerListMessage {
                event_type: "player_list".to_string(),
                users,
            };
            self.send_message(&serde_json::to_string(&message).unwrap(), id_to);
        } else {
            println!("attempting to send player list but couldn't find room id.");
        }
    }

    fn print_sessions(&self) {
        println!("sessions: {:?}", self.sessions);
    }

    fn print_rooms(&self) {
        println!("rooms: {:?}", self.rooms);
    }

    fn get_users_in_room(&self, room_id: &Uuid) -> Option<&HashSet<Uuid>> {
        self.rooms.get(room_id)
    }
}

// makes lobby an actor, so that it can receive messages
impl Actor for Lobby {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

// Connect new users to the lobby
impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let welcome_msg = OutboundWelcomeMessage {
            event_type: "welcome".to_string(),
            id: msg.self_id,
            msg: format!("Your name is {}!", msg.self_id),
            room_id: msg.room_id,
        };

        let join_notif = OutboundJoinMessage {
            event_type: "join".to_string(),
            id: msg.self_id,
            msg: format!("{} has joined the lobby!", msg.self_id),
            room_id: msg.room_id,
        };

        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                self.send_message(&serde_json::to_string(&join_notif).unwrap(), conn_id)
            });

        self.sessions.insert(msg.self_id, msg.addr);

        self.send_message(&serde_json::to_string(&welcome_msg).unwrap(), &msg.self_id);
    }
}

// other stuff to stop the errors
// handles disconnect
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let disconnect_msg = OutboundDisconnectMessage {
            event_type: "disconnect".to_string(),
            msg: format!("{} has left the lobby!", msg.id),
            room_id: msg.room_id,
        };

        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&serde_json::to_string(&disconnect_msg).unwrap(), user_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

// handles messages that the client sends
impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.print_sessions();
        self.print_rooms();
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}

impl Handler<PlayerListMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: PlayerListMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // send player list
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_player_list(&msg.room_id, client));
    }
}

impl Handler<GeneralMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: GeneralMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|conn_id| {
                self.send_message(&serde_json::to_string(&msg).unwrap(), conn_id)
            });
    }
}
