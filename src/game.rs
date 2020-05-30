use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::errors::HanabiError;
use crate::moves::{HanabiMove, Hint, HintForPlayer};
use crate::player::{generate_players, Player, PubID, UID};
use crate::rules::{MAX_HINTS, NUM_BOMBS};

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
    //pub turns_since_last_pickup : Option<usize>,
}

impl Game {
    /**
     * @brief Create a new Game
     */
    pub fn new(num_players: usize, mut deck: VecDeque<Card>) -> Self {
        // TODO: should be able to generate_deck
        //let mut deck = generate_deck::<C>();
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
            //turns_since_last_pickup : None,
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
     *      1.) If you don't have any hits, you can't hint
     *      2.) If you have max hints (10) then you can't discard
     *      [... 2.) If the card isn't in your hand you can't play it ??? ...]
     */
    fn legal_move(&self, mv: HanabiMove) -> bool {
        match mv {
            // You have hints to give and it's a legal hint
            HanabiMove::Hint(hint) => self.hints != 0 && self.legal_hint(&hint),
            HanabiMove::Discard(_) => self.hints != MAX_HINTS,
            // No way for a play to be illegal
            _ => true,
        }
    }

    /**
     * @brief Checks to see if the card being played can be played on the current board. Usually,
     * if this function returns true, the card will be "played". If not, it will be moved to the
     * discard pile.
     */
    fn card_playable(&self, _card: Card) -> bool {
        todo!()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn play_move(
        &mut self,
        _play: HanabiMove,
        _requesting_player_uid: UID,
    ) -> Result<(), HanabiError> {
        todo!()
    }

    /**
     * @brief Checks to see if a hint is legal in the game
     *
     *      1.) You can't give a hint for a number to a player if the player doesn't have any cards
     *        of that number
     *      2.) You can't give a hint for a color to a player if the player doesn't have any cards
     *        of that color
     */
    fn legal_hint(&self, hint: &HintForPlayer) -> bool {
        let (target_player_id, hint_type) = hint;
        let target_player_hand = &self.player_hands[*target_player_id as usize];
        match hint_type {
            Hint::ColorHint(color) => target_player_hand
                .iter()
                .any(|x| x.color == *color),
            Hint::NumberHint(number) => target_player_hand
                .iter()
                .any(|x| x.number == *number)
        }
    }
}
