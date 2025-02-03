use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::structs::{RegisterTeam, SubscribePlayer};


#[derive(Deserialize, Serialize, Debug)]
pub enum RegisterTeamResult {
    Ok {
        expected_players: u8,
        registration_token: String,
    },
    Err(RegistrationError),
}


#[derive(Deserialize, Serialize, Debug)]
pub enum RegistrationError {
    AlreadyRegistered,
    InvalidName,
    InvalidRegistrationToken,
    TooManyPlayers,
}

use std::fmt;

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::AlreadyRegistered => write!(f, "Team already registered"),
            RegistrationError::InvalidName => write!(f, "Invalid team name"),
            RegistrationError::InvalidRegistrationToken => write!(f, "Invalid registration token"),
            RegistrationError::TooManyPlayers => write!(f, "Too many players in the team"),
        }
    }
}

impl Error for RegistrationError {}

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    RegisterTeam(RegisterTeam),
    RegisterTeamResult(RegisterTeamResult),
    SubscribePlayer(SubscribePlayer),
    SubscribePlayerResult(SubscribePlayerResult),
}


#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribePlayerResult {
    Ok,
    Err(RegistrationError),
}