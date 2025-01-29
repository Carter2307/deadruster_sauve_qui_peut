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
}


#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribePlayerResult {
    Ok,
    Err(RegistrationError),
}