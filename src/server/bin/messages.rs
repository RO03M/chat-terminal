use actix::{Message, Recipient};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Text(pub String);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub recipient: Recipient<Text>,
    pub lobby_id: Uuid,
    pub session_id: Uuid
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: Uuid
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct LobbyMessage {
    pub session_id: Uuid,
    pub message: String
}
