extern crate lazy_static;

use std::collections::{HashMap, VecDeque};

use card::*;

pub const MAX_HINTS : u8 = 8;
pub const NUM_BOMBS : u8 = 3;

lazy_static! {
    pub static ref CARD_FREQUENCIES: HashMap<Number, u8> = {
        let mut m = HashMap::new();
        m.insert(Number::One,   3);
        m.insert(Number::Two,   2);
        m.insert(Number::Three, 2);
        m.insert(Number::Four,  2);
        m.insert(Number::Five,  1);
        m
    };
}

lazy_static! {
    pub static ref NUMS_BELOW: HashMap<Number, Option<Number>> = {
        let mut m = HashMap::new();
        m.insert(Number::One,   None);
        m.insert(Number::Two,   Some(Number::One));
        m.insert(Number::Three, Some(Number::Two));
        m.insert(Number::Four,  Some(Number::Three));
        m.insert(Number::Five,  Some(Number::Four));
        m
    };
}

lazy_static! {
    pub static ref VEC_NORMAL_COLORS: Vec<NormalColor> = {
        let mut v = Vec::new();
        v.push(NormalColor::Red);
        v.push(NormalColor::White);
        v.push(NormalColor::Blue);
        v.push(NormalColor::Yellow);
        v.push(NormalColor::Green);
        v
    };
}

lazy_static! {
    pub static ref VEC_NUMBERS: Vec<Number> = {
        let mut v = Vec::new();
        v.push(Number::One);
        v.push(Number::Two);
        v.push(Number::Three);
        v.push(Number::Four);
        v.push(Number::Five);
        v
    };
}

