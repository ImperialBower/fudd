use crate::analysis::eval::Eval;
use crate::analysis::evals::EvalsPerClass;
use crate::types::arrays::five_card::FiveCard;
use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::slots::hole_cards::HoleCards;
use crate::types::U32Card;
use ckc_rs::hand_rank::HandRank;
use ckc_rs::PokerCard;
use log::warn;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Flop(Cell<PlayingCard>, Cell<PlayingCard>, Cell<PlayingCard>);

impl Flop {
    #[must_use]
    pub fn new(first: PlayingCard, second: PlayingCard, third: PlayingCard) -> Flop {
        Flop(Cell::new(first), Cell::new(second), Cell::new(third))
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

    pub fn take_first_card(&self, card: PlayingCard) {
        self.0.set(card);
    }

    pub fn take_second_card(&self, card: PlayingCard) {
        self.1.set(card);
    }

    pub fn take_third_card(&self, card: PlayingCard) {
        self.2.set(card);
    }

    pub fn to_array(&self) -> [U32Card; 3] {
        [
            self.get_first_card().as_u32(),
            self.get_second_card().as_u32(),
            self.get_third_card().as_u32(),
        ]
    }

    pub fn to_array_add_hole_cards(&self, hole_cards: &HoleCards) -> [U32Card; 5] {
        let ha = hole_cards.to_array();
        let fa = self.to_array();
        [ha[0], ha[1], fa[0], fa[1], fa[2]]
    }

    pub fn to_poker_hand_add_hole_cards(&self, hole_cards: &HoleCards) -> FiveCard {
        FiveCard::from(self.to_array_add_hole_cards(hole_cards))
    }

    pub fn eval_against_hole_cards(&self, hole_cards: &HoleCards) -> HandRank {
        Eval::from(self.to_array_add_hole_cards(hole_cards)).rank
    }

    pub fn is_the_nuts(&self, hole_cards: &HoleCards) -> bool {
        let nuts = self.the_nuts();
        self.eval_against_hole_cards(hole_cards) == nuts.rank
    }

    /// Determine The Nuts for a given flop.
    #[allow(clippy::missing_panics_doc)]
    pub fn the_nuts(&self) -> Eval {
        if !self.is_dealt() {
            return Eval::default();
        }

        let hands = self.all_possible();
        match hands.to_vec().first() {
            None => Eval::default(),
            Some(hand) => *hand,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn all_possible(&self) -> EvalsPerClass {
        let mut hands = EvalsPerClass::default();

        if !self.is_dealt() {
            return hands;
        }

        let dummy_kev_value: U32Card = 0;
        let mut current_hand: [U32Card; 5] = [dummy_kev_value; 5];

        // Determine what cards are left
        let remaining = self.remaining();
        for i1 in 0..49usize {
            for i2 in (i1 + 1)..49usize {
                current_hand[0] = remaining.get_index(i1).unwrap().as_u32();
                current_hand[1] = remaining.get_index(i2).unwrap().as_u32();
                current_hand[2] = self.get_first_card().as_u32();
                current_hand[3] = self.get_second_card().as_u32();
                current_hand[4] = self.get_third_card().as_u32();

                hands.push(Eval::from(current_hand));
            }
        }
        hands.sort_in_place();
        hands
    }
}

impl CardSlot for Flop {
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
        false
    }

    fn fold(&self) -> PlayingCards {
        let folded = self.to_playing_cards();
        self.0.set(PlayingCard::default());
        self.1.set(PlayingCard::default());
        self.2.set(PlayingCard::default());
        folded
    }

    fn is_dealt(&self) -> bool {
        !self.get_first_card().is_blank()
            && !self.get_second_card().is_blank()
            && !self.get_third_card().is_blank()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        playing_cards.insert(self.0.get());
        playing_cards.insert(self.1.get());
        playing_cards.insert(self.2.get());
        playing_cards
    }
}

impl Default for Flop {
    fn default() -> Flop {
        Flop::new(
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
        )
    }
}

impl fmt::Display for Flop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.get_first_card(),
            self.get_second_card(),
            self.get_third_card()
        )
    }
}

impl From<&'static str> for Flop {
    fn from(value: &'static str) -> Flop {
        let flop = Flop::default();
        let valid = flop.take_from_index(value);
        if !valid {
            warn!("Invalid index: {}", value);
        }
        flop
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_flop_tests {
    use super::*;
    use crate::types::arrays::Vectorable;
    use ckc_rs::CardNumber;

    #[test]
    fn from_index() {
        let slot = Flop::from("AS AD AH");

        assert_eq!("A♠ A♦ A♥", format!("{}", slot));
        assert!(slot.is_dealt());
    }

    #[test]
    fn from_index__invalid() {
        let slot = Flop::from("FF");

        assert_eq!("__ __ __", format!("{}", slot));
        assert!(!slot.is_dealt());
    }

    #[test]
    fn take_from_index() {
        let flop = Flop::default();
        flop.take_from_index("AS AD AH");

        assert_eq!("A♠ A♦ A♥", format!("{}", flop));
    }

    #[test]
    fn to_array() {
        let slot = Flop::from("AS AD AH");

        assert_eq!(
            [
                CardNumber::ACE_SPADES,
                CardNumber::ACE_DIAMONDS,
                CardNumber::ACE_HEARTS
            ],
            slot.to_array()
        );
    }

    #[test]
    fn to_array__default() {
        assert_eq!(
            [CardNumber::BLANK, CardNumber::BLANK, CardNumber::BLANK],
            Flop::default().to_array()
        );
    }

    #[test]
    fn is_the_nuts() {
        let slot = Flop::from("TS 9D 8H");
        let hole_cards = HoleCards::from("Q♠ J♠");

        assert!(slot.is_the_nuts(&hole_cards));
    }

    #[test]
    fn is_the_nuts__false() {
        let slot = Flop::from("TS 9D 8H");
        let hole_cards = HoleCards::from("7S J♠");

        assert!(!slot.is_the_nuts(&hole_cards));
    }

    #[test]
    fn the_nuts() {
        let slot = Flop::from("TS 9D 8H");

        let nuts = slot.the_nuts();

        assert_eq!(
            "HandRank { value: 1602, name: Straight, class: QueenHighStraight }",
            nuts.rank.to_string()
        );
        assert_eq!("Q♠ J♠ T♠ 9♦ 8♥", nuts.hand.to_poker_cards().to_string());
    }

    // This test runs the full gamut of a flop flop.
    #[test]
    fn full_run() {
        let flop = Flop::default();
        assert_eq!("__ __ __", format!("{}", flop));
        assert_eq!("", format!("{}", flop.dealt()));
        assert!(!flop.is_dealt());

        let taken = flop.take_from_index("AS");
        assert_eq!("A♠ __ __", format!("{}", flop));
        assert_eq!("A♠", format!("{}", flop.dealt()));
        assert!(!flop.is_dealt());
        assert!(taken);

        let taken = flop.take_from_index("AD");
        assert_eq!("A♠ A♦ __", format!("{}", flop));
        assert_eq!("A♠ A♦", format!("{}", flop.dealt()));
        assert!(!flop.is_dealt());
        assert!(taken);

        let taken = flop.take_from_index("AH");
        assert_eq!("A♠ A♦ A♥", format!("{}", flop));
        assert_eq!("A♠ A♦ A♥", format!("{}", flop.dealt()));
        assert!(flop.is_dealt());
        assert!(taken);

        // Try to take a 4th card, which should fail.
        let taken = flop.take_from_index("AH");
        assert_eq!("A♠ A♦ A♥", format!("{}", flop));
        assert_eq!("A♠ A♦ A♥", format!("{}", flop.dealt()));
        assert!(flop.is_dealt());
        assert!(!taken);
    }

    #[test]
    fn default() {
        let flop = Flop::default();

        assert_eq!("__ __ __", format!("{}", flop));
        assert_eq!("", format!("{}", flop.to_playing_cards()));
        assert_eq!(0, flop.dealt().len());
        assert!(!flop.is_dealt());
    }

    #[test]
    fn to_string() {
        let flop = Flop::from("AS AD AH");

        assert_eq!("A♠ A♦ A♥", flop.to_string());
    }
}
