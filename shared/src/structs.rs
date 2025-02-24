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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MazeData {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Position>,
    pub start: Position,
    pub end: Position,
}
