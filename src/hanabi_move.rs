/*------------*/
/* Move Types */
/*------------*/

use std::fmt::Debug;

use card::*;
use player::*;

// TODO: this needs consideration
#[derive(Debug)]
pub enum HintType<C> where
    C: Card
{
    ColorHint(C::ColorType),
    NumberHint(C::NumberType),
}

pub type HintForPlayer<C> = (PubID, HintType<C>);

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

