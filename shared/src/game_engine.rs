use std::{collections::HashMap};
use std::collections::{VecDeque};
use serde::{Deserialize, Serialize};

use crate::radar_view::{Cell, Element, Entity, RadarView};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub position: (i32, i32),
    pub name: String,
    pub secret: Option<u64>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Front,
    Right,
    Left,
    Back,
}

pub enum GlobalCell {
    Unknown,  // Zone encore non explorée
    Wall,     // Mur détecté
    Open,     // Passage libre
    Player,   // Position du joueur
    Goal,
}

#[derive(Debug)]
pub struct GameState {
    pub team_secrets: HashMap<String, u64>, // Secrets des coéquipiers (nom -> secret)
}

impl GameState {
    // Met à jour le secret du joueur courant ou des coéquipiers
    pub fn update_secret(&mut self, player_name: &str, secret: u64) {
        self.team_secrets.insert(player_name.to_string(), secret);
    }

    // Calcule la somme des secrets pour SecretSumModulo
    pub fn calculate_secret_sum_modulo(&self, modulo: u64) -> u64 {
        let mut total = 0;

        // Ajoute les secrets des coéquipiers
        for &secret in self.team_secrets.values() {
            total += secret;
        }

        total % modulo
    }
}

pub struct GlobalMap {
    pub map: HashMap<(i32, i32), GlobalCell>,
    pub player_pos: (i32, i32),  // Position actuelle du joueur
}

impl GlobalMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), GlobalCell::Player);  // Position initiale
        Self { map, player_pos: (0, 0) }
    }

    pub fn update_from_radar(&mut self, radar: &RadarView) {
        let (px, py) = self.player_pos;  // Position du joueur dans la carte globale

        // Convertir la vision RadarView en coordonnées globales
        for dy in 0..3 {
            for dx in 0..3 {
                let global_x = px + dx as i32 - 1; // Décale pour centrer la vue sur le joueur
                let global_y = py + dy as i32 - 1;
                
                let cell = match radar.cells[dy * 3 + dx] {
                    Cell::Invalid => GlobalCell::Unknown,
                    Cell::Valid { element, entity } => {
                        if let Element::Target = element {
                            GlobalCell::Goal
                        } else if let Entity::Ally = entity {
                            GlobalCell::Player
                        } else {
                            GlobalCell::Open
                        }
                    }
                };
                self.map.insert((global_x, global_y), cell);
            }
        }
    }
    pub fn get_direction(from: (i32, i32), to: (i32, i32), facing: Direction) -> Direction {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
    
        match facing {
            Direction::Front => match (dx, dy) {
                (0, -1) => Direction::Front,  // Aller en haut
                (1, 0) => Direction::Right,  // Aller à droite
                (-1, 0) => Direction::Left,  // Aller à gauche
                (0, 1) => Direction::Back,   // Aller en bas
                _ => facing,
            },
            Direction::Right => match (dx, dy) {
                (0, -1) => Direction::Left,
                (1, 0) => Direction::Front,
                (-1, 0) => Direction::Back,
                (0, 1) => Direction::Right,
                _ => facing,
            },
            Direction::Left => match (dx, dy) {
                (0, -1) => Direction::Right,
                (1, 0) => Direction::Back,
                (-1, 0) => Direction::Front,
                (0, 1) => Direction::Left,
                _ => facing,
            },
            Direction::Back => match (dx, dy) {
                (0, -1) => Direction::Back,
                (1, 0) => Direction::Left,
                (-1, 0) => Direction::Right,
                (0, 1) => Direction::Front,
                _ => facing,
            },
        }
    }
    

pub fn next_move(&self, facing: Direction) -> Direction {
    let (px, py) = self.player_pos;
    let moves = [
        (0, -1, Direction::Front), // Haut
        (1, 0, Direction::Right),  // Droite
        (-1, 0, Direction::Left),  // Gauche
        (0, 1, Direction::Back),   // Bas
    ];

    // Vérifier s'il y a un objectif découvert
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if let Some(GlobalCell::Goal) = self.map.get(&(nx, ny)) {
            return dir;
        }
    }

    // Chercher une zone inexplorée en priorité
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if !self.map.contains_key(&(nx, ny)) {
            return dir; // Aller vers la première zone inconnue
        }
    }

    // Sinon, aller vers une case ouverte
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if let Some(GlobalCell::Open) = self.map.get(&(nx, ny)) {
            return dir;
        }
    }

    // Sinon, rester sur place
    facing
}


pub fn explore_unknown(&self) -> Option<Vec<(i32, i32)>> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut came_from = HashMap::new();

    queue.push_back(self.player_pos);
    visited.insert(self.player_pos, true);

    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if visited.contains_key(&(nx, ny)) {
                continue;
            }

            match self.map.get(&(nx, ny)).unwrap_or(&GlobalCell::Unknown) {
                GlobalCell::Unknown => {
                    // On a trouvé une zone inexplorée, on s'arrête ici
                    let mut path = vec![];
                    let mut current = (x, y);
                    while let Some(prev) = came_from.get(&current) {
                        path.push(current);
                        current = *prev;
                    }
                    path.reverse();
                    return Some(path);
                }
                GlobalCell::Open | GlobalCell::Goal => {
                    queue.push_back((nx, ny));
                    visited.insert((nx, ny), true);
                    came_from.insert((nx, ny), (x, y));
                }
                GlobalCell::Wall => {}  // Impossible d'y aller
                _ => {}
            }
        }
    }
    None
}

}

fn get_direction(from: (i32, i32), to: (i32, i32), facing: Direction) -> Direction {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;

    match facing {
        Direction::Front => match (dx, dy) {
            (0, -1) => Direction::Front,  // Aller en haut
            (1, 0) => Direction::Right,  // Aller à droite
            (-1, 0) => Direction::Left,  // Aller à gauche
            (0, 1) => Direction::Back,   // Aller en bas
            _ => facing,
        },
        Direction::Right => match (dx, dy) {
            (0, -1) => Direction::Left,
            (1, 0) => Direction::Front,
            (-1, 0) => Direction::Back,
            (0, 1) => Direction::Right,
            _ => facing,
        },
        Direction::Left => match (dx, dy) {
            (0, -1) => Direction::Right,
            (1, 0) => Direction::Back,
            (-1, 0) => Direction::Front,
            (0, 1) => Direction::Left,
            _ => facing,
        },
        Direction::Back => match (dx, dy) {
            (0, -1) => Direction::Back,
            (1, 0) => Direction::Left,
            (-1, 0) => Direction::Right,
            (0, 1) => Direction::Front,
            _ => facing,
        },
    }
}



fn next_move(map: &GlobalMap, facing: Direction) -> Direction {
    let (px, py) = map.player_pos;
    let moves = [
        (0, -1, Direction::Front), // Haut
        (1, 0, Direction::Right),  // Droite
        (-1, 0, Direction::Left),  // Gauche
        (0, 1, Direction::Back),   // Bas
    ];

    // Vérifier s'il y a un objectif découvert
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if let Some(GlobalCell::Goal) = map.map.get(&(nx, ny)) {
            return dir;
        }
    }

    // Chercher une zone inexplorée en priorité
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if !map.map.contains_key(&(nx, ny)) {
            return dir; // Aller vers la première zone inconnue
        }
    }

    // Sinon, aller vers une case ouverte
    for &(dx, dy, dir) in &moves {
        let nx = px + dx;
        let ny = py + dy;
        if let Some(GlobalCell::Open) = map.map.get(&(nx, ny)) {
            return dir;
        }
    }

    // Sinon, rester sur place
    facing
}
