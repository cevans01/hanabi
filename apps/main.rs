#![allow(dead_code)]
use hanabi::game::*;
use hanabi::rules::*;
use hanabi::errors::*;

pub fn play() -> Result<(), HanabiError> {
    let deck = generate_normal_deck();
    let _g = Game::new(3, deck)?;

    //println!("{:?}", g.player_hands[0]);

    //comms::init_player_interfaces();

    //comms::send_players_initial_state(&g);

    //while !g.finished() {
    //comms::get_move()
    //}

    Ok(())
}

pub fn main() -> Result<(), HanabiError> {
    play()?;

    Ok(())
}
