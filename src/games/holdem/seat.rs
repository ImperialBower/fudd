use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::slots::hole_cards::HoleCards;
use ckc_rs::PokerCard;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Seat {
    pub number: usize,
    folded: Cell<bool>,
    pub hole_cards: HoleCards,
}

impl Seat {
    #[must_use]
    pub fn new(number: usize) -> Seat {
        Seat::new_with_hole_cards(number, HoleCards::default())
    }

    #[must_use]
    pub fn from_index(number: usize, index: &'static str) -> Seat {
        Seat::new_with_hole_cards(number, HoleCards::from(index))
    }

    pub fn new_with_hole_cards(number: usize, hole_cards: HoleCards) -> Seat {
        Seat {
            number,
            folded: Cell::new(false),
            hole_cards,
        }
    }

    pub fn did_fold(&self) -> bool {
        self.folded.get()
    }
}

impl CardSlot for Seat {
    fn take(&self, card: PlayingCard) -> bool {
        self.hole_cards.take(card)
    }

    fn fold(&self) -> PlayingCards {
        self.folded.set(true);
        self.hole_cards.fold()
    }

    fn is_dealt(&self) -> bool {
        !self.hole_cards.get_first_card().is_blank()
            && !self.hole_cards.get_second_card().is_blank()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        playing_cards.insert(self.hole_cards.get_first_card());
        playing_cards.insert(self.hole_cards.get_second_card());
        playing_cards
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seat {}: {}", self.number, self.hole_cards)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_seat_tests {
    use super::*;

    #[test]
    fn default() {
        let seat = Seat {
            number: 0,
            folded: Cell::new(false),
            hole_cards: HoleCards::default(),
        };

        assert_eq!(Seat::default(), seat);
    }
}
