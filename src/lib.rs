
use std::fmt::Debug;

/*
mod hanabi_err;
use hanabi_err::*;
*/

/*------------*/
/* Game Types */
/*------------*/
pub trait Color : Debug { }
#[derive(Debug)]
pub enum NormalColor {
    Red,
    White,
    Blue,
    Yellow,
    Green,
}

#[derive(Debug)]
pub enum ExtendedColor {
    Red,
    White,
    Blue,
    Yellow,
    Green,
    Rainbow,
}

impl Color for NormalColor { }
impl Color for ExtendedColor { }

// TODO: consider impl `From` for u8
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub trait Card : Debug {
    type ColorType : Debug;
    type NumberType : Debug;
}

#[derive(Debug)]
pub struct NormalCard<C> where
    C: Color
{
    pub color : C,
    pub number : Number,
}

impl Card for NormalCard<NormalColor> {
    type ColorType = NormalColor;
    type NumberType = Number;
}

impl Card for NormalCard<ExtendedColor> {
    type ColorType = ExtendedColor;
    type NumberType = Number;
}

#[derive(Debug, Clone, Copy)]
/**
 * @brief Represents a view onto the Card. The option sentinel represents the added possibility
 * that the player doesn't know anything about this card. For instance, if a player has no
 * knowledge about a card then its value will be CardView{ None, None }
 */
/*
pub struct CardView<C> where
    C: Color
{
    pub color: Option<C>,
    pub number: Option<Number>,
}
*/

pub struct UID(u64);
pub struct PubID(u8);
pub struct Player {
    pub public_id: PubID,

    uid: UID,
    cohorts: Vec<PubID>,
}

#[derive(Debug)]
pub struct Game<C> where C: Card {
    deck: Vec<C>,
    pub discard: Vec<C>,
    pub board: Vec<C>,

    player_hands: Vec<Vec<C>>,

    pub hints : u8,
    pub bombs : u8,
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


#[derive(Debug)]
pub enum Move<C: Card> {
    Play(C),
    Discard(C),
    Hint(HintType<C>),
}

/*--------------*/
/*   Helpers    */
/*--------------*/

/**
 * @brief Create a Non Shuffled Deck
 */
fn generate_standard_deck<C: Card>() -> Vec<C> {
    // TODO:
    Vec::new()
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

/* Game impl */
impl<C> Game<C> where C: Card {
    // Move related functions
    /**
     * @brief Checks to see if the move follows the rules of hanabi
     */
    //fn legal_move(&self, mv : Move) -> bool { } 

    /**
     * @brief Checks to see if the card being played can be played on the current board. Usually,
     * if this function returns true, the card will be "played". If not, it will be mvoed to the
     * discard pile.
     */
    //fn valid_play(&self, mv : Move) -> bool { }

    /**
     * @brief Submit a move to the game, ending your turn. The game will complete the turn and then
     * deal the player a new card from the deck.
     */
    //fn submit_move(&mut self, mv : Move) { }

    /**
     * @brief Check to see if the game is done.
     */
    fn game_finished(&self) -> bool {/*TODO*/ false }
    
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
    fn test_number_eq() {
        assert_eq!(Number::One, Number::One);
        assert_eq!(Number::Two, Number::Two);
        assert_eq!(Number::Three, Number::Three);
        assert_eq!(Number::Four, Number::Four);
        assert_eq!(Number::Five, Number::Five);
    }

    #[test]
    fn test_number_ord() {
        assert!( Number::One < Number::Two );
        assert!( Number::Two < Number::Three );
        assert!( Number::Three < Number::Four );
        assert!( Number::Four < Number::Five );
    }
} /* test */
