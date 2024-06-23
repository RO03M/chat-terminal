use std::collections::{HashMap, HashSet};

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use crate::messages::{Connect, Disconnect, LobbyMessage, Text};

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<Uuid, Recipient<Text>>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Lobby {
    fn send_message_to_session(&self, message: &str, session_id: &Uuid) {
        if let Some(target_socket) = self.sessions.get(session_id) {
            target_socket.do_send(Text(message.into()));
        } else {
            println!("Couldn't find target");
        }
    }
}



impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, new_session_data: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("{new_session_data:?}");

        new_session_data
            .recipient
            .do_send(Text("Bem vindo ao lobby!!".into()));

        self.sessions
            .insert(new_session_data.session_id, new_session_data.recipient);
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.session_id);

        println!("Session {} removed", msg.session_id);
    }
}

impl Handler<LobbyMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: LobbyMessage, _ctx: &mut Self::Context) -> Self::Result {
        for (session_id, _) in &self.sessions {
            self.send_message_to_session(&msg.message, session_id)
        }
    }
}
