use crate::types::arrays::two_cards::TwoCards;
use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::U32Card;
use ckc_rs::PokerCard;
use log::warn;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HoleCards(Cell<PlayingCard>, Cell<PlayingCard>);

impl HoleCards {
    #[must_use]
    pub fn new(first: PlayingCard, second: PlayingCard) -> HoleCards {
        HoleCards(Cell::new(first), Cell::new(second))
    }

    /// # Errors
    ///
    /// Will throw a `HandError::InvalidCard` if an invalid index is passed in.
    ///
    /// Will throw a `HandError::InvalidIndex` if the number of cards passed in
    /// doesn't equal 2. (There must be two cards for each `Player`.)
    #[must_use]
    pub fn from_index(index: &'static str) -> HoleCards {
        HoleCards::from(index)
    }

    pub fn get_first_card(&self) -> PlayingCard {
        self.0.get()
    }

    pub fn get_second_card(&self) -> PlayingCard {
        self.1.get()
    }

    pub fn take_first_card(&self, card: PlayingCard) {
        self.0.set(card);
    }

    pub fn take_second_card(&self, card: PlayingCard) {
        self.1.set(card);
    }

    #[must_use]
    pub fn simple_index(&self) -> String {
        format!(
            "{} {}",
            self.get_first_card().simple_index(),
            self.get_second_card().simple_index()
        )
    }

    #[must_use]
    pub fn simple_index_short(&self) -> String {
        format!(
            "{}{}",
            self.get_first_card().simple_index(),
            self.get_second_card().simple_index()
        )
    }

    pub fn to_array(&self) -> [U32Card; 2] {
        [
            self.get_first_card().as_u32(),
            self.get_second_card().as_u32(),
        ]
    }
}

impl CardSlot for HoleCards {
    fn take(&self, card: PlayingCard) -> bool {
        if self.get_first_card().is_blank() {
            self.take_first_card(card);
            return true;
        }
        if self.get_second_card().is_blank() {
            self.take_second_card(card);
            return true;
        }
        false
    }

    fn fold(&self) -> PlayingCards {
        let folded = self.to_playing_cards();
        self.0.set(PlayingCard::default());
        self.1.set(PlayingCard::default());
        folded
    }

    fn is_dealt(&self) -> bool {
        !self.get_first_card().is_blank() && !self.get_second_card().is_blank()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        playing_cards.insert(self.0.get());
        playing_cards.insert(self.1.get());
        playing_cards
    }
}

impl Default for HoleCards {
    fn default() -> HoleCards {
        HoleCards::new(PlayingCard::default(), PlayingCard::default())
    }
}

impl fmt::Display for HoleCards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.get_first_card(), self.get_second_card())
    }
}

impl From<&'static str> for HoleCards {
    fn from(value: &'static str) -> HoleCards {
        let hole_cards = HoleCards::default();
        if !hole_cards.take_from_index(value) {
            warn!("Invalid index: {}", value);
        }
        hole_cards
    }
}

impl From<TwoCards> for HoleCards {
    fn from(two_cards: TwoCards) -> Self {
        HoleCards::new(
            PlayingCard::from(two_cards.first()),
            PlayingCard::from(two_cards.second()),
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_hole_cards_tests {
    use super::*;
    use ckc_rs::CardNumber;

    #[test]
    fn new() {
        let first = PlayingCard::from("A♦");
        let second = PlayingCard::from("K♦");

        let hole = HoleCards::new(first, second);

        assert_eq!("A♦ K♦", format!("{}", hole));
        assert_eq!("A♦ K♦", format!("{}", hole.to_playing_cards()));
        assert_eq!(2, hole.dealt().len());
        assert!(hole.is_dealt());
    }

    #[test]
    fn from_index() {
        let slot = HoleCards::from("AS AD");

        assert_eq!("A♠ A♦", format!("{}", slot));
        assert!(slot.is_dealt());
    }

    #[test]
    fn from_index__invalid() {
        let slot = HoleCards::from("FF");

        assert_eq!("__ __", format!("{}", slot));
        assert!(!slot.is_dealt());
    }

    #[test]
    fn is_dealt() {
        let hole = HoleCards::default();

        assert!(!hole.is_dealt());
    }

    #[test]
    fn is_dealt__first_card_dealt__false() {
        let hole = HoleCards::default();
        hole.take_from_index("AS");

        assert!(!hole.is_dealt());
    }

    #[test]
    fn is_dealt__second_card_dealt__false() {
        let hole = HoleCards::default();
        hole.take_second_card(PlayingCard::from("A♦"));

        assert!(!hole.is_dealt());
    }

    #[test]
    fn is_dealt__both_cards_dealt__true() {
        let hole = HoleCards::default();
        hole.take_from_index("AS AD");

        assert!(hole.is_dealt());
    }

    #[test]
    fn to_array() {
        let slot = HoleCards::from("AS AD");

        assert_eq!(
            [CardNumber::ACE_SPADES, CardNumber::ACE_DIAMONDS],
            slot.to_array()
        );
    }

    #[test]
    fn to_array__default() {
        assert_eq!(
            [CardNumber::BLANK, CardNumber::BLANK],
            HoleCards::default().to_array()
        );
    }

    #[test]
    fn default() {
        let hole = HoleCards::default();

        assert_eq!("__ __", format!("{}", hole));
        assert_eq!("", format!("{}", hole.to_playing_cards()));
        assert_eq!(0, hole.dealt().len());
        assert!(!hole.is_dealt());
    }

    #[test]
    fn display() {
        let hole = HoleCards::default();
        hole.take_from_index("AS");
        hole.take_from_index("AD");

        assert_eq!("A♠ A♦", format!("{}", hole));
    }

    #[test]
    fn display__default() {
        let hole = HoleCards::default();

        assert_eq!("__ __", format!("{}", hole));
    }

    #[test]
    fn display__one_card() {
        let hole = HoleCards::default();
        hole.take_from_index("AS");

        assert_eq!("A♠ __", format!("{}", hole));
    }

    #[test]
    fn display__only_second_card() {
        let hole = HoleCards::default();
        hole.take_second_card(PlayingCard::from("A♦"));

        assert_eq!("__ A♦", format!("{}", hole));
    }

    #[test]
    fn to_string() {
        let cards = HoleCards::from("AS AD");

        assert_eq!("A♠ A♦", cards.to_string());
    }

    #[test]
    fn from__two_cards() {
        let two_cards = TwoCards::try_from("AD AS").unwrap();
        let expected = HoleCards::from("AS AD");

        let actual = HoleCards::from(two_cards);

        assert_eq!(actual, expected);
    }
}
