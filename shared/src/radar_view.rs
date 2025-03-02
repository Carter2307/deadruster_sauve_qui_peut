use crate::base64::{decode, encode};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WallState {
    Undefined,
    Open,
    Wall,
}

impl WallState {
    fn from_bits(bits: u8) -> Result<Self, &'static str> {
        match bits {
            0 => Ok(WallState::Undefined),
            1 => Ok(WallState::Open),
            2 => Ok(WallState::Wall),
            _ => Err("Invalid wall state bits"),
        }
    }

    fn to_bits(&self) -> u8 {
        match self {
            WallState::Undefined => 0,
            WallState::Open => 1,
            WallState::Wall => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Element {
    None,
    Hint,
    Target,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Entity {
    None,
    Ally,
    Enemy,
    Monster,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Valid { element: Element, entity: Entity },
    Invalid,
}

impl Cell {
    fn from_bits(bits: u8) -> Result<Self, &'static str> {
        if bits == 0b1111 {
            return Ok(Cell::Invalid);
        }
        let element_bits = (bits >> 2) & 0b11;
        let entity_bits = bits & 0b11;

        let element = match element_bits {
            0b00 => Element::None,
            0b01 => Element::Hint,
            0b10 => Element::Target,
            _ => return Err("Invalid element bits"),
        };

        let entity = match entity_bits {
            0b00 => Entity::None,
            0b01 => Entity::Ally,
            0b10 => Entity::Enemy,
            0b11 => Entity::Monster,
            _ => return Err("Invalid Entity"),

        };

        Ok(Cell::Valid { element, entity })
    }

    fn to_bits(&self) -> u8 {
        match self {
            Cell::Invalid => 0b1111,
            Cell::Valid { element, entity } => {
                let element_bits = match element {
                    Element::None => 0b00,
                    Element::Hint => 0b01,
                    Element::Target => 0b10,
                };
                let entity_bits = match entity {
                    Entity::None => 0b00,
                    Entity::Ally => 0b01,
                    Entity::Enemy => 0b10,
                    Entity::Monster => 0b11,
                };
                (element_bits << 2) | entity_bits
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GridCell {
    Unknown,
    Wall,
    Player,
    Open,
    Goal // Target
}

#[derive(Debug, PartialEq, Eq)]
pub struct RadarView {
    pub horizontal: Vec<WallState>,
    pub vertical: Vec<WallState>,
    pub cells: Vec<Cell>,
}

impl RadarView {
    fn radar_to_grid(&self) -> [[GridCell; 3]; 3] {
        let mut grid = [[GridCell::Unknown; 3]; 3];

        for (i, cell) in self.cells.iter().enumerate() {
            let x = i % 3;
            let y = i / 3;

            grid[y][x] = match cell {
                Cell::Invalid => GridCell::Unknown,
                Cell::Valid { element, entity } => {
                    if *element == Element::Target {
                        GridCell::Goal
                    } else if *entity == Entity::Ally {
                        GridCell::Player
                    } else {
                        GridCell::Open
                    }
                }
            }
        }

        grid
    }

    fn add_horizontal_walls(&self, grid: &mut [[GridCell; 3]; 3]) {
        for y in 0..2 {
            for x in 0..2 {
                let index = y * 3 + x;
                if let WallState::Wall = self.horizontal[index] {
                    grid[y * 2 + 1][x * 2] = GridCell::Wall
                }
            }
        }
    }

    fn add_vertical_walls(&self, grid: &mut [[GridCell; 3]; 3]) {
        for y in 0..3 {
            for x in 0..2 {
                let index = y * 2 + x;
                if let WallState::Wall = self.vertical[index] {
                    grid[y * 2][x * 2 + 1] = GridCell::Wall;  // Position des murs verticaux
                }
            }
        }
    }

    fn print_grid(grid: &[[GridCell; 3]; 3]) {
        for row in grid.iter() {
            for cell in row.iter() {
                match cell {
                    GridCell::Unknown => print!("# "),  // Case inexplorée
                    GridCell::Wall => print!("█ "),    // Mur
                    GridCell::Open => print!(". "),    // Passage
                    GridCell::Player => print!("P "),  // Joueur
                    GridCell::Goal => print!("G "),    // Objectif
                }
            }
            println!();
        }
    }
    
    
}

// DÉCODEUR
fn decode_walls(bytes: &[u8], expected_count: usize) -> Result<Vec<WallState>, &'static str> {
    if bytes.len() != 3 {
        return Err("Invalid wall bytes length");
    }

    // Le ">> 8" décale des bits vers la droite
    // Le Décalage à droite (>> n) : Ajoute n zéros à gauche et supprime n bits à droite.
    // Le Décalage à gauche (<< n) : Ajoute n zéros à droite et supprime n bits à gauche.
    // En gros le décalage permet de supprimer le padding à la fin
    let combined = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]);

    //print!("{:08b} \n", &combined);

    let mut walls = Vec::with_capacity(expected_count);
    for i in 0..expected_count {
        let shift = 22 - i * 2;
        let two_bits = ((combined >> shift) & 0b11) as u8;

        //print!("{:02b} \n", &two_bits);

        let wall = WallState::from_bits(two_bits)?;
        walls.push(wall);
    }

    Ok(walls)
}

fn decode_cells(bytes: &[u8]) -> Result<Vec<Cell>, &'static str> {
    if bytes.len() != 5 {
        return Err("Invalid cell bytes length");
    }

    let mut cells = Vec::with_capacity(9);

    for &byte in bytes.iter() {
        let high = (byte >> 4) & 0x0F; // 4 bits de gauche
        let low = byte & 0x0F; // 4 bits de droite

        cells.push(Cell::from_bits(high)?);
        cells.push(Cell::from_bits(low)?);
    }

    // Supprimer le padding final si nécessaire
    if cells.len() > 9 {
        cells.truncate(9);
    }

    Ok(cells)
}

pub fn decode_radarview(encoded: &str) -> Result<RadarView, &'static str> {
    let bytes = decode(encoded)?;

    // for byte in &bytes {
    //     print!("{:08b} \n", &byte);
    // }

    if bytes.len() != 11 {
        return Err("Invalid radar view bytes length");
    }

    let horizontal = decode_walls(&bytes[0..3], 12)?;
    let vertical = decode_walls(&bytes[3..6], 12)?;
    let cells = decode_cells(&bytes[6..11])?;

    Ok(RadarView {
        horizontal,
        vertical,
        cells,
    })
}

// ENCODEUR
fn encode_walls(walls: &[WallState]) -> Result<[u8; 3], &'static str> {
    if walls.len() != 12 {
        return Err("Invalid horizontal or vertical walls count");
    }

    let mut combined = 0u32;
    for &wall in walls {
        let bits = wall.to_bits() as u32;
        combined = (combined << 2) | bits;
    }

    // Convertir en little-endian en inversant l'ordre des octets
    let bytes = [
        combined as u8,
        (combined >> 8) as u8,
        (combined >> 16) as u8,
    ];
    Ok(bytes)
}

fn encode_cells(cells: &[Cell]) -> Result<[u8; 5], &'static str> {
    if cells.len() != 9 {
        return Err("Invalid cells count");
    }

    let mut buffer = 0u64;
    for cell in cells {
        buffer = (buffer << 4) | cell.to_bits() as u64;
    }
    buffer <<= 4;

    let bytes = [
        (buffer >> 32) as u8,
        (buffer >> 24) as u8,
        (buffer >> 16) as u8,
        (buffer >> 8) as u8,
        buffer as u8,
    ];

    Ok(bytes)
}

pub fn encode_radarview(radar: &RadarView) -> Result<String, &'static str> {
    let horizontal_bytes = encode_walls(&radar.horizontal)?;
    let vertical_bytes = encode_walls(&radar.vertical)?;
    let cell_bytes = encode_cells(&radar.cells)?;

    let mut bytes = Vec::with_capacity(11);
    bytes.extend_from_slice(&horizontal_bytes);
    bytes.extend_from_slice(&vertical_bytes);
    bytes.extend_from_slice(&cell_bytes);

    Ok(encode(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_encode() {
        let encoded = "ieysGjGO8papd/a";
        let radar = decode_radarview(encoded).unwrap();
        let re_encoded = encode_radarview(&radar).unwrap();

        print!("{:?}", radar);
        assert_eq!(encoded, re_encoded);
    }
}
