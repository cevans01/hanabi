use crate::card::{Color, Number};
use crate::player::PubID;

#[derive(Debug)]
pub enum Hint {
    ColorHint(Color),
    NumberHint(Number),
}

pub type HintForPlayer = (PubID, Hint);

#[derive(Debug)]
pub enum HanabiMove {
    Play(usize),    // usize is index in hand of which card to play
    Discard(usize), // usize is index in hand of which card to discard
    Hint(HintForPlayer),
}
