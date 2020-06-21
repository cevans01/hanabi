//use std::fmt::Debug;
use crate::card::{
    not_this_color, not_this_number, this_color, this_number, Card, CardKnowledge, CardView, Color,
    Number,
};
use crate::moves::Hint;
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

    pub fn any_of_color(&self, color: Color) -> bool {
        self.hand.iter().any(|(x, _)| x.color() == color)
    }

    pub fn any_of_number(&self, number: Number) -> bool {
        self.hand.iter().any(|(x, _)| x.number() == number)
    }

    pub fn get_knowledge(&self) -> Vec<CardKnowledge> {
        self.hand
            .iter()
            .map(|(_, knowledge)| knowledge.clone())
            .collect()
    }

    pub fn give_hint(&mut self, hint: Hint) -> Result<(), HanabiError> {
        match hint {
            Hint::ColorHint(color) => {
                for (card, card_knowledge) in &mut self.hand {
                    if card.color() == color {
                        *card_knowledge = this_color(card_knowledge.clone(), color)?;
                    } else {
                        *card_knowledge = not_this_color(card_knowledge.clone(), color)?;
                    }
                }
            }
            Hint::NumberHint(number) => {
                for (card, card_knowledge) in &mut self.hand {
                    if card.number() == number {
                        *card_knowledge = this_number(card_knowledge.clone(), number)?;
                    } else {
                        *card_knowledge = not_this_number(card_knowledge.clone(), number)?;
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn get_public_id(players: &[Player], uid: UID) -> Result<PubID, HanabiError> {
    players
        .iter()
        .find(|p| p.uid == uid)
        .map(|p| p.public_id)
        .ok_or_else(|| HanabiError::InvalidMove("That uid doesn't exist".to_string()))
}

// TODO: might be able to remove this
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
        let new_player = Player {
            public_id: public_id as u8,
            uid,
            hand: Vec::new(),
        };
        players.push(new_player);
    }
    players
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{CardKnowledge, ColorKnowledge, NumberKnowledge};

    #[test]
    fn test_generate_players() {
        for num_players in 2..6 {
            let players = generate_players(num_players);
            assert!(players.len() == num_players);
        }
    }

    #[test]
    fn test_give_hint() {
        let mut player = Player {
            public_id: 0,
            uid: generate_uid(),
            hand: Vec::new(),
        };

        player.push_card(Card::new(Color::Red, Number::One));
        player.push_card(Card::new(Color::Red, Number::One));
        player.push_card(Card::new(Color::Red, Number::Two));
        player.push_card(Card::new(Color::White, Number::Five));
        player.push_card(Card::new(Color::Blue, Number::Two));

        // ------------------
        // First hint
        // ------------------
        player.give_hint(Hint::ColorHint(Color::Red)).unwrap();

        let hand_knowledge: Vec<CardKnowledge> = (0..(player.hand_len()))
            .map(|idx| player.hand_at(idx))
            .map(|(_, knowledge)| knowledge.clone())
            .collect();

        let expected_knowledge = vec![
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::default(),
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::default(),
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::default(),
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::default(),
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::default(),
            },
        ];
        assert_eq!(hand_knowledge, expected_knowledge);

        // ------------------
        // Second hint
        // ------------------
        player.give_hint(Hint::NumberHint(Number::Two)).unwrap();

        let hand_knowledge: Vec<CardKnowledge> = (0..(player.hand_len()))
            .map(|idx| player.hand_at(idx))
            .map(|(_, knowledge)| knowledge.clone())
            .collect();

        let expected_knowledge = vec![
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::TWO,
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::TWO,
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::ALL_COLORS ^ ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::ALL_NUMBERS ^ NumberKnowledge::TWO,
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::TWO,
            },
            CardKnowledge {
                not_these_colors: ColorKnowledge::RED,
                not_these_numbers: NumberKnowledge::ALL_NUMBERS ^ NumberKnowledge::TWO,
            },
        ];

        assert_eq!(hand_knowledge, expected_knowledge);
    }
}
