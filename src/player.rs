//use std::fmt::Debug;
use crate::card::{Card, CardKnowledge, CardView, Color, Number};
use crate::rules::{MAX_PLAYERS, MIN_PLAYERS};

use crate::errors::HanabiError;

fn generate_uid() -> UID {
    rand::random::<u64>()
}

//#[derive(Debug)]
pub type UID = u64;

//#[derive(Debug)]
pub type PubID = u8;

#[derive(Eq, PartialEq)]
pub struct Player {
    pub public_id: PubID,
    pub uid: UID,

    hand: Vec<(Card, CardKnowledge)>,
    //cohorts: Vec<(PubID, CardKnowledge)>,
}

impl Player {
    pub fn new(public_id: u8, uid: u64) -> Player {
        Player {
            public_id,
            uid,
            hand: Vec::new(),
            //cohorts: Vec::new(),
        }
    }

    pub fn push_card(&mut self, card: Card) {
        self.hand.push((card, CardKnowledge::new()))
    }

    // TODO: re-implement Index trait?
    pub fn hand_at(&self, idx: usize) -> &(Card, CardKnowledge) {
        &self.hand[idx]
    }

    pub fn remove_card(&mut self, idx: usize) -> (Card, CardKnowledge) {
        self.hand.remove(idx)
    }

    pub fn hand_view(&self) -> Vec<CardView> {
        self.hand.iter().map(|(card, _)| card.view()).collect()
    }

    pub fn hand_len(&self) -> usize {
        self.hand.len()
    }

    pub fn any_of_color(&self, color: &Color) -> bool {
        self.hand.iter().any(|(x, _)| x.color() == *color)
    }

    pub fn any_of_number(&self, number: &Number) -> bool {
        self.hand.iter().any(|(x, _)| x.number() == *number)
    }

    pub fn get_knowledge(&self) -> Vec<CardKnowledge> {
        self.hand
            .iter()
            .map(|(_, knowledge)| knowledge.clone())
            .collect()
    }
}

pub fn get_public_id(players: &[Player], uid: UID) -> Result<PubID, HanabiError> {
    players
        .iter()
        .find(|p| p.uid == uid)
        .map(|p| p.public_id)
        .ok_or_else(|| HanabiError::InvalidMove("That uid doesn't exist".to_string()))
}

pub fn get_id(players: &[Player], pub_id: PubID) -> Result<UID, HanabiError> {
    players
        .iter()
        .find(|p| p.public_id == pub_id)
        .map(|p| p.uid)
        .ok_or_else(|| HanabiError::InvalidMove("That PubID doesn't exist".to_string()))
}

/**
 * @brief Create players, initialized with IDs
 */
pub fn generate_players(num_players: usize) -> Vec<Player> {
    assert!(num_players as u8 <= MAX_PLAYERS && num_players as u8 >= MIN_PLAYERS);

    let mut players = Vec::new();

    // Create the players
    for public_id in 0..num_players {
        let uid = generate_uid();
        /*
        let cohorts: Vec<(PubID, CardKnowledge)> = all_public_ids
            .iter()
            .filter(|&p| *p != public_id as u8)
            .cloned()
            .map(|pub_id| (pub_id, CardKnowledge::new()))
            .collect();
        */
        let new_player = Player {
            public_id: public_id as u8,
            uid,
            hand: Vec::new(),
            //cohorts,
        };
        players.push(new_player);
    }
    players
}
