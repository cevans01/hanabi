#![allow(dead_code)]

// TODO:
//      1. get rid of to_string() for errors?
//      2. Make Player own the cards?
//      3.

#[macro_use]
extern crate bitflags;

extern crate thiserror;

extern crate lazy_static;

pub mod card;
pub mod errors;
pub mod rules;

pub mod game;
pub mod moves;
pub mod player;
