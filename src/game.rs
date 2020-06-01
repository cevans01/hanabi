use std::collections::VecDeque;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::errors::HanabiError;
use crate::moves::{HanabiMove, Hint, HintForPlayer};
use crate::player::{generate_players, get_public_id, Player, PubID, UID};
use crate::rules::{number_below, MAX_HINTS, NUM_BOMBS};

/**
 * @brief Shuffle an existing deck
 */
fn shuffle_deck(deck: VecDeque<Card>) -> VecDeque<Card> {
    // TODO: copying out and back in sucks but it'll work for now
    let mut temp = Vec::from(deck);
    temp.shuffle(&mut thread_rng());

    VecDeque::from(temp)
}

/* Game */
#[derive(Debug)]
pub struct Game {
    // Stacks of cards
    deck: VecDeque<Card>,
    pub discard: Vec<Card>,
    pub board: Vec<Card>,

    // Players and Player's cards
    players: Vec<Player>,
    player_hands: Vec<Vec<Card>>,
    active_player: PubID,

    // Counters
    pub hints: u8,
    pub bombs: u8,
    pub turn_number: usize,
    // TODO: consider removing this since it can likely be calculated from the number of cards in
    // the players' hands.
    pub turns_since_last_pickup: Option<usize>,
}

impl Game {
    /**
     * @brief Create a new Game
     */
    pub fn new(num_players: usize, mut deck: VecDeque<Card>) -> Self {
        // TODO: should be able to generate_deck
        let before_len = deck.len();
        deck = shuffle_deck(deck);
        assert!(deck.len() == before_len);
        assert!(num_players < 6 && num_players > 1);
        println!("deck.len() = {:?}", deck.len());
        let mut g = Game {
            // SUUUPER annoying that it can't create the deck itself and must be passed in. TODO
            // FIX THIS
            deck,
            discard: Vec::new(),
            board: Vec::new(),
            players: generate_players(num_players),
            player_hands: Vec::new(),
            active_player: 0,
            hints: MAX_HINTS,
            bombs: NUM_BOMBS,
            turn_number: 0,
            turns_since_last_pickup: None,
        };

        for _ in 0..num_players {
            g.player_hands.push(Vec::new());
        }

        g.deal_cards();

        assert_eq!(num_players, g.player_hands.len());
        assert_eq!(num_players, g.players.len());
        return g;
    }

    /**
     * @brief Deal cards from the Deck into Player's hands
     *  According to the rules:
     *      Deal a hand of 5 cards to each with 2 or 3 players
     *      Deal a hand of 4 cards to each with 4 or 5 players
     */
    fn deal_cards(&mut self) {
        let cards_to_deal = match self.players.len() {
            2 | 3 => 5,
            4 | 5 => 4,
            _ => unreachable!(),
        };
        println!("cards_to_deal = {:?}", cards_to_deal);

        for _ in 0..cards_to_deal {
            for p in &mut self.player_hands {
                let c = self
                    .deck
                    .pop_front()
                    .expect("deck doesn't have enough cards");
                p.push(c);
            }
        }
    }

    /**
     * @brief Get a look at another player's hand. You must submit your own player UID here, and
     * the public ID of the player you are requesting the cards for. This is to prevent players
     * requesting their own hands, which is not legal.
     *
     * @return If you are allowed to request the player's hand, a Vec<Card> for that player
     */
    fn get_player_cards(
        &self,
        requesting_player_uid: UID,
        target_player_pubid: PubID,
    ) -> Result<Vec<Card>, HanabiError> {
        // Validate you are allowed to request this
        if let Some(player) = self.players.iter().find(|p| p.uid == requesting_player_uid) {
            if let Some(_cohort_pub_id) = player.cohorts.iter().find(|&c| target_player_pubid == *c)
            {
                Ok(Vec::new())
            } else {
                Err(HanabiError::InvalidMove(
                    "Unknown cohort player ID".to_string(),
                ))
            }
        } else {
            Err(HanabiError::InvalidMove(
                "Cannot request to see your own cards".to_string(),
            ))
        }
    }

    /**
     * @brief Checks to see if the move follows the rules of hanabi
     *      1.) If there are no hints left, you can't hint
     *      2.) If all hints are available, then you can't discard
     *      3.) The index of the card to play or discard must be valid
     */
    fn legal_move(&self, mv: &HanabiMove, pub_id: PubID) -> Result<bool, HanabiError> {
        if pub_id >= self.players.len().try_into()? {
            return Err(HanabiError::InvalidMove(
                "PubID is out of range for num players".to_string(),
            ));
        }

        Ok(match mv {
            // You have hints to give and it's a legal hint
            HanabiMove::Hint(hint) => self.hints != 0 && self.legal_hint(&hint)?,
            HanabiMove::Discard(idx) => {
                self.hints != MAX_HINTS && idx < &self.player_hands[pub_id as usize].len()
            }
            HanabiMove::Play(idx) => idx < &self.player_hands[pub_id as usize].len(),
        })
    }

    /**
     * @brief Checks to see if a hint is legal in the game
     *
     *      1.) You can't give a hint for a number to a player if the player doesn't have any cards
     *        of that number
     *      2.) You can't give a hint for a color to a player if the player doesn't have any cards
     *        of that color
     */
    fn legal_hint(&self, hint: &HintForPlayer) -> Result<bool, HanabiError> {
        let (target_player_id, hint_type) = hint;

        if target_player_id >= &(self.players.len() as PubID) {
            return Err(HanabiError::InvalidMove(
                "PubID is out of range for num players".to_string(),
            ));
        }

        let target_player_hand = &self.player_hands[*target_player_id as usize];
        Ok(match hint_type {
            Hint::ColorHint(color) => target_player_hand.iter().any(|x| x.color == *color),
            Hint::NumberHint(number) => target_player_hand.iter().any(|x| x.number == *number),
        })
    }

    /**
     * @brief Checks to see if playing the given card is a legal play according to the rules.
     */
    fn card_playable(&self, card: &Card) -> bool {
        let move_color = &card.color;
        let highest_current_number = self
            .board
            .iter()
            .filter(|c| &c.color == move_color)
            .map(|c| &c.number)
            .max();

        // One condition must be satisfied to be playable:
        //  1.) The highest number of this suit must be the number just below this card (if any)

        number_below(&card.number).as_ref() == highest_current_number
    }

    fn finished(&self) -> bool {
        // You win
        if 25 == self.board.len() {
            return true;
        }
        // You lose
        if 0 == self.bombs {
            return true;
        }
        // You ran out of turns
        if Some(self.players.len()) == self.turns_since_last_pickup {
            return true;
        }

        false
    }

    pub fn is_players_turn(&self, player_id: PubID) -> bool {
        player_id == self.active_player
    }

    pub fn play_move(
        &mut self,
        play: HanabiMove,
        requesting_player_uid: UID,
    ) -> Result<(), HanabiError> {
        // Check to make sure it is this player's turn
        let requester_pub_id = get_public_id(&self.players, requesting_player_uid)?;

        if self.finished() {
            return Err(HanabiError::GameFinished);
        }

        if self.active_player != requester_pub_id {
            return Err(HanabiError::InvalidMove("It's not your turn!".to_string()));
        }

        // Check to make sure this is a legal move
        self.legal_move(&play, requester_pub_id)?;

        // If the card is playable, play it
        // Else, this is a bomb and move it to the discard
        match play {
            HanabiMove::Hint((_pub_id, _hint)) => {}
            HanabiMove::Discard(idx) => {
                // Remove
                let hand = &mut self.player_hands[requester_pub_id as usize];
                if idx >= hand.len() {
                    return Err(HanabiError::InvalidMove(format!(
                        "Cannot discard with idx = '{}' which is out of range of hand size = '{}'",
                        idx,
                        hand.len()
                    )));
                }

                let removed_card = hand.remove(idx);

                // Discard and get a hint back
                self.discard.push(removed_card);

                self.hints += 1;

                // Pickup another card
                let new_card = self.deck.pop_front().expect("Deck somehow is empty");
                hand.push(new_card);
            }
            HanabiMove::Play(idx) => {
                // Remove
                let removed_card = {
                    let hand = &mut self.player_hands[requester_pub_id as usize];
                    if idx >= hand.len() {
                        return Err(HanabiError::InvalidMove(format!(
                            "Cannot play with idx = '{}' which is out of range of hand size = '{}'",
                            idx,
                            hand.len()
                        )));
                    }

                    hand.remove(idx)
                };

                // Play if playable, else discard
                if self.card_playable(&removed_card) {
                    self.board.push(removed_card);
                } else {
                    self.bombs += 1;
                    self.discard.push(removed_card);
                }

                // Pickup another card
                {
                    let hand = &mut self.player_hands[requester_pub_id as usize];
                    let new_card = self.deck.pop_front().expect("Deck somehow is empty");
                    hand.push(new_card);
                }
            }
        }

        Ok(())
    }
}
