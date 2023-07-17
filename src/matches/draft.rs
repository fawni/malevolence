use serde::{Deserialize, Serialize};

/// Draft timings data
#[derive(Debug, Deserialize, Serialize)]
pub struct Draft {
    pub order: u8,
    pub pick: bool,
    pub active_team: u8,
    /// The ID value of the hero played
    pub hero_id: u16,
    /// Which slot the player is in. 0-127 are Radiant, 128-255 are Dire
    pub player_slot: Option<u8>,
    pub extra_time: u8,
    pub total_time_taken: u8,
}
