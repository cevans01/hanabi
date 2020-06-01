extern crate bitflags;

use crate::errors::HanabiError;

// TODO: make a macro that makes both Color, ColorKnowledge, and impls the From trait
// TODO: same with Number, etc...

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red = 0b00001,
    White = 0b00010,
    Blue = 0b00100,
    Green = 0b01000,
    Yellow = 0b10000,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    One = 0b00001,
    Two = 0b00010,
    Three = 0b00100,
    Four = 0b01000,
    Five = 0b10000,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub color: Color,
    pub number: Number,
}

// Private here
bitflags! {
    #[derive(Default)]
    pub struct ColorKnowledge: u32 {
        const RED    = Color::Red    as u32;
        const WHITE  = Color::White  as u32;
        const BLUE   = Color::Blue   as u32;
        const GREEN  = Color::Green  as u32;
        const YELLOW = Color::Yellow as u32;
        const ALL_COLORS = Self::RED.bits | Self::WHITE.bits | Self::BLUE.bits | Self::GREEN.bits | Self::YELLOW.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct NumberKnowledge: u32 {
        const ONE   = Number::One   as u32;
        const TWO   = Number::Two   as u32;
        const THREE = Number::Three as u32;
        const FOUR  = Number::Four  as u32;
        const FIVE  = Number::Five  as u32;
        const ALL_NUMBERS = Self::ONE.bits | Self::TWO.bits | Self::THREE.bits | Self::FOUR.bits | Self::FIVE.bits;
    }
}

impl From<Color> for ColorKnowledge {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => ColorKnowledge::RED,
            Color::White => ColorKnowledge::WHITE,
            Color::Blue => ColorKnowledge::BLUE,
            Color::Green => ColorKnowledge::GREEN,
            Color::Yellow => ColorKnowledge::YELLOW,
        }
    }
}

impl From<Number> for NumberKnowledge {
    fn from(color: Number) -> Self {
        match color {
            Number::One => NumberKnowledge::ONE,
            Number::Two => NumberKnowledge::TWO,
            Number::Three => NumberKnowledge::THREE,
            Number::Four => NumberKnowledge::FOUR,
            Number::Five => NumberKnowledge::FIVE,
        }
    }
}

// The best way to keep knowledge about a card is to keep track of what you *don't* know about the
// card. Much easier to keep track of.
#[derive(Default)]
pub struct CardKnowledge {
    pub not_these_colors: ColorKnowledge,
    pub not_these_numbers: NumberKnowledge,
}

impl CardKnowledge {
    pub fn new() -> Self {
        CardKnowledge {
            not_these_colors: Default::default(),
            not_these_numbers: Default::default(),
        }
    }
}

pub fn apply_color_knowledge(
    ck: CardKnowledge,
    color: Color,
) -> Result<CardKnowledge, HanabiError> {
    let new_colors = ck.not_these_colors | color.into();

    if new_colors == ColorKnowledge::ALL_COLORS {
        Err(HanabiError::LogicError(
            "Impossible for a card to not be every color".to_string(),
        ))
    } else {
        Ok(CardKnowledge {
            not_these_colors: new_colors,
            not_these_numbers: ck.not_these_numbers,
        })
    }
}

pub fn apply_number_knowledge(
    ck: CardKnowledge,
    number: Number,
) -> Result<CardKnowledge, HanabiError> {
    let new_numbers = ck.not_these_numbers | number.into();

    if new_numbers == NumberKnowledge::ALL_NUMBERS {
        Err(HanabiError::LogicError(
            "Impossible for a card to not be every number".to_string(),
        ))
    } else {
        Ok(CardKnowledge {
            not_these_colors: ck.not_these_colors,
            not_these_numbers: new_numbers,
        })
    }
}
