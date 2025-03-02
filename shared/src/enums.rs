
use serde::{Deserialize, Serialize};

use crate::{game_engine::Direction, structs::{RegisterTeam, SubscribePlayer}};

#[derive(Deserialize, Serialize, Debug)]
pub enum RegisterTeamResult {
    Ok {
        expected_players: u8,
        registration_token: String,
    },
    Err(RegistrationError),
}

// pub enum ErrorMessage {
//     ConnectionError(Error),
// }

#[derive(Deserialize, Serialize, Debug)]
pub enum RegistrationError {
    AlreadyRegistered,
    InvalidName,
    InvalidRegistrationToken,
    TooManyPlayers,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    RegisterTeam(RegisterTeam),
    RegisterTeamResult(RegisterTeamResult),
    SubscribePlayer(SubscribePlayer),
    SubscribePlayerResult(SubscribePlayerResult),
    RadarView(String),
    Action(Action),
    Challenge(Challenge),
    ActionError(ActionError),
    Hint(Hint),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    MoveTo(Direction),
    SolveChallenge { answer: String },
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ActionError {
    CannotPassThroughWall,
    CannotPassThroughOpponent,
    NoRunningChallenge,
    SolveChallengeFirst,
    InvalidChallengeSolution,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Challenge {
    SecretSumModulo(u64),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Hint {
    RelativeCompass { angle: f32 },
    GridSize { columns: u32, rows: u32 },
    Secret(u64),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribePlayerResult {
    Ok,
    Err(RegistrationError),
}
