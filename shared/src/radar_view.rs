
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
    Clue,
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
pub enum RadarItem {
    Valid { element: Element, entity: Entity },
    Invalid,
}

impl RadarItem {
    fn from_bits(bits: u8) -> Result<Self, &'static str> {
        if bits == 0b1111 {
            return Ok(RadarItem::Invalid);
        }
        let element_bits = (bits >> 2) & 0b11;
        let entity_bits = bits & 0b11;

        let element = match element_bits {
            0b00 => Element::None,
            0b01 => Element::Clue,
            0b10 => Element::Target,
            _ => return Err("Invalid element bits"),
        };

        let entity = match entity_bits {
            0b00 => Entity::None,
            0b01 => Entity::Ally,
            0b10 => Entity::Enemy,
            0b11 => Entity::Monster,
            _ => unreachable!(),
        };

        Ok(RadarItem::Valid { element, entity })
    }

    fn to_bits(&self) -> u8 {
        match self {
            RadarItem::Invalid => 0b1111,
            RadarItem::Valid { element, entity } => {
                let element_bits = match element {
                    Element::None => 0b00,
                    Element::Clue => 0b01,
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

#[derive(Debug, PartialEq, Eq)]
pub struct RadarView {
    pub horizontal: Vec<WallState>,
    pub vertical: Vec<WallState>,
    pub cells: Vec<RadarItem>,
}

// DÉCODEUR
fn decode(encoded: &str) -> Result<Vec<u8>, &'static str> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    let bytes = STANDARD.decode(encoded).map_err(|_| "Invalid base64 string")?;
    Ok(bytes)
}
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

fn decode_cells(bytes: &[u8]) -> Result<Vec<RadarItem>, &'static str> {
    if bytes.len() != 5 {
        return Err("Invalid cell bytes length");
    }

    let mut cells = Vec::with_capacity(9);

    for &byte in bytes.iter() {
        let high = (byte >> 4) & 0x0F; // 4 bits de gauche
        let low = byte & 0x0F;         // 4 bits de droite

        cells.push(RadarItem::from_bits(high)?);
        cells.push(RadarItem::from_bits(low)?);
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
