/*------------*/
/* Game Types */
/*------------*/

use std::fmt::Debug;

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
}
