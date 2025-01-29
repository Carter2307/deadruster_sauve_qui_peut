use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterTeam {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribePlayer {
    pub name: String,
    pub registration_token: String,
}
