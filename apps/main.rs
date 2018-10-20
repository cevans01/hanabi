
#[macro_use]
extern crate lazy_static;

extern crate rand;

extern crate hanabi;

use hanabi::hanabi_err::*;
use hanabi::game::*;
use hanabi::card::*;

pub fn play() {

    let deck = generate_normal_deck();
    let g = Game::<NormalCard>::new(3, deck);

    println!("{:?}", g.player_hands[0]);

    //comms::init_player_interfaces();

    //comms::send_players_initial_state(&g);
    
    //while !g.finished() {
        //comms::get_move()
    //}
}

pub fn main() {
    play();
}
