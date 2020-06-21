extern crate bitflags;

use std::collections::VecDeque;

use crate::errors::HanabiError;

// TODO: make a macro that makes both Color, ColorKnowledge, and impls the From trait
// TODO: same with Number, etc...

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red = 0b00001,
    White = 0b00010,
    Blue = 0b00100,
    Green = 0b01000,
    Yellow = 0b10000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    One = 0b00001,
    Two = 0b00010,
    Three = 0b00100,
    Four = 0b01000,
    Five = 0b10000,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    color: Color,
    number: Number,
}

impl Card {
    // TODO: this is pub(crate) so that we can write a unit test in player.rs
    // There might be a better way to structure this
    pub(crate) fn new(color: Color, number: Number) -> Card {
        Card {
            color,
            number,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn number(&self) -> Number {
        self.number
    }

    pub fn view(&self) -> CardView {
        CardView {
            inner: Card::new(self.color, self.number),
        }
    }
}

pub struct CardView {
    inner: Card,
}

impl CardView {
    pub fn color(&self) -> Color {
        self.inner.color
    }

    pub fn number(&self) -> Number {
        self.inner.number
    }
}

// Private here
bitflags! {
    #[derive(Default)]
    pub struct ColorKnowledge: u32 {
        const RED    = Color::Red    as u32;
        const WHITE  = Color::White  as u32;
        const BLUE   = Color::Blue   as u32;
        const GREEN  = Color::Green  as u32;
        const YELLOW = Color::Yellow as u32;
        const ALL_COLORS = Self::RED.bits | Self::WHITE.bits | Self::BLUE.bits | Self::GREEN.bits | Self::YELLOW.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct NumberKnowledge: u32 {
        const ONE   = Number::One   as u32;
        const TWO   = Number::Two   as u32;
        const THREE = Number::Three as u32;
        const FOUR  = Number::Four  as u32;
        const FIVE  = Number::Five  as u32;
        const ALL_NUMBERS = Self::ONE.bits | Self::TWO.bits | Self::THREE.bits | Self::FOUR.bits | Self::FIVE.bits;
    }
}

impl From<Color> for ColorKnowledge {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => ColorKnowledge::RED,
            Color::White => ColorKnowledge::WHITE,
            Color::Blue => ColorKnowledge::BLUE,
            Color::Green => ColorKnowledge::GREEN,
            Color::Yellow => ColorKnowledge::YELLOW,
        }
    }
}

impl From<Number> for NumberKnowledge {
    fn from(color: Number) -> Self {
        match color {
            Number::One => NumberKnowledge::ONE,
            Number::Two => NumberKnowledge::TWO,
            Number::Three => NumberKnowledge::THREE,
            Number::Four => NumberKnowledge::FOUR,
            Number::Five => NumberKnowledge::FIVE,
        }
    }
}

// The best way to keep knowledge about a card is to keep track of what you *don't* know about the
// card. Much easier to keep track of.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CardKnowledge {
    pub not_these_colors: ColorKnowledge,
    pub not_these_numbers: NumberKnowledge,
}

impl CardKnowledge {
    pub fn new() -> Self {
        CardKnowledge {
            not_these_colors: Default::default(),
            not_these_numbers: Default::default(),
        }
    }
}

pub fn this_color(ck: CardKnowledge, color: Color) -> Result<CardKnowledge, HanabiError> {
    // For a sanity check, just make sure that Card::not_these_colors doesn't have the color bit
    // set already ... would indicate some sort of logic error in the game overall since we should
    // never indicate a card both "is" and "is not" a given color at the same time.

    if ck.not_these_colors == (ck.not_these_colors | color.into()) {
        return Err(HanabiError::LogicError("Card was previously designated as not being this color, indicating an internal game logic error".to_string()));
    }

    let new_colors = ColorKnowledge::ALL_COLORS ^ color.into();

    Ok(CardKnowledge {
        not_these_colors: new_colors,
        not_these_numbers: ck.not_these_numbers,
    })
}

pub fn this_number(ck: CardKnowledge, number: Number) -> Result<CardKnowledge, HanabiError> {
    // Sanity check -- see above in this_color()
    if ck.not_these_numbers == (ck.not_these_numbers | number.into()) {
        return Err(HanabiError::LogicError("Card was previously designated as not being this number, indicating an internal game logic error".to_string()));
    }

    let new_numbers = NumberKnowledge::ALL_NUMBERS ^ number.into();

    Ok(CardKnowledge {
        not_these_colors: ck.not_these_colors,
        not_these_numbers: new_numbers,
    })
}

pub fn not_this_color(ck: CardKnowledge, color: Color) -> Result<CardKnowledge, HanabiError> {
    let new_colors = ck.not_these_colors | color.into();

    if new_colors == ColorKnowledge::ALL_COLORS {
        Err(HanabiError::LogicError(
            "Impossible for a card to not be every color".to_string(),
        ))
    } else {
        Ok(CardKnowledge {
            not_these_colors: new_colors,
            not_these_numbers: ck.not_these_numbers,
        })
    }
}

pub fn not_this_number(ck: CardKnowledge, number: Number) -> Result<CardKnowledge, HanabiError> {
    let new_numbers = ck.not_these_numbers | number.into();

    if new_numbers == NumberKnowledge::ALL_NUMBERS {
        Err(HanabiError::LogicError(
            "Impossible for a card to not be every number".to_string(),
        ))
    } else {
        Ok(CardKnowledge {
            not_these_colors: ck.not_these_colors,
            not_these_numbers: new_numbers,
        })
    }
}

// This is messy... It doesn't really need to take a functor for card frequencies... this code used
// to live in rules.rs but it needs to construct `Card`s and I wanted to make Card have a "private
// constructor" so I moved here but kept card_frequencies in rules.rs
pub fn generate_deck<F: Fn(&Number) -> u8>(card_frequencies: F) -> VecDeque<Card> {
    let colors = vec![
        Color::Red,
        Color::White,
        Color::Blue,
        Color::Green,
        Color::Yellow,
    ];
    let numbers = vec![
        Number::One,
        Number::Two,
        Number::Three,
        Number::Four,
        Number::Five,
    ];

    let mut deck = VecDeque::new();
    for col in &colors {
        for num in &numbers {
            for _ in 0..card_frequencies(num) {
                deck.push_front(Card::new(*col, *num));
            }
        }
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_this_color() {
        {
            let mut card_knowledge = CardKnowledge::new();

            card_knowledge = this_color(card_knowledge, Color::Red).unwrap();

            assert_eq!(
                card_knowledge.not_these_colors,
                ColorKnowledge::WHITE
                    | ColorKnowledge::BLUE
                    | ColorKnowledge::YELLOW
                    | ColorKnowledge::GREEN
            );
        }
        {
            let mut card_knowledge = CardKnowledge::new();
            card_knowledge = this_color(card_knowledge, Color::Green).unwrap();
            let err = this_color(card_knowledge, Color::Blue);

            assert!(err.is_err());
        }
    }
    #[test]
    fn test_not_this_color() {
        let mut card_knowledge = CardKnowledge::new();

        // First color is fine.
        card_knowledge = not_this_color(card_knowledge, Color::Green).unwrap();
        assert_eq!(card_knowledge.not_these_colors, ColorKnowledge::GREEN);

        // Second color is fine
        card_knowledge = not_this_color(card_knowledge, Color::Red).unwrap();
        assert_eq!(
            card_knowledge.not_these_colors,
            ColorKnowledge::GREEN | ColorKnowledge::RED
        );

        // Third and Fourth color is fine
        card_knowledge = not_this_color(card_knowledge, Color::Blue).unwrap();
        card_knowledge = not_this_color(card_knowledge, Color::White).unwrap();
        assert_eq!(
            card_knowledge.not_these_colors,
            ColorKnowledge::GREEN
                | ColorKnowledge::RED
                | ColorKnowledge::BLUE
                | ColorKnowledge::WHITE
        );

        // You can give the same clue twice... TODO: consider changing this to an error???
        card_knowledge = not_this_color(card_knowledge, Color::White).unwrap();

        // Fifth color is no good because a card can't *NOT* be *every* color (it must be *some*
        // color)
        let err = not_this_color(card_knowledge, Color::Yellow);
        assert!(err.is_err());
    }

    #[test]
    fn test_this_number() {
        {
            let mut card_knowledge = CardKnowledge::new();

            card_knowledge = this_number(card_knowledge, Number::Two).unwrap();

            assert_eq!(
                card_knowledge.not_these_numbers,
                NumberKnowledge::ONE
                    | NumberKnowledge::THREE
                    | NumberKnowledge::FOUR
                    | NumberKnowledge::FIVE
            );
        }
        {
            let mut card_knowledge = CardKnowledge::new();
            card_knowledge = this_number(card_knowledge, Number::Three).unwrap();
            let err = this_number(card_knowledge, Number::Four);

            assert!(err.is_err());
        }
    }
}
