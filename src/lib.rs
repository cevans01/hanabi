#![allow(dead_code)]

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
