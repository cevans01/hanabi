use crate::card::{Card, Color, Number};
use crate::player::PubID;

#[derive(Debug)]
pub enum Hint {
    ColorHint(Color),
    NumberHint(Number),
}

pub type HintForPlayer = (PubID, Hint);

#[derive(Debug)]
pub enum HanabiMove {
    Play(Card),
    Discard(Card),
    Hint(HintForPlayer),
}
