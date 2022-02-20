use crate::types::arrays::four_cards::FourCards;
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
pub struct OmahaHand(
    Cell<PlayingCard>,
    Cell<PlayingCard>,
    Cell<PlayingCard>,
    Cell<PlayingCard>,
);

impl OmahaHand {
    #[must_use]
    pub fn new(
        first: PlayingCard,
        second: PlayingCard,
        third: PlayingCard,
        forth: PlayingCard,
    ) -> OmahaHand {
        OmahaHand(
            Cell::new(first),
            Cell::new(second),
            Cell::new(third),
            Cell::new(forth),
        )
    }

    pub fn get_first_card(&self) -> PlayingCard {
        self.0.get()
    }

    pub fn get_second_card(&self) -> PlayingCard {
        self.1.get()
    }

    pub fn get_third_card(&self) -> PlayingCard {
        self.2.get()
    }

    pub fn get_forth_card(&self) -> PlayingCard {
        self.3.get()
    }

    pub fn take_first_card(&self, card: PlayingCard) {
        self.0.set(card);
    }

    pub fn take_second_card(&self, card: PlayingCard) {
        self.1.set(card);
    }

    pub fn take_third_card(&self, card: PlayingCard) {
        self.2.set(card);
    }

    pub fn take_forth_card(&self, card: PlayingCard) {
        self.3.set(card);
    }

    pub fn to_array(&self) -> [U32Card; 4] {
        [
            self.get_first_card().as_u32(),
            self.get_second_card().as_u32(),
            self.get_third_card().as_u32(),
            self.get_forth_card().as_u32(),
        ]
    }
}

impl CardSlot for OmahaHand {
    fn take(&self, card: PlayingCard) -> bool {
        if self.get_first_card().is_blank() {
            self.take_first_card(card);
            return true;
        }
        if self.get_second_card().is_blank() {
            self.take_second_card(card);
            return true;
        }
        if self.get_third_card().is_blank() {
            self.take_third_card(card);
            return true;
        }
        if self.get_forth_card().is_blank() {
            self.take_forth_card(card);
            return true;
        }
        false
    }

    fn fold(&self) -> PlayingCards {
        let folded = self.to_playing_cards();
        self.0.set(PlayingCard::default());
        self.1.set(PlayingCard::default());
        self.2.set(PlayingCard::default());
        self.3.set(PlayingCard::default());
        folded
    }

    fn is_dealt(&self) -> bool {
        !self.get_first_card().is_blank()
            && !self.get_second_card().is_blank()
            && !self.get_third_card().is_blank()
            && !self.get_forth_card().is_blank()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        playing_cards.insert(self.0.get());
        playing_cards.insert(self.1.get());
        playing_cards.insert(self.2.get());
        playing_cards.insert(self.3.get());
        playing_cards
    }
}

impl Default for OmahaHand {
    fn default() -> OmahaHand {
        OmahaHand::new(
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
        )
    }
}

impl fmt::Display for OmahaHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_playing_cards())
    }
}

impl From<[U32Card; 4]> for OmahaHand {
    fn from(array: [U32Card; 4]) -> Self {
        let hand = OmahaHand::default();
        hand.take_first_card(PlayingCard::from(array[0]));
        hand.take_second_card(PlayingCard::from(array[1]));
        hand.take_third_card(PlayingCard::from(array[2]));
        hand.take_forth_card(PlayingCard::from(array[3]));
        hand
    }
}

impl From<FourCards> for OmahaHand {
    fn from(four_cards: FourCards) -> Self {
        OmahaHand::from(four_cards.to_arr())
    }
}

impl From<&'static str> for OmahaHand {
    fn from(value: &'static str) -> OmahaHand {
        let omaha = OmahaHand::default();
        if !omaha.take_from_index(value) {
            warn!("Invalid index: {}", value);
        }
        omaha
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_slots_omaha_hand_tests {
    use super::*;

    #[test]
    fn from__four_cards() {
        let index = "AS KD 4C 2S";
        let four = FourCards::try_from(index).unwrap();
        let omaha = OmahaHand::from(index);

        assert_eq!(four.to_string(), omaha.to_string());
    }
}
