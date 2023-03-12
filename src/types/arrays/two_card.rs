use crate::types::arrays::Vectorable;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::ranges::two_cards_set::TwoCardsSet;
use crate::types::slots::hole_cards::HoleCards;
use crate::types::U32Card;
use ckc_rs::cards::two::Two;
use ckc_rs::{CardNumber, HandError, PokerCard};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::slice::Iter;

/// * [400+ Hole Card Hand Names](http://www.holdemsecrets.com/handnames.htm)
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct TwoCard([U32Card; 2]);

impl TwoCard {
    /// Factory method that ensures that the two cards are different, and neither
    /// is blank. Creates an array sorted their values.
    ///
    /// # Errors
    ///
    /// Throws a `HandError::DuplicateCard` error if the two cards are identical.
    #[allow(clippy::comparison_chain)]
    pub fn new(first: U32Card, second: U32Card) -> Result<TwoCard, HandError> {
        if first == second {
            Err(HandError::DuplicateCard)
        } else if first == CardNumber::BLANK || second == CardNumber::BLANK {
            Err(HandError::BlankCard)
        } else if first > second {
            Ok(TwoCard([
                PlayingCard::from(first).as_u32(),
                PlayingCard::from(second).as_u32(),
            ]))
        } else {
            Ok(TwoCard([
                PlayingCard::from(second).as_u32(),
                PlayingCard::from(first).as_u32(),
            ]))
        }
    }

    /// # Errors
    ///
    /// Will throw a `HandError::InvalidCard` if an invalid index is passed in.
    ///
    /// Will throw a `HandError::InvalidIndex` if the number of cards passed in
    /// doesn't equal 2. (There must be two cards for each `Player`.)
    pub fn from_index(index: &'static str) -> Result<TwoCard, HandError> {
        TwoCard::try_from(index)
    }

    #[must_use]
    pub fn chen_formula(&self) -> i8 {
        Two::from(self.to_arr()).chen_formula()
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn every_other(&self) -> TwoCardsSet {
        TwoCardsSet::from(self.remaining())
    }

    #[must_use]
    pub fn is_dealt(&self) -> bool {
        (self.0[0] != CardNumber::BLANK) && (self.0[1] != CardNumber::BLANK)
    }

    #[must_use]
    pub fn is_pocket_pair(&self) -> bool {
        self.rank_count() == 1
    }

    #[must_use]
    pub fn is_suited(&self) -> bool {
        self.suit_count() == 1
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, U32Card> {
        self.0.iter()
    }

    #[must_use]
    pub fn to_arr(&self) -> [U32Card; 2] {
        self.0
    }

    //region getters
    #[must_use]
    pub fn first(&self) -> U32Card {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> U32Card {
        self.0[1]
    }
    //endregion

    //region nicknamed
    pub const POCKET_ROCKETS: [U32Card; 2] = [CardNumber::ACE_DIAMONDS, CardNumber::ACE_CLUBS];
    pub const COWBOYS: [U32Card; 2] = [CardNumber::KING_DIAMONDS, CardNumber::KING_CLUBS];
    pub const LADIES: [U32Card; 2] = [CardNumber::QUEEN_DIAMONDS, CardNumber::QUEEN_CLUBS];
    pub const FISHHOOKS: [U32Card; 2] = [CardNumber::JACK_DIAMONDS, CardNumber::JACK_CLUBS];
    pub const DIMES: [U32Card; 2] = [CardNumber::TEN_DIAMONDS, CardNumber::TEN_CLUBS];
    pub const POPEYES: [U32Card; 2] = [CardNumber::NINE_DIAMONDS, CardNumber::NINE_CLUBS];
    pub const SNOWMEN: [U32Card; 2] = [CardNumber::EIGHT_DIAMONDS, CardNumber::EIGHT_CLUBS];
    pub const WALKING_STICKS: [U32Card; 2] = [CardNumber::SEVEN_DIAMONDS, CardNumber::SEVEN_CLUBS];
    pub const ROUTE_66: [U32Card; 2] = [CardNumber::SIX_DIAMONDS, CardNumber::SIX_CLUBS];
    pub const SPEED_LIMIT: [U32Card; 2] = [CardNumber::FIVE_DIAMONDS, CardNumber::FIVE_CLUBS];
    pub const SAILBOATS: [U32Card; 2] = [CardNumber::FOUR_DIAMONDS, CardNumber::FOUR_CLUBS];
    pub const CRABS: [U32Card; 2] = [CardNumber::TREY_DIAMONDS, CardNumber::TREY_CLUBS];
    pub const DUCKS: [U32Card; 2] = [CardNumber::DEUCE_DIAMONDS, CardNumber::DEUCE_CLUBS];
    //endregion
}

impl Vectorable for TwoCard {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.to_vec()
    }
}

impl fmt::Display for TwoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            PlayingCard::from(self.0[0]),
            PlayingCard::from(self.0[1])
        )
    }
}

impl From<&[U32Card; 2]> for TwoCard {
    fn from(array: &[U32Card; 2]) -> Self {
        TwoCard(*array)
    }
}

impl From<[U32Card; 2]> for TwoCard {
    fn from(array: [U32Card; 2]) -> Self {
        TwoCard(array)
    }
}

impl From<Vec<U32Card>> for TwoCard {
    fn from(v: Vec<U32Card>) -> Self {
        // let mut one = CardNumber::BLANK;

        let one = match v.first() {
            Some(c) => *c,
            _ => CardNumber::BLANK,
        };
        let two = match v.get(1) {
            Some(c) => *c,
            _ => CardNumber::BLANK,
        };

        match TwoCard::new(one, two) {
            Ok(two_cards) => two_cards,
            _ => TwoCard::default(),
        }
    }
}

impl From<Two> for TwoCard {
    fn from(two: Two) -> Self {
        TwoCard::from(two.to_arr())
    }
}

impl From<Vec<&PlayingCard>> for TwoCard {
    fn from(v: Vec<&PlayingCard>) -> Self {
        if v.len() < 2 {
            return TwoCard::default();
        }
        let first = *(*v.get(0).unwrap());
        let second = *(*v.get(1).unwrap());
        match TwoCard::new(first.as_u32(), second.as_u32()) {
            Ok(two_card) => two_card,
            Err(_) => TwoCard::default(),
        }
    }
}

impl TryFrom<HoleCards> for TwoCard {
    type Error = HandError;

    fn try_from(value: HoleCards) -> Result<Self, Self::Error> {
        TwoCard::new(
            value.get_first_card().as_u32(),
            value.get_second_card().as_u32(),
        )
    }
}

impl TryFrom<&'static str> for TwoCard {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match PlayingCards::try_from(value) {
            Ok(cards) => {
                if cards.len() == 2 {
                    TwoCard::new(
                        cards.get_index(0).unwrap().as_u32(),
                        cards.get_index(1).unwrap().as_u32(),
                    )
                } else {
                    Err(HandError::InvalidCardCount)
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod arrays_two_card_tests {
    use super::*;

    #[test]
    fn new() {
        let cards = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES);

        assert!(cards.is_ok());
        assert_eq!("A♠ A♣", cards.unwrap().to_string());
    }

    #[test]
    fn new__blank() {
        let err = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::BLANK);

        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), HandError::BlankCard);
    }

    #[test]
    fn new__duplicate() {
        let err = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_CLUBS);

        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), HandError::DuplicateCard);
    }

    #[test]
    fn array() {
        assert_eq!([0, 0], TwoCard::default().to_arr());
    }

    #[test]
    fn is_suited() {
        assert!(!TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES)
            .unwrap()
            .is_suited());
        assert!(TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::KING_CLUBS)
            .unwrap()
            .is_suited());
    }

    #[test]
    fn every_other() {
        let hand = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES).unwrap();

        let every_other = hand.every_other();

        assert!(!every_other.contains(&hand));
    }

    #[test]
    fn display() {
        let two = TwoCard::default();

        assert_eq!("__ __", two.to_string());
    }

    #[test]
    fn try_from__index() {
        let two = TwoCard::try_from("AS AC").unwrap();

        assert_eq!("A♠ A♣", two.to_string());
    }

    #[test]
    fn try_from__index_duplicate() {
        let err = TwoCard::try_from("AC AC");

        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), HandError::InvalidCardCount);
    }

    #[test]
    fn try_from__index__sort() {
        assert_eq!("A♠ A♣", TwoCard::try_from("AC AS").unwrap().to_string());
        assert_eq!("K♣ 9♠", TwoCard::try_from("9S KC").unwrap().to_string());
    }
}
