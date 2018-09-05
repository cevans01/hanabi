
extern crate static_assertions;
extern crate lazy_static;

use std::fmt::Debug;
//use static_assertions::{assert_impl};


use std::collections::HashMap;

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
    type ColorType : Debug + Color;
    type NumberType : Debug;

    fn new(col : Self::ColorType, num : Self::NumberType) -> Self;
}

#[derive(Debug)]
// DO NOT IMPL CLONE/COPY
pub struct HanabiCard<C> where
    C: Color
{
    pub color : C,
    pub number : Number,
}

pub type NormalCard = HanabiCard<NormalColor>;
pub type ExtendedCard = HanabiCard<ExtendedColor>;

impl Card for NormalCard {
    type ColorType = NormalColor;
    type NumberType = Number;

    fn new(col : Self::ColorType, num : Self::NumberType) -> NormalCard {
        NormalCard{color : col, number : num  }
    }
}

impl Card for ExtendedCard {
    type ColorType = ExtendedColor;
    type NumberType = Number;

    fn new(col : Self::ColorType, num : Self::NumberType) -> ExtendedCard {
        ExtendedCard{color : col, number : num  }
    }
}

/* ----------------------- */
/* --- CardView Types ---- */
/* ----------------------- */
pub trait CardView : Debug + Clone + Copy + Eq + PartialEq {
    type ColorType : Color + Debug;
    type NumberType : Debug;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/**
 * @brief Represents a view onto the Card. The option sentinel represents the added possibility
 * that the player doesn't know anything about this card. For instance, if a player has no
 * knowledge about a card then its value will be CardView{ None, None }
 */
pub struct HanabiCardView<C> where
    C: Color
{
    pub color: Option<C>,
    pub number: Option<Number>,
}

pub type NormalCardView = HanabiCardView<NormalColor>;
pub type ExtendedCardView = HanabiCardView<ExtendedColor>;

impl CardView for NormalCardView {
    type ColorType = NormalColor;
    type NumberType = Number;
}

impl CardView for ExtendedCardView {
    type ColorType = ExtendedColor;
    type NumberType = Number;
}

pub struct UID(u64);
pub struct PubID(u8);
pub struct Player {
    pub public_id: PubID,

    uid: UID,
    cohorts: Vec<PubID>,
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

pub trait Move : Debug {
    type Card;
}

#[derive(Debug)]
pub enum HanabiMove<C: Card> {
    Play(C),
    Discard(C),
    Hint(HintType<C>),
}

impl Move for HanabiMove<NormalCard> {
    type Card = NormalCard;
}
impl Move for HanabiMove<ExtendedCard> {
    type Card = ExtendedCard;
}

/*--------------*/
/*   Helpers    */
/*--------------*/

pub fn generate_normal_deck() -> Vec<NormalCard>
{
    let mut deck = Vec::new();
    for col in VEC_NORMAL_COLORS.iter() {
        for num in VEC_NUMBERS.iter() {
            for num_card in 0..CARD_FREQUENCIES[&*num] {
                deck.push(NormalCard::new(*col, *num));
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

/**
 * @brief Create a Non Shuffled Deck
 */
pub fn generate_deck<C: Card>() -> Vec<C> 
    where <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{
    let color_iter = <C::ColorType as Color>::new().get_iter();
    let number_iter = NumberTypeIter::new();

    let rv = Vec::new();

    for col in color_iter {
        for num in number_iter {

            //let foo = C::new(col, num);

            // TODO: TODO: TODO: Compile error here, what the HECK is going on???
            //let foo = <C as Card>::new(col, num);

            println!("col = {:?}, num = {:?}", col, num);
        }
    }
    rv
}

/**
 * @brief Create players, initialized with IDs
 */
fn generate_players() -> Vec<Player> {
    // TODO:
    Vec::new()
}

/**
 * @brief Shuffle an existing deck
 */
fn shuffle<C: Card>(deck : &mut Vec<C>) { }


/* Game */
#[derive(Debug)]
pub struct Game<C>
    where C: Card,
    <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{
    deck: Vec<C>,
    pub discard: Vec<C>,
    pub board: Vec<C>,

    player_hands: Vec<Vec<C>>,

    pub hints : u8,
    pub bombs : u8,
}

impl<C> Game<C>
    where C: Card,
    <<<C as Card>::ColorType as Color>::IterType as Iterator>::Item : Debug
{

    pub fn new(deck : Vec<C>, _players: u8) -> Game<C> {
        Game {
            // SUUUPER annoying that it can't create the deck itself and must be passed in. TODO
            // FIX THIS
            deck    : deck,
            discard : Vec::new(),
            board   : Vec::new(),
            player_hands : Vec::new(),
            hints : 10,
            bombs : 3,
        }
    }
    // Move related functions
    /**
     * @brief Checks to see if the move follows the rules of hanabi
     */
    fn legal_move<M : Move>(&self, mv : M) -> bool {/*TODO*/ false } 

    /**
     * @brief Checks to see if the card being played can be played on the current board. Usually,
     * if this function returns true, the card will be "played". If not, it will be mvoed to the
     * discard pile.
     */
    fn valid_play<M : Move>(&self, mv : M) -> bool {/*TODO*/ false }

    /**
     * @brief Submit a move to the game, ending your turn. The game will complete the turn and then
     * deal the player a new card from the deck.
     */
    //fn submit_move(&mut self, mv : Move) { }

    /**
     * @brief Check to see if the game is done.
     */
    fn finished(&self) -> bool {/*TODO*/ false }
    
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
        let y = x.clone();
    }

    #[test]
    fn card_clone() {
        let x = NormalCard{ color : NormalColor::Red, number : Number::One };
        //let y = x.clone();
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

        let mut type_iter = NormalColorTypeIter::from(NormalColor::Red);
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

        let mut type_iter = ExtendedColorTypeIter::from(ExtendedColor::Red);
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

        let mut type_iter = NumberTypeIter::new();
        for col in type_iter {
            println!("col = {:?}", col);
        }
    }
} /* test */
