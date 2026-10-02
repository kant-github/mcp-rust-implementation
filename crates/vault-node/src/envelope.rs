use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub session_id: String,
    pub sender_id: u16,
    pub broadcast: bool,
    pub payload: Value,
}
