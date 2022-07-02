use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use ckc_rs::PokerCard;
use log::warn;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SingleCard(Cell<PlayingCard>);

impl SingleCard {
    #[must_use]
    pub fn new(poker_card: PlayingCard) -> SingleCard {
        let slot = SingleCard::default();
        slot.0.set(poker_card);
        slot
    }

    pub fn get(&self) -> PlayingCard {
        self.0.get()
    }
}

impl CardSlot for SingleCard {
    fn take(&self, card: PlayingCard) -> bool {
        if self.get().is_blank() {
            self.0.set(card);
            return true;
        }
        false
    }

    fn fold(&self) -> PlayingCards {
        let folded = self.to_playing_cards();
        self.0.set(PlayingCard::default());
        folded
    }

    fn is_dealt(&self) -> bool {
        !self.get().is_blank()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        playing_cards.insert(self.get());
        playing_cards
    }
}

impl Default for SingleCard {
    fn default() -> SingleCard {
        SingleCard(Cell::new(PlayingCard::default()))
    }
}

impl fmt::Display for SingleCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl From<&'static str> for SingleCard {
    fn from(value: &'static str) -> SingleCard {
        let slot = SingleCard::default();
        let valid = slot.take_from_index(value);
        if !valid {
            warn!("Invalid index: {}", value);
        }
        slot
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_single_card_tests {
    use super::*;

    #[test]
    fn new() {
        let slot: SingleCard = SingleCard::new(PlayingCard::from("4C"));

        assert_eq!("4♣", slot.to_string());
        assert!(slot.is_dealt());
    }

    #[test]
    fn from_index() {
        let slot = SingleCard::from("A♦");

        assert_eq!("A♦", format!("{}", slot));
        assert!(slot.is_dealt());
    }

    #[test]
    fn from_index__invalid() {
        let slot = SingleCard::from("FF");

        assert_eq!("__", format!("{}", slot));
        assert!(!slot.is_dealt());
        assert!(slot.is_blank());
    }

    #[test]
    fn get_playing_card() {
        let slot = SingleCard::from("A♦");

        let expected = PlayingCard::from("A♦");

        assert_eq!(expected, slot.get());
    }

    #[test]
    fn take() {
        let slot = SingleCard::default();

        let taken = slot.take(PlayingCard::from("A♦"));

        assert_eq!("A♦", slot.to_string());
        assert!(slot.is_dealt());
        assert!(taken);
        assert!(!slot.take(PlayingCard::from("A♦")));
    }

    // This test runs the full gamut of a flop flop.
    #[test]
    fn full_run() {
        let slot = SingleCard::default();
        assert_eq!("__", format!("{}", slot));
        assert!(!slot.is_dealt());

        let taken = slot.take_from_index("AS");
        assert_eq!("A♠", format!("{}", slot));
        assert!(slot.is_dealt());
        assert!(taken);

        // Try to take a 2nd card, which should fail.
        let taken = slot.take_from_index("A♦");
        assert_eq!("A♠", format!("{}", slot));
        assert!(slot.is_dealt());
        assert!(!taken);
    }

    #[test]
    fn default() {
        let slot = SingleCard::default();

        assert_eq!("__", format!("{}", slot));
        assert_eq!("", format!("{}", slot.to_playing_cards()));
        assert_eq!(0, slot.dealt().len());
        assert!(!slot.is_dealt());
    }

    #[test]
    fn to_string() {
        let flop = SingleCard::from("AH");

        assert_eq!("A♥", flop.to_string());
    }
}
