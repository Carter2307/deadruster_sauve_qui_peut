use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::radar_view::{Cell, Element, Entity, RadarView};

// Les directions possibles
#[derive(Clone, Copy)]
pub enum Direction {
    North, // Vers le haut
    East,  //  Vers la droite
    West,  //  Vers la gauche
    South, // Vers le bas
}

#[derive(Debug)]
pub struct GameState {
    pub team_secrets: HashMap<String, u64>, // Secrets des coéquipiers (nom -> secret)
    pub map: GameMap,                       // Carte des cellules découvertes
    pub visited: HashSet<(i32, i32)>,       // Positions déjà visitées
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

#[derive(Debug)]
pub struct GameMap {
    pub cells: HashMap<(i32, i32), Cell>,
}

impl GameState {
    pub fn update_from_radar(&mut self, player: &Player, radar: &RadarView) {
        for (i, cell) in radar.cells.iter().enumerate() {
            if let Cell::Valid { element, entity } = cell {
                let (dx, dy) = get_relative_position(i);
                let abs_x = player.position.0 + dx;
                let abs_y = player.position.1 + dy;
                self.map.cells.insert(
                    (abs_x, abs_y),
                    Cell::Valid {
                        element: *element,
                        entity: *entity,
                    },
                );

                // Marquer la position actuelle comme visitée
                self.visited.insert(player.position);
            }
        }
    }
}

pub struct Player {
    pub name: String,
    pub position: (i32, i32), // Position du joueur (x, y)
    pub direction: Direction, // Orientation actuelle (ou est ce que le joueur regarde)
    pub secret: Option<u64>,  // Dernier secret reçu par ce joueur
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
// Les directions dépendes de l'orientation du joueur.
pub enum RelativeDirection {
    Front,
    Back,
    Left,
    Right,
}

// Fonction qui donne le *décalage*  (dx, dy) du joueur selon *l'orientation* et *la direction* choisie
pub fn get_offset(direction: Direction, relative: &RelativeDirection) -> (i32, i32) {
    match (direction, relative) {
        // Orientation : North
        (Direction::North, RelativeDirection::Front) => (0, 1), // Haut
        (Direction::North, RelativeDirection::Right) => (1, 0), // Droite
        (Direction::North, RelativeDirection::Back) => (0, -1), // Bas
        (Direction::North, RelativeDirection::Left) => (-1, 0), // Gauche

        // Orientation : East
        (Direction::East, RelativeDirection::Front) => (1, 0), // Droite
        (Direction::East, RelativeDirection::Right) => (0, -1), // Bas
        (Direction::East, RelativeDirection::Back) => (-1, 0), // Gauche
        (Direction::East, RelativeDirection::Left) => (0, 1),  // Haut

        // Orientation : South
        (Direction::South, RelativeDirection::Front) => (0, -1), // Bas
        (Direction::South, RelativeDirection::Right) => (-1, 0), // Gauche
        (Direction::South, RelativeDirection::Back) => (0, 1),   // Haut
        (Direction::South, RelativeDirection::Left) => (1, 0),   // Droite

        // Orientation : West
        (Direction::West, RelativeDirection::Front) => (-1, 0), // Gauche
        (Direction::West, RelativeDirection::Right) => (0, 1),  // Haut
        (Direction::West, RelativeDirection::Back) => (1, 0),   // Droite
        (Direction::West, RelativeDirection::Left) => (0, -1),  // Bas
    }
}

pub fn move_player(player: &mut Player, relative: &RelativeDirection) {
    let (dx, dy) = get_offset(player.direction, relative);
    player.position = (player.position.0 + dx, player.position.1 + dy);
    // Note : On ne change pas l'orientation ici, juste la position.
}

pub fn get_relative_position(index: usize) -> (i32, i32) {
    match index {
        0 => (-1, 1),  // Nord-Ouest
        1 => (0, 1),   // Nord
        2 => (1, 1),   // Nord-Est
        3 => (-1, 0),  // Ouest
        4 => (0, 0),   // Centre
        5 => (1, 0),   // Est
        6 => (-1, -1), // Sud-Ouest
        7 => (0, -1),  // Sud
        8 => (1, -1),  // Sud-Est
        _ => panic!("Index invalide"),
    }
}

pub fn get_radar_index_for_direction(
    player_direction: Direction,
    relative: RelativeDirection,
) -> usize {
    match (player_direction, relative) {
        // Orientation North
        (Direction::North, RelativeDirection::Front) => 1, // Nord
        (Direction::North, RelativeDirection::Right) => 5, // Est
        (Direction::North, RelativeDirection::Back) => 7,  // Sud
        (Direction::North, RelativeDirection::Left) => 3,  // Ouest

        // Orientation East
        (Direction::East, RelativeDirection::Front) => 5, // Est
        (Direction::East, RelativeDirection::Right) => 7, // Sud
        (Direction::East, RelativeDirection::Back) => 3,  // Ouest
        (Direction::East, RelativeDirection::Left) => 1,  // Nord

        // Orientation South
        (Direction::South, RelativeDirection::Front) => 7, // Sud
        (Direction::South, RelativeDirection::Right) => 3, // Ouest
        (Direction::South, RelativeDirection::Back) => 1,  // Nord
        (Direction::South, RelativeDirection::Left) => 5,  // Est

        // Orientation West
        (Direction::West, RelativeDirection::Front) => 3, // Ouest
        (Direction::West, RelativeDirection::Right) => 1, // Nord
        (Direction::West, RelativeDirection::Back) => 5,  // Est
        (Direction::West, RelativeDirection::Left) => 7,  // Sud
    }
}

pub fn can_move_to_direction(
    radar: &RadarView,
    player_direction: Direction,
    relative: RelativeDirection,
) -> bool {
    let index = get_radar_index_for_direction(player_direction, relative);
    print!("cell item: {:?}\n", radar.cells[index]);
    matches!(
        radar.cells[index],
        Cell::Valid {
            element: Element::None,
            entity: Entity::None
        }
    )
    // Note : Vous pouvez ajouter des conditions supplémentaires ici, par exemple :
    // - Vérifier que la cellule n'est pas occupée par un monstre
    // - Vérifier la présence de murs dans radar.horizontal ou radar.vertical
}

pub fn choose_next_move(
    player: &Player,
    state: &GameState,
    radar: &RadarView,
) -> RelativeDirection {
    let directions = [
        RelativeDirection::Right,
        RelativeDirection::Front,
        RelativeDirection::Left,
        RelativeDirection::Back,
    ];

    // Vérifier si la sortie est visible dans la RadarView
    for &direction in &directions {
        let index = get_radar_index_for_direction(player.direction, direction);
        if let Cell::Valid {
            element: Element::Target,
            entity: Entity::None,
        } = radar.cells[index]
        {
            print!("La sortie est visible dans la RadarView \n");
            return direction; // Aller directement vers la sortie
        }
    }

    // Explorer une direction non visitée
    for &direction in &directions {
        if can_move_to_direction(radar, player.direction, direction) {
            let (dx, dy) = get_offset(player.direction, &direction);
            let next_pos = (player.position.0 + dx, player.position.1 + dy);
            if !state.visited.contains(&next_pos) {
                return direction; // Explorer une nouvelle case
            }
        }
    }

    // Si toutes les directions adjacentes sont visitées, revenir en arrière
    for &direction in &directions {
        if can_move_to_direction(radar, player.direction, direction) {
            return direction; // Revenir en arrière vers une case déjà visitée
        }
    }

    // Par défaut (ne devrait pas arriver si le labyrinthe est bien formé)
    RelativeDirection::Back
}
