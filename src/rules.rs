use std::collections::VecDeque;

use crate::card::{generate_deck, Card, Number};
use crate::errors::HanabiError;

pub const MAX_HINTS: u8 = 8;
pub const NUM_BOMBS: u8 = 3;

pub const MAX_PLAYERS: u8 = 5;
pub const MIN_PLAYERS: u8 = 2;

pub enum GameResultState {
    Finished(usize),
    InProgress,
}

pub fn cards_to_deal(num_players: u8) -> Result<u8, HanabiError> {
    match num_players {
        2 | 3 => Ok(5),
        4 | 5 => Ok(4),
        _ => Err(HanabiError::LogicError(
            "Invalid number of players".to_string(),
        )),
    }
}

pub fn card_frequencies(num: &Number) -> u8 {
    match num {
        Number::One => 3,
        Number::Two => 2,
        Number::Three => 2,
        Number::Four => 2,
        Number::Five => 1,
    }
}

pub fn number_below(num: &Number) -> Option<Number> {
    match num {
        Number::One => None,
        Number::Two => Some(Number::One),
        Number::Three => Some(Number::Two),
        Number::Four => Some(Number::Three),
        Number::Five => Some(Number::Four),
    }
}

pub fn generate_normal_deck() -> VecDeque<Card> {
    generate_deck(|num| card_frequencies(&num))
}
