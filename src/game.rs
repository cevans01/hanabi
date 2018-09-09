
extern crate static_assertions;
extern crate lazy_static;
extern crate rand;

use rand::{thread_rng, Rng};
use std::fmt::Debug;
//use static_assertions::{assert_impl};

use std::collections::{HashMap, VecDeque};

const MAX_HINTS : u8 = 8;
const NUM_BOMBS : u8 = 3;

lazy_static! {
    static ref CARD_FREQUENCIES: HashMap<Number, u8> = {
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
    static ref NUMS_BELOW: HashMap<Number, Option<Number>> = {
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
    static ref VEC_NORMAL_COLORS: Vec<NormalColor> = {
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
    static ref VEC_NUMBERS: Vec<Number> = {
        let mut v = Vec::new();
        v.push(Number::One);
        v.push(Number::Two);
        v.push(Number::Three);
        v.push(Number::Four);
        v.push(Number::Five);
        v
    };
}

/*
mod hanabi_err;
use hanabi_err::*;
*/

/*------------*/
/* Game Types */
/*------------*/
pub trait Color : Clone + Copy + Debug {
    type IterType : Debug + Iterator;
    fn new() -> Self;
    fn get_iter(&self) -> Self::IterType;
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum NormalColor {
    Red,
    White,
    Blue,
    Yellow,
    Green,
}

#[derive(Debug)]
pub struct NormalColorTypeIter(pub Option<NormalColor>);

impl NormalColorTypeIter {
    fn new() -> NormalColorTypeIter {
        NormalColorTypeIter(Some(NormalColor::Red))
    }
}

impl<'a> From<&'a NormalColor> for NormalColorTypeIter {
    fn from(col : &NormalColor) -> NormalColorTypeIter {
        NormalColorTypeIter(Some(col.clone()))
    }
}

impl From<NormalColor> for NormalColorTypeIter {
    fn from(col : NormalColor) -> NormalColorTypeIter {
        NormalColorTypeIter(Some(col.clone()))
    }
}

impl Iterator for NormalColorTypeIter {
    type Item = NormalColor;

    fn next(&mut self) -> Option<Self::Item> {
        let rv = self.0.clone();
        self.0 = match self.0 {
            Some(NormalColor::Red)    => Some(NormalColor::White),
            Some(NormalColor::White)  => Some(NormalColor::Blue),
            Some(NormalColor::Blue)   => Some(NormalColor::Yellow),
            Some(NormalColor::Yellow) => Some(NormalColor::Green),
            Some(NormalColor::Green)  => None,
            None => None,
        };
        return rv;
    }
}


#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum ExtendedColor {
    Red,
    White,
    Blue,
    Yellow,
    Green,
    Rainbow,
}

#[derive(Debug)]
pub struct ExtendedColorTypeIter(pub Option<ExtendedColor>);

impl ExtendedColorTypeIter {
    fn new() -> ExtendedColorTypeIter {
        ExtendedColorTypeIter(Some(ExtendedColor::Red))
    }
}

impl<'a> From<&'a ExtendedColor> for ExtendedColorTypeIter {
    fn from(col : &ExtendedColor) -> ExtendedColorTypeIter {
        ExtendedColorTypeIter(Some(col.clone()))
    }
}

impl From<ExtendedColor> for ExtendedColorTypeIter {
    fn from(col : ExtendedColor) -> ExtendedColorTypeIter {
        ExtendedColorTypeIter(Some(col.clone()))
    }
}

impl Iterator for ExtendedColorTypeIter {
    type Item = ExtendedColor;

    fn next(&mut self) -> Option<Self::Item> {
        let rv = self.0.clone();
        self.0 = match self.0 {
            Some(ExtendedColor::Red)    => Some(ExtendedColor::White),
            Some(ExtendedColor::White)  => Some(ExtendedColor::Blue),
            Some(ExtendedColor::Blue)   => Some(ExtendedColor::Yellow),
            Some(ExtendedColor::Yellow) => Some(ExtendedColor::Green),
            Some(ExtendedColor::Green)  => Some(ExtendedColor::Rainbow),
            Some(ExtendedColor::Rainbow)  => None,
            None => None,
        };
        return rv;
    }
}

impl Color for NormalColor {
    type IterType = NormalColorTypeIter;
    fn new() -> NormalColor {
        NormalColor::Red
    }
    fn get_iter(&self) -> Self::IterType {
        NormalColorTypeIter::new()
    }
}

impl Color for ExtendedColor {
    type IterType = ExtendedColorTypeIter;
    fn new() -> ExtendedColor {
        ExtendedColor::Red
    }
    fn get_iter(&self) -> Self::IterType {
        ExtendedColorTypeIter::new()
    }
}


// TODO: consider impl `From` for u8
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone,Hash)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug,Clone,Copy)]
struct NumberTypeIter(pub Option<Number>);
impl NumberTypeIter {
    pub fn new() -> NumberTypeIter {
        NumberTypeIter(Some(Number::One))
    }
}
impl Iterator for NumberTypeIter {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let rv = self.0.clone();
        self.0  = match self.0 {
            Some(Number::One)   => Some(Number::Two),
            Some(Number::Two)   => Some(Number::Three),
            Some(Number::Three) => Some(Number::Four),
            Some(Number::Four)  => Some(Number::Five),
            Some(Number::Five)  => None,
            None => None,
        };
        return rv;
    }
}

/* ----------------------- */
/* ----- Card Types ------ */
/* ----------------------- */
pub trait Card : Debug {
    type ColorType : Debug + Color + PartialEq + Eq;
    type NumberType : Debug + PartialEq + Eq + Ord + PartialOrd;

    fn new(col : Self::ColorType, num : Self::NumberType) -> Self;
    fn get_color(&self) -> Self::ColorType;
    fn get_number(&self) -> Self::NumberType;

    // TODO: wtf why do I need this, why can't I use get_number()
    fn get_real_number(&self) -> Number;
}


#[derive(Debug)]
// DO NOT IMPL CLONE/COPY
pub struct HanabiCard<C> where
    C: Color
{
    pub color : C,
    pub number : Number,

    pub card_view : CardView<C>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/**
 * @brief Represents a view onto the Card. The option sentinel represents the added possibility
 * that the player doesn't know anything about this card. For instance, if a player has no
 * knowledge about a card then its value will be CardView{ None, None }
 */
pub struct CardView<C> where
    C: Color
{
    pub color: Option<C>,
    pub number: Option<Number>,
}

impl<C> CardView<C> where
    C: Color
{
    pub fn new() -> CardView<C> {
        CardView {color: None, number: None}
    }
}

pub type NormalCardView = CardView<NormalColor>;
pub type ExtendedCardView = CardView<ExtendedColor>;

pub type NormalCard = HanabiCard<NormalColor>;
pub type ExtendedCard = HanabiCard<ExtendedColor>;

impl Card for NormalCard {
    type ColorType = NormalColor;
    type NumberType = Number;

    fn new(col : Self::ColorType, num : Self::NumberType) -> NormalCard {
        NormalCard{color : col, number : num, card_view : NormalCardView::new() }
    }

    fn get_color(&self) -> Self::ColorType {
        self.color.clone()
    }

    fn get_number(&self) -> Self::NumberType {
        self.number.clone()
    }

    fn get_real_number(&self) -> Number {
        self.number.clone()
    }
}

impl Card for ExtendedCard {
    type ColorType = ExtendedColor;
    type NumberType = Number;

    fn new(col : Self::ColorType, num : Self::NumberType) -> ExtendedCard {
        ExtendedCard{color : col, number : num, card_view : ExtendedCardView::new() }
    }

    fn get_color(&self) -> Self::ColorType {
        self.color.clone()
    }

    fn get_number(&self) -> Self::NumberType {
        self.number.clone()
    }

    fn get_real_number(&self) -> Number {
        self.number.clone()
    }
}

//#[derive(Debug)]
//pub struct UID(u64);
pub type UID = u64;

//#[derive(Debug)]
//pub struct PubID(u8);
pub type PubID = u8;

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub public_id: PubID,

    pub uid: UID,
    pub cohorts: Vec<PubID>,
}

impl Player {
    pub fn new(public_id: u8, uid: u64) -> Player {
        Player { public_id : public_id, uid: uid, cohorts : Vec::new()  }
    }
}


/*------------*/
/* Move Types */
/*------------*/

// TODO: this needs consideration
#[derive(Debug)]
pub enum HintType<C> where
    C: Card
{
    ColorHint(C::ColorType),
    NumberHint(C::NumberType),
}

type HintForPlayer<C> = (PubID, HintType<C>);

//pub trait Move : Debug {
//    type CardType : Card + Debug;
//}

#[derive(Debug)]
pub enum HanabiMove<C: Card> {
    Play(C),
    Discard(C),
    Hint(HintForPlayer<C>),
}

/*
impl Move for HanabiMove<NormalCard> {
    type CardType = NormalCard;
}

impl Move for HanabiMove<ExtendedCard> {
    type CardType = ExtendedCard;
}
*/

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

            /* TODO: XXX: TODO: Compile error here, what the HECK is going on???
             * let foo = <C as Card>::new(col, num);
             */

            println!("col = {:?}, num = {:?}", col, num);
        }
    }
    rv
}
*/

fn generate_uid() -> UID {
    rand::random::<u64>()
}

/**
 * @brief Create players, initialized with IDs
 */
pub fn generate_players(num_players : usize) -> Vec<Player> {
    assert!(num_players < 6 && num_players > 1);

    let mut players = Vec::new();
    let all_public_ids : Vec<u8> = (0..num_players as u8).collect();

    // Create the players
    for public_id in 0..num_players {
        let uid = generate_uid();
        let other_public_ids : Vec<u8> = all_public_ids.iter().filter(|&p| *p != public_id as u8).cloned().collect();
        let new_player = Player{ public_id : public_id as u8, uid: uid, cohorts : other_public_ids };
        players.push(new_player);
    }
    players
}

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
            //self.player_hands.iter()
            for p in &mut self.player_hands {
                let c = self.deck.pop_front().expect("deck doesn't have enough cards");
                p.push( c );
                //p.push( self.deck.pop_front().expect("deck doesn't have enough cards") );

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

        // Two conditions must be satisfied to be playable:
        //  1.) The card can't already be played on the board
        //  2.) The card just below this card must already be on the table
        let move_number : Number = card.get_real_number();
        if highest_num_of_color == Some(card.get_real_number()) {
            return false;
        }
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
    
//    // Player related functions
//    /**
//     * @brief Get a look at another player's hand. You must submit your own player UID here, and
//     * the public ID of the player you are requesting the cards for. This is to prevent players
//     * requesting their own hands, which is not legal.
//     *
//     * @return If you are allowed to request the player's hand, a Vec<Card> for that player
//     */
//    //pub fn get_player_cards(&self, requesting_player_uid : UID, target_player_pubid : PubID) -> Result<Vec<CardView>, HanabiErr> { }
//
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
    fn number_eq() {
        assert_eq!(Number::One, Number::One);
        assert_eq!(Number::Two, Number::Two);
        assert_eq!(Number::Three, Number::Three);
        assert_eq!(Number::Four, Number::Four);
        assert_eq!(Number::Five, Number::Five);
    }

    #[test]
    fn number_ord() {
        assert!( Number::One < Number::Two );
        assert!( Number::Two < Number::Three );
        assert!( Number::Three < Number::Four );
        assert!( Number::Four < Number::Five );
    }

    #[test]
    fn card_view_clone() {
        let x = NormalCardView{ color : Some(NormalColor::Red), number : Some(Number::One) };
        let _y = x.clone();
    }

    #[test]
    fn card_clone() {
        let _x = NormalCard{ color : NormalColor::Red, number : Number::One, card_view: NormalCardView::new() };
        //let y = x.clone(); // TODO: static assert that this doesn't compile somehow?
    }

    #[test]
    fn card_view_eq() {
        let x = NormalCardView{ color : Some(NormalColor::Red), number : Some(Number::One) };
        let y = NormalCardView{ color : Some(NormalColor::Red), number : Some(Number::One) };
        assert_eq!(x,y);
    }

    #[test]
    fn normal_color_iter() {
        let mut type_iter = NormalColorTypeIter::from(NormalColor::Red);
        assert_eq!(type_iter.next(), Some(NormalColor::Red));
        assert_eq!(type_iter.next(), Some(NormalColor::White));
        assert_eq!(type_iter.next(), Some(NormalColor::Blue));
        assert_eq!(type_iter.next(), Some(NormalColor::Yellow));
        assert_eq!(type_iter.next(), Some(NormalColor::Green));
        assert_eq!(type_iter.next(), None);

        let type_iter = NormalColorTypeIter::from(NormalColor::Red);
        for col in type_iter {
            println!("col = {:?}", col);
        }
    }

    #[test]
    fn extended_color_iter() {
        let mut type_iter = ExtendedColorTypeIter::from(ExtendedColor::Red);
        assert_eq!(type_iter.next(), Some(ExtendedColor::Red));
        assert_eq!(type_iter.next(), Some(ExtendedColor::White));
        assert_eq!(type_iter.next(), Some(ExtendedColor::Blue));
        assert_eq!(type_iter.next(), Some(ExtendedColor::Yellow));
        assert_eq!(type_iter.next(), Some(ExtendedColor::Green));
        assert_eq!(type_iter.next(), Some(ExtendedColor::Rainbow));
        assert_eq!(type_iter.next(), None);

        let type_iter = ExtendedColorTypeIter::from(ExtendedColor::Red);
        for col in type_iter {
            println!("col = {:?}", col);
        }
    }

    #[test]
    fn number_type_iter() {
        let mut type_iter = NumberTypeIter::new();
        assert_eq!(type_iter.next(), Some(Number::One));
        assert_eq!(type_iter.next(), Some(Number::Two));
        assert_eq!(type_iter.next(), Some(Number::Three));
        assert_eq!(type_iter.next(), Some(Number::Four));
        assert_eq!(type_iter.next(), Some(Number::Five));
        assert_eq!(type_iter.next(), None);

        let type_iter = NumberTypeIter::new();
        for col in type_iter {
            println!("col = {:?}", col);
        }
    }

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
