use std::collections::VecDeque;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, CardKnowledge, CardView};
use crate::errors::HanabiError;
use crate::moves::{HanabiMove, Hint, HintForPlayer};
use crate::player::{generate_players, get_public_id, Player, PubID, UID};
use crate::rules::{number_below, GameResultState, MAX_HINTS, NUM_BOMBS};

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
//#[derive(Debug)]
pub struct Game {
    // Stacks of cards
    deck: VecDeque<Card>,
    pub discard: Vec<Card>,
    pub board: Vec<Card>,

    // Players and Player's cards
    players: Vec<Player>,
    active_player: PubID,

    // Counters
    pub hints: u8,
    pub bombs: u8,
    //pub turn_number: usize,
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
        let mut game = Game {
            // SUUUPER annoying that it can't create the deck itself and must be passed in. TODO
            // FIX THIS
            deck,
            discard: Vec::new(),
            board: Vec::new(),
            players: generate_players(num_players),
            active_player: 0,
            hints: MAX_HINTS,
            bombs: NUM_BOMBS,
            //turn_number: 0,
            turns_since_last_pickup: None,
        };

        game.deal_cards();

        assert_eq!(num_players, game.players.len());

        game
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
            for p in &mut self.players {
                let c = self
                    .deck
                    .pop_front()
                    .expect("deck doesn't have enough cards");
                p.push_card(c);
            }
        }
    }

    /**
     * @brief Get a look at another player's hand-knowledge.
     *
     * @return Vec<CardKnowledge> of the player's hand
     */
    pub fn get_player_knowledge(
        &self,
        target_player_pubid: PubID,
    ) -> Result<Vec<CardKnowledge>, HanabiError> {
        self.players
            .iter()
            .find(|p| target_player_pubid == p.public_id)
            .map(|p| p.get_knowledge())
            .ok_or_else(|| HanabiError::InvalidMove("Unknown cohort player ID".to_string()))
    }

    /**
     * @brief Get a look at another player's hand. You must submit your own player UID here, and
     * the public ID of the player you are requesting the cards for. This is to prevent players
     * requesting their own hands, which is not legal.
     *
     * @return If you are allowed to request the player's hand, a Vec<Card> for that player
     */
    pub fn get_player_cards(
        &self,
        requesting_player_uid: UID,
        target_player_pubid: PubID,
    ) -> Result<Vec<CardView>, HanabiError> {
        // Get cohorts for this player
        // Assert that the requesting player is one of the players
        let requesting_player = &self
            .players
            .iter()
            .find(|p| p.uid == requesting_player_uid)
            .ok_or_else(|| {
                HanabiError::InvalidMove(
                    "You must provide a valid UID in order to request player cards".to_string(),
                )
            })?;

        let target_player = &self
            .players
            .iter()
            .find(|p| p.public_id == target_player_pubid)
            .ok_or_else(|| {
                HanabiError::InvalidMove(
                    "The public ID given does not correspond to any player".to_string(),
                )
            })?;

        if target_player.public_id == requesting_player.public_id {
            return Err(HanabiError::InvalidMove(
                "You can't request to see your own cards".to_string(),
            ));
        }

        Ok(target_player.hand_view())
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
                self.hints != MAX_HINTS && idx < &self.players[pub_id as usize].hand_len()
                //self.hints != MAX_HINTS && idx < &self.player_hands[pub_id as usize].len()
            }
            //HanabiMove::Play(idx) => idx < &self.player_hands[pub_id as usize].len(),
            HanabiMove::Play(idx) => idx < &self.players[pub_id as usize].hand_len(),
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

        let target_player = &self.players[*target_player_id as usize];
        Ok(match hint_type {
            Hint::ColorHint(color) => target_player.any_of_color(&color),
            Hint::NumberHint(number) => target_player.any_of_number(&number),
        })
    }

    /**
     * @brief Checks to see if playing the given card is a legal play according to the rules.
     */
    fn card_playable(&self, card: &Card) -> bool {
        let move_color = &card.color();
        let highest_current_number = self
            .board
            .iter()
            .filter(|c| &c.color() == move_color)
            .map(|c| c.number())
            .max();

        // One condition must be satisfied to be playable:
        //  1.) The highest number of this suit must be the number just below this card (if any)

        number_below(&card.number()) == highest_current_number
    }

    fn score(&self) -> usize {
        self.board.len()
    }

    pub fn finished(&self) -> GameResultState {
        // You win -- got 25
        if 25 == self.board.len() {
            return GameResultState::Finished(25);
        }
        // You lose --
        if 0 == self.bombs {
            return GameResultState::Finished(self.score());
        }
        // You ran out of turns
        if Some(self.players.len()) == self.turns_since_last_pickup {
            return GameResultState::Finished(self.score());
        }

        // TODO: additional condition
        // there's no way to get any more points given the cards on the board and what is in the
        // discard pile
        // if self.no_way_to_win() {
        //     return GameResultState::Lose;
        // }

        GameResultState::InProgress
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

        if let GameResultState::Finished(_) = self.finished() {
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
                let p = &mut self.players[requester_pub_id as usize];
                if idx >= p.hand_len() {
                    return Err(HanabiError::InvalidMove(format!(
                        "Cannot discard with idx = '{}' which is out of range of hand size = '{}'",
                        idx,
                        p.hand_len()
                    )));
                }

                let (removed_card, _) = p.remove_card(idx);

                // Discard and get a hint back
                self.discard.push(removed_card);

                self.hints += 1;

                // Pickup another card
                let new_card = self.deck.pop_front().expect("Deck somehow is empty");
                p.push_card(new_card);
            }
            HanabiMove::Play(idx) => {
                // Remove
                let (removed_card, _) = {
                    let p = &mut self.players[requester_pub_id as usize];
                    if idx >= p.hand_len() {
                        return Err(HanabiError::InvalidMove(format!(
                            "Cannot play with idx = '{}' which is out of range of hand size = '{}'",
                            idx,
                            p.hand_len()
                        )));
                    }

                    p.remove_card(idx)
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
                    let p = &mut self.players[requester_pub_id as usize];
                    let new_card = self.deck.pop_front().expect("Deck somehow is empty");
                    p.push_card(new_card);
                }
            }
        }

        Ok(())
    }
}
