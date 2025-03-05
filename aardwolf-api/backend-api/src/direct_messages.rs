// direct_messages.rs
use crate::responses::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrivateMessage {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: String,
    pub created_at: String,
}

pub struct PrivateMessageReply(pub(crate) PrivateMessage);

impl Response for PrivateMessageReply {
    fn into_response(self) -> Self {
        self
    }
}
