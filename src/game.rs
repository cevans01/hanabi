extern crate static_assertions;
extern crate lazy_static;
extern crate rand;

use self::rand::{thread_rng, Rng};
use std::fmt::Debug;
//use static_assertions::{assert_impl};

use std::collections::{VecDeque};

use player::*;
use hanabi_move::*;
use constants::*;
use card::*;
use hanabi_err::*;


/*--------------*/
/*   Helpers    */
/*--------------*/

pub fn generate_normal_deck() -> VecDeque<NormalCard>
{
    let mut deck = VecDeque::new();
    for col in VEC_NORMAL_COLORS.iter() {
        for num in VEC_NUMBERS.iter() {
            for _num_card in 0..CARD_FREQUENCIES[&*num] {
                deck.push_front(NormalCard::new(*col, *num));
            }
        }
    }
    deck
}

pub fn generate_extended_deck() -> Vec<ExtendedCard>
{
    // TODO
    Vec::new()
}

/*
/**
 * @brief Create a Non Shuffled Deck
 */
pub fn generate_deck<C: Card>() -> VecDeque<C>
    where <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{
    let color_iter = <C::ColorType as Color>::new().get_iter();
    let number_iter = NumberTypeIter::new();

    let rv = VecDeque::new();

    for col in color_iter {
        for num in number_iter {

            //let foo = C::new(col, num);

            /* TODO: XXX: TODO: Compile error here when used inside Game::new(), what the HECK is
             * going on???
             * let foo = <C as Card>::new(col, num);
             */

            println!("col = {:?}, num = {:?}", col, num);
        }
    }
    rv
}
*/

/**
 * @brief Shuffle an existing deck
 */
fn shuffle_deck<C: Card>(deck : VecDeque<C>) -> VecDeque<C> {
    // TODO: copying out and back in sucks but it'll work for now
    let mut temp = Vec::from(deck);
    thread_rng().shuffle(&mut temp[..]);
    return VecDeque::from(temp);
}


/* Game */
#[derive(Debug)]
pub struct Game<C>
    where C: Card,
    <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{
    // Stacks of cards
    deck: VecDeque<C>,
    pub discard: Vec<C>,
    pub board: Vec<C>,

    // Players and Player's cards
    players : Vec<Player>,
    player_hands: Vec<Vec<C>>,
    active_player : PubID,

    // Counters
    pub hints : u8,
    pub bombs : u8,
    pub turn_number : usize,
    pub turns_since_last_pickup : Option<usize>,

}

impl<C> Game<C>
    where C: Card,
    <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{

    /**
     * @brief Create a new Game
     */
    pub fn new(num_players: usize, mut deck: VecDeque<C>) -> Game<C> {
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
            deck    : deck,
            discard : Vec::new(),
            board   : Vec::new(),
            players : generate_players(num_players),
            player_hands : Vec::new(),
            active_player : 0,
            hints : MAX_HINTS,
            bombs : NUM_BOMBS,
            turn_number : 0,
            turns_since_last_pickup : None,
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
                let c = self.deck.pop_front().expect("deck doesn't have enough cards");
                p.push( c );
            }
        }
    }

    // Move related functions
    /**
     * @brief Checks to see if the move follows the rules of hanabi
     *      1.) If you don't have any hits, you can't hint
     *      2.) If you have max hints (10) then you can't discard
     *      [... 2.) If the card isn't in your hand you can't play it ??? ...]
     */
    fn legal_move(&self, mv : HanabiMove<C>) -> bool {
        match mv {
            // You have hints to give and it's a legal hint
            HanabiMove::Hint(hint) => self.hints != 0 && self.legal_hint(&hint),
            HanabiMove::Discard(_) => self.hints != MAX_HINTS,
            // No way for a play to be illegal
            _ => true,
        }
    }

    /**
     * @brief Checks to see if a hint is legal in the game
     *
     *      1.) You can't give a hint for a number to a player if the player doesn't have any cards
     *        of that number
     *      2.) You can't give a hint for a color to a player if the player doesn't have any cards
     *        of that color
     */
    fn legal_hint(&self, hint : &HintForPlayer<C>) -> bool {
        let (target_player_id, hint_type) = hint;
        let target_player_hand = &self.player_hands[*target_player_id as usize];
        match hint_type {
            HintType::ColorHint(color) => {
                target_player_hand.iter().find(|x| x.get_color() == *color).is_some()
            },
            HintType::NumberHint(number) => {
                target_player_hand.iter().find(|x| x.get_number() == *number).is_some()
            },
        }
    }

    /**
     * @brief Checks to see if the card being played can be played on the current board. Usually,
     * if this function returns true, the card will be "played". If not, it will be moved to the
     * discard pile.
     */
    fn valid_play(&self, card : C) -> bool {
        let move_color = card.get_color();
        let highest_num_of_color : Option<Number> = self.board.iter()
                                    .filter(|c| c.get_color() == move_color)
                                    .map(|c| c.get_real_number())
                                    .max();

        // Condition which must be satisfied to be playable:
        //  1.) The highest number of this suit must be the number just below this card (if any)
        if highest_num_of_color != NUMS_BELOW[&card.get_real_number()] {
            return false
        }
        return true;
    }

    /**
     * @brief Submit a move to the game, ending your turn. The game will complete the turn and then
     * deal the player a new card from the deck.
     */
    //fn submit_move<M : Move>(&mut self, mv : M) { }

    /**
     * @brief Check to see if the game is done.
     */
    fn finished(&self) -> bool {
        // You win
        if 25 == self.board.len() {
            return true;
        }
        // You lose
        if 0 == self.bombs {
            return true;
        }

        match self.turns_since_last_pickup {
            None => false,
            Some(turns) if turns >= self.players.len() => true,
            Some(_) => false
        }
    }
    
    // Player related functions
    /**
     * @brief Get a look at another player's hand. You must submit your own player UID here, and
     * the public ID of the player you are requesting the cards for. This is to prevent players
     * requesting their own hands, which is not legal.
     *
     * @return If you are allowed to request the player's hand, a Vec<Card> for that player
     */
    pub fn get_player_cards(&self, requesting_player_uid : UID, target_player_pubid : PubID) -> 
            Result<Vec<CardView<<C as Card>::ColorType>>, HanabiErr>
    {
        // Validate you are allowed to request this
        if let Some(player) = self.players.iter().find(|p| p.uid == requesting_player_uid) {
            if let Some(cohort_pub_id) = player.cohorts.iter().find(|&c| target_player_pubid == *c) {
                return Ok(Vec::new());
            }
            else{
                return Err(HanabiErr::RequestNotAllowed);
            }
        }
        else {
            return Err(HanabiErr::RequestNotAllowed);
        }

    }

//    /**
//     * @brief Get a Vec of CardView representing the knowledge that another player has of their own
//     * hand. It is valid to call this against your own Player::public_id
//     *
//     * @return Knowlege that the player has of their hand
//     */
//    //pub fn request_player_knowledge(&self, pubid : PubID) -> Vec<CardView> { }
}



/*--------------*/
/*    TESTS     */
/*--------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_play() {
        let deck = generate_normal_deck();
        let mut g = Game::new(3, deck);
        g.board.push(NormalCard::new(NormalColor::Blue, Number::One));
        g.board.push(NormalCard::new(NormalColor::Blue, Number::Two));
        g.board.push(NormalCard::new(NormalColor::Blue, Number::Three));

        g.board.push(NormalCard::new(NormalColor::Red, Number::One));

        assert_eq!( true,  g.valid_play(NormalCard::new(NormalColor::Blue, Number::Four)) );
        assert_eq!( false, g.valid_play(NormalCard::new(NormalColor::Blue, Number::Five)) );
        assert_eq!( false, g.valid_play(NormalCard::new(NormalColor::Blue, Number::Three)) );

        assert_eq!( true,  g.valid_play(NormalCard::new(NormalColor::Red, Number::Two)) );

        assert_eq!( true,  g.valid_play(NormalCard::new(NormalColor::Green, Number::One)) );
        assert_eq!( false, g.valid_play(NormalCard::new(NormalColor::Green, Number::Two)) );
    }

    #[test]
    fn test_generate_players() {
        let players = generate_players(3);

        println!("players = {:?}", players);
    }

} /* test */
