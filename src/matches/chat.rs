use serde::{Deserialize, Serialize};

/// Chat data
#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    /// Time in seconds at which the message was said
    pub time: i32,
    /// Name of the player who sent the message
    pub unit: Option<String>,
    /// The message the player sent
    pub key: String,
    pub slot: u8,
    /// Which slot the player is in. 0-127 are Radiant, 128-255 are Dire
    pub player_slot: u8,
}
