//use crate::base64::{decode, encode};
use std::error::Error;
use std::io::{Cursor, Read, Write};

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
