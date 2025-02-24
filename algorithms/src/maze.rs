use std::collections::{HashSet, VecDeque};
use shared::structs::Position;

#[derive(Debug)]
pub struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    start: Position,
    end: Position,
}

impl Maze {
    pub fn new(width: usize, height: usize, start: Position, end: Position) -> Self {
        Maze {
            width,
            height,
            walls: HashSet::new(),
            start,
            end,
        }
    }

    pub fn add_wall(&mut self, pos: Position) {
        self.walls.insert(pos);
    }

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions {
            let new_x = pos.x as isize + dx;
            let new_y = pos.y as isize + dy;

            if new_x >= 0 && new_x < self.width as isize &&
               new_y >= 0 && new_y < self.height as isize {
                let new_pos = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                if !self.walls.contains(&new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }
        neighbors
    }

    pub fn solve(&self) -> Option<Vec<Position>> {
        println!("🔍 Démarrage de l'algorithme de résolution !");
        println!("Labyrinthe : taille {}x{}", self.width, self.height);
        println!("Départ : {:?}, Arrivée : {:?}", self.start, self.end);
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = std::collections::HashMap::new();
    
        queue.push_back(self.start);
        visited.insert(self.start);
    
        while let Some(current) = queue.pop_front() {
            println!("Exploration de la case {:?}", current);
            
            if current == self.end {
                println!("✅ Chemin trouvé !");
                return Some(self.reconstruct_path(&came_from));
            }
    
            for neighbor in self.get_neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    came_from.insert(neighbor, current);
                    queue.push_back(neighbor);
                    println!("Ajout du voisin {:?} à la file d'attente", neighbor);
                }
            }
        }
    
        println!("❌ Aucun chemin trouvé !");
        None
    }
    

    fn reconstruct_path(&self, came_from: &std::collections::HashMap<Position, Position>) -> Vec<Position> {
        let mut path = Vec::new();
        let mut current = self.end;
        
        path.push(current);
        while current != self.start {
            if let Some(&prev) = came_from.get(&current) {
                current = prev;
                path.push(current);
            } else {
                return Vec::new(); // Chemin invalide
            }
        }
        path.reverse();
        path
    }
}
