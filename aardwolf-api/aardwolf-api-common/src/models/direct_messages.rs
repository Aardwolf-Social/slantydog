// aardwolf-api-common/src/models/direct_messages.rs
use serde::{Deserialize, Serialize};
use crate::responses::Response;

#[derive(Serialize, Deserialize)]
pub struct PrivateMessage {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateMessageReply(pub(crate) PrivateMessage);

impl Response for PrivateMessageReply {
    type Output = Self; // Keeps it agnostic

    fn into_response(self) -> Self::Output {
        self
    }
}
