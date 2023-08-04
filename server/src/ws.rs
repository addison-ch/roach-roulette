use crate::lobby::Lobby;
use crate::messages::{
    ClientActorMessage, Connect, Disconnect, GeneralMessage, PlayerListMessage, WsMessage,
};
use actix::{fut, ActorContext, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;
// not sure, added this to fix .then error
use actix::ActorFutureExt;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// lobby_addr is needed in order to send messages to the lobby itself, and then send it to the rooms etc.
pub struct WsConn {
    pub room: Uuid,
    pub lobby_addr: Addr<Lobby>,
    pub hb: Instant,
    pub id: Uuid,
}

impl WsConn {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby,
        }
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"Heartbeat");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Web socket started");
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                room_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

// handles the heartbeats to make sure connection is alive
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),

            Ok(Text(data)) => {
                if data == "ping" {
                    ctx.text("pong"); // pong needs to be replaced with json.
                } else if data == "players" {
                    self.lobby_addr.do_send(PlayerListMessage {
                        id: self.id,
                        room_id: self.room,
                    });
                } else if data == "start_game" {
                    self.lobby_addr.do_send(GeneralMessage {
                        event_type: "start_game".to_string(),
                        msg: "success".to_string(),
                        room_id: self.room,
                    })
                } else {
                    self.lobby_addr.do_send(ClientActorMessage {
                        id: self.id,
                        msg: "Hey from the WsConn StreamHandler logic".to_string(),
                        room_id: self.room,
                    })
                }
            }
            Err(e) => std::panic::panic_any(e),
        }
    }
}

// the actual messages that different users send
impl Handler<WsMessage> for WsConn {
    type Result = ();
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        println!("WsConn Handler<WsMessage> received: {}", msg.0);
        ctx.text(msg.0);
    }
}
