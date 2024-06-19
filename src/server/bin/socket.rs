use std::time::Instant;

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    lobby::Lobby,
    messages::{Connect, Disconnect, LobbyMessage, Text},
};

pub struct WsConnection {
    lobby_addr: Addr<Lobby>,
    id: Uuid,
}

impl WsConnection {
    pub fn new(lobby_addr: Addr<Lobby>) -> Self {
        Self {
            lobby_addr,
            id: Uuid::new_v4(),
        }
    }

    // pub fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
    //     ctx.run_interval(Duration::from_secs(5), |act, ctx| {
    //         if (Instant::now().duration_since(act.heartbeat(ctx)))
    //     })
    // }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("New connection!");

        let addr = ctx.address();

        self.lobby_addr.do_send(Connect {
            lobby_id: Uuid::new_v4(),
            recipient: addr.recipient(),
            session_id: self.id,
        })
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(Disconnect {
            session_id: self.id,
        });

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Close(reason)) => {
                println!("Session {} closed", self.id);
                ctx.close(None);
                ctx.stop();
            }
            Ok(ws::Message::Text(text)) => {
                self.lobby_addr.do_send(LobbyMessage {
                    message: text.clone().into(),
                    session_id: self.id,
                });
                println!("{}", text);
            }
            Ok(_) => todo!("Implement"),
            Err(_) => println!("Failed to handle the message"),
        }
    }
}

impl Handler<Text> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: Text, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}
