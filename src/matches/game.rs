use serde::{Deserialize, Serialize};

/// Game mode for the match
#[derive(Debug, Deserialize, Serialize)]
pub enum Game {
    Unknown = 0,
    AllPick = 1,
    CaptainsMode = 2,
    RandomDraft = 3,
    SingleDraft = 4,
    AllRandom = 5,
    Intro = 6,
    Diretide = 7,
    ReverseCaptainsMode = 8,
    TheGreeviling = 9,
    Tutorial = 10,
    MidOnly = 11,
    LeastPlayed = 12,
    LimitedHeroes = 13,
    CompendiumMatchmaking = 14,
    Custom = 15,
    CaptainsDraft = 16,
    BalancedDraft = 17,
    AbilityDraft = 18,
    Event = 19,
    AllRandomDeathmatch = 20,
    OneVsOneSoloMid = 21,
    AllDraft = 22,
    Turbo = 23,
    Mutation = 24,
    CoachesChallenge = 25,
}

impl From<u8> for Game {
    fn from(mode: u8) -> Self {
        match mode {
            1 => Self::AllPick,
            2 => Self::CaptainsMode,
            3 => Self::RandomDraft,
            4 => Self::SingleDraft,
            5 => Self::AllRandom,
            6 => Self::Intro,
            7 => Self::Diretide,
            8 => Self::ReverseCaptainsMode,
            9 => Self::TheGreeviling,
            10 => Self::Tutorial,
            11 => Self::MidOnly,
            12 => Self::LeastPlayed,
            13 => Self::LimitedHeroes,
            14 => Self::CompendiumMatchmaking,
            15 => Self::Custom,
            16 => Self::CaptainsDraft,
            17 => Self::BalancedDraft,
            18 => Self::AbilityDraft,
            19 => Self::Event,
            20 => Self::AllRandomDeathmatch,
            21 => Self::OneVsOneSoloMid,
            22 => Self::AllDraft,
            23 => Self::Turbo,
            24 => Self::Mutation,
            25 => Self::CoachesChallenge,
            _ => Self::Unknown,
        }
    }
}
