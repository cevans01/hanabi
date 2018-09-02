
pub mod hanabi_err;

/*------------*/
/* Game Types */
/*------------*/
#[derive(Debug)]
pub enum Color {
    Red,
    White,
    Blue,
    Yellow,
    Green,
};

// TODO: consider impl `From` for u8
#[derive(Debug)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
};

#[derive(Debug)]
pub struct Card {
    pub color : Color,
    pub number : Number,
};

#[derive(Debug, Clone, Copy)]
/**
 * @brief Represents a view onto the Card. The option sentinel represents the added possibility
 * that the player doesn't know anything about this card. For instance, if a player has no
 * knowledge about a card then its value will be CardView{ None, None }
 */
pub struct CardView {
    pub color: Option<Color>,
    pub number: Option<Number>,
};

pub struct UID(u64);
pub struct PubID(u8);
pub struct Player {
    pub public_id: PubID,

    uid: UID,
    cohorts: Vec<PubID>,
};

#[derive(Debug)]
pub struct Game {
    deck; Vec<Card>,
    pub discard: Vec<Card>,
    pub board: Vec<Card>,

    player_hands: Vec<Vec<Card>>,

    pub hints : u8,
    pub bombs : u8,
};


/*------------*/
/* Move Types */
/*------------*/

#[derive(Debug)]
pub enum HintType {
    ColorHint(Color),
    NumberHint(Number),
};


#[derive(Debug)]
pub enum Move {
    Play(Card),
    Discard(Card),
    Hint(HintType),
};

/*--------------*/
/*   Helpers    */
/*--------------*/

/**
 * @brief Create a Non Shuffled Deck
 */
fn generate_standard_deck() -> Vec<Card> { }

/**
 * @brief Create players, initialized with IDs
 */
fn generate_players() -> Vec<Player> { }

/**
 * @brief Shuffle an existing deck
 */
fn shuffle(&mut deck : Vec<Card>) { }

/* Game impl */
impl Game {
    // Move related functions
    /**
     * @brief Checks to see if the move follows the rules of hanabi
     */
    fn legal_move(&self, mv : Move) -> bool { } 

    /**
     * @brief Checks to see if the card being played can be played on the current board. Usually,
     * if this function returns true, the card will be "played". If not, it will be mvoed to the
     * discard pile.
     */
    fn valid_play(&self, mv : Move) -> bool { }

    /**
     * @brief Submit a move to the game, ending your turn. The game will complete the turn and then
     * deal the player a new card from the deck.
     */
    fn submit_move(&mut self, mv : Move) { }

    /**
     * @brief Check to see if the game is done.
     */
    fn game_finished(&self) -> bool { }
    
    // Player related functions
    /**
     * @brief Get a look at another player's hand. You must submit your own player UID here, and
     * the public ID of the player you are requesting the cards for. This is to prevent players
     * requesting their own hands, which is not legal.
     *
     * @return If you are allowed to request the player's hand, a Vec<Card> for that player
     */
    pub fn get_player_cards(&self, requesting_player_uid : UID, target_player_pubid : PubID) -> Result<Vec<CardView>, HanabiErr> { }

    /**
     * @brief Get a Vec of CardView representing the knowledge that another player has of their own
     * hand. It is valid to call this against your own Player::public_id
     *
     * @return Knowlege that the player has of their hand
     */
    pub fn request_player_knowledge(&self, pubid : PubID) -> Vec<CardView> { }
};
