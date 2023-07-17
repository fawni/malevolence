use serde::{Deserialize, Serialize};

/// Lobby type for the match
#[derive(Debug, Deserialize, Serialize)]
pub enum Lobby {
    Normal = 0,
    Practice = 1,
    Tournament = 2,
    Tutorial = 3,
    CoopWithBots = 4,
    RankedTeamMM = 5,
    RankedSoloMM = 6,
    Ranked = 7,
    OneVsOneSoloMid = 8,
    BattleCup = 9,
    LocalBots = 10,
    Spectator = 11,
    Event = 12,
    Gauntlet = 13,
    NewPlayer = 14,
    Featured = 15,
}

impl From<u8> for Lobby {
    fn from(lobby: u8) -> Self {
        match lobby {
            1 => Self::Practice,
            2 => Self::Tournament,
            3 => Self::Tutorial,
            4 => Self::CoopWithBots,
            5 => Self::RankedTeamMM,
            6 => Self::RankedSoloMM,
            7 => Self::Ranked,
            8 => Self::OneVsOneSoloMid,
            9 => Self::BattleCup,
            10 => Self::LocalBots,
            11 => Self::Spectator,
            12 => Self::Event,
            13 => Self::Gauntlet,
            14 => Self::NewPlayer,
            15 => Self::Featured,
            _ => Self::Normal,
        }
    }
}
