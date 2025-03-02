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

fn decode_walls(bytes: &[u8], expected_count: usize) -> Result<Vec<WallState>, &'static str> {
    if bytes.len() != 3 {
        return Err("Invalid wall bytes length");
    }

    let combined = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]);

    let mut walls = Vec::with_capacity(expected_count);
    for i in 0..expected_count {
        let shift = 22 - i * 2;
        let two_bits = ((combined >> shift) & 0b11) as u8;

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
    for (i, &wall) in walls.iter().enumerate() {
        let bits = wall.to_bits() as u32;
        let shift = 22 - i * 2;
        combined |= bits << shift;
    }

    // Convertir en little-endian
    let bytes = [
        (combined & 0xFF) as u8,
        ((combined >> 8) & 0xFF) as u8,
        ((combined >> 16) & 0xFF) as u8,
    ];
    Ok(bytes)
}

fn encode_cells(cells: &[RadarItem]) -> Result<[u8; 5], &'static str> {
    if cells.len() != 9 {
        return Err("Invalid cells count");
    }

    let mut bytes = [0u8; 5];

    for i in 0..5 {
        let idx1 = i * 2;
        let idx2 = i * 2 + 1;

        if idx1 < cells.len() {
            let high = cells[idx1].to_bits() << 4;
            bytes[i] |= high;
        }

        if idx2 < cells.len() {
            let low = cells[idx2].to_bits();
            bytes[i] |= low;
        }
    }

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
        // Utiliser une chaîne de test valide pour le décodage
        let valid_encoded = "AAAAAAAAAAA="; // 11 octets de zéros encodés en base64

        match decode_radarview(valid_encoded) {
            Ok(radar) => {
                let re_encoded = encode_radarview(&radar).unwrap();
                assert_eq!(valid_encoded, re_encoded);

                // Vérifier les dimensions
                assert_eq!(radar.horizontal.len(), 12);
                assert_eq!(radar.vertical.len(), 12);
                assert_eq!(radar.cells.len(), 9);

                // Vérifier que tous les murs sont Undefined (car tous les bits sont à 0)
                for wall in &radar.horizontal {
                    assert_eq!(*wall, WallState::Undefined);
                }

                for wall in &radar.vertical {
                    assert_eq!(*wall, WallState::Undefined);
                }

                // Vérifier que toutes les cellules sont Valid avec None/None
                for cell in &radar.cells {
                    match cell {
                        RadarItem::Valid { element, entity } => {
                            assert_eq!(*element, Element::None);
                            assert_eq!(*entity, Entity::None);
                        },
                        _ => panic!("Expected Valid cell, got Invalid"),
                    }
                }
            },
            Err(e) => {
                panic!("Failed to decode valid radar view: {}", e);
            }
        }
    }

    #[test]
    fn test_roundtrip() {
        // Créer un RadarView personnalisé
        let mut horizontal = vec![WallState::Undefined; 12];
        let mut vertical = vec![WallState::Undefined; 12];
        let mut cells = vec![RadarItem::Valid { element: Element::None, entity: Entity::None }; 9];

        // Modifier quelques valeurs pour un test plus significatif
        horizontal[0] = WallState::Wall;
        horizontal[5] = WallState::Open;
        vertical[2] = WallState::Wall;
        vertical[8] = WallState::Open;
        cells[4] = RadarItem::Valid { element: Element::Clue, entity: Entity::Ally };
        cells[7] = RadarItem::Valid { element: Element::Target, entity: Entity::Enemy };

        let radar = RadarView {
            horizontal,
            vertical,
            cells,
        };

        // Encoder puis décoder
        let encoded = encode_radarview(&radar).unwrap();
        let decoded = decode_radarview(&encoded).unwrap();

        // Vérifier que la structure est préservée
        assert_eq!(radar.horizontal, decoded.horizontal);
        assert_eq!(radar.vertical, decoded.vertical);
        assert_eq!(radar.cells, decoded.cells);
    }
}