
#[macro_use]
extern crate lazy_static;

extern crate rand;

extern crate hanabi;

use hanabi::hanabi_err::*;
use hanabi::game::*;
use hanabi::card::*;

pub fn play() {

    let deck = generate_normal_deck();
    let _g = Game::<NormalCard>::new(3, deck);

    let _x = HanabiErr::MoveNotAllowed;
}

pub fn main() {
    play();
}
