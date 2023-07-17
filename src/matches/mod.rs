use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Client, Result, Team};

pub mod chat;
pub mod draft;
pub mod game;
pub mod lobby;

use chat::Chat;
use draft::Draft;
use game::Game;
use lobby::Lobby;

impl Client {
    /// Get a match by its ID
    /// # Errors
    ///
    /// Returns an error if the API request fails
    pub async fn get_match(&self, id: i64) -> Result<Match> {
        let url = self.key.as_ref().map_or_else(
            || format!("https://api.opendota.com/api/matches/{id}"),
            |key| format!("https://api.opendota.com/api/matches/{id}?api_key={key}",),
        );

        self.json(url).await
    }
}

/// Match data
#[derive(Debug, Deserialize, Serialize)]
pub struct Match {
    /// The ID number of the match assigned by Valve
    #[serde(rename = "match_id")]
    pub id: i64,
    /// Bitmask. An integer that represents a binary of which barracks are still standing. 63 would mean all barracks still stand at the end of the game
    pub barracks_status_dire: u8,
    /// Bitmask. An integer that represents a binary of which barracks are still standing. 63 would mean all barracks still stand at the end of the game
    pub barracks_status_radiant: u8,
    /// Vector containing information on the chat of the game
    pub chat: Option<Vec<Chat>>,
    pub cluster: u32,
    pub cosmetics: Option<HashMap<String, u8>>,
    /// Number of kills the Dire team had when the match ended
    pub dire_score: u32,
    /// Draft timings
    pub draft_timings: Option<Vec<Draft>>,
    /// Duration of the game in seconds
    pub duration: u32,
    pub engine: u8,
    /// Time in seconds at which first blood occurred
    pub first_blood_time: u32,
    /// Integer corresponding to game mode played
    pub game_mode: u8,
    /// Number of human players in the match
    pub human_players: u8,
    pub leagueid: u32,
    /// Integer corresponding to lobby type of match
    pub lobby_type: u8,

    /// Boolean indicating whether radiant won the match
    pub radiant_win: bool,
}

impl Match {
    /// Get the winning team
    #[must_use]
    pub const fn winner(&self) -> Team {
        if self.radiant_win {
            Team::Radiant
        } else {
            Team::Dire
        }
    }

    /// Get the game mode
    #[must_use]
    pub fn game_mode(&self) -> Game {
        Game::from(self.game_mode)
    }

    /// Get the lobby type
    #[must_use]
    pub fn lobby_type(&self) -> Lobby {
        Lobby::from(self.lobby_type)
    }
}
