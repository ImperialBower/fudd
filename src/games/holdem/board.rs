use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::slots::flop::Flop;
use crate::types::slots::hole_cards::HoleCards;
use crate::types::slots::single_card::SingleCard;
use ckc_rs::hand_rank::HandRank;
use ckc_rs::HandError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Board {
    pub flop: Flop,
    pub turn: SingleCard,
    pub river: SingleCard,
}

impl Board {
    /// Takes a Card index string and returns a `Board` entity with all
    /// of the holdem filled in.
    ///
    /// # Errors
    ///
    /// Will return a `HandError::InvalidCard` error if an invalid index is passed in.
    pub fn from_index(index: &'static str) -> Result<Board, HandError> {
        Board::try_from(index)
    }

    pub fn eval_against_flop(&self, hole_cards: &HoleCards) -> HandRank {
        if !hole_cards.is_dealt() || !self.flop.is_dealt() {
            HandRank::default()
        } else {
            self.flop.eval_against_hole_cards(hole_cards)
        }
    }

    pub fn take_from_playing_cards(&self, cards: &PlayingCards) {
        for card in cards.iter() {
            self.take(*card);
        }
    }
}

impl CardSlot for Board {
    fn take(&self, card: PlayingCard) -> bool {
        if !self.flop.is_dealt() {
            self.flop.take(card)
        } else if !self.turn.is_dealt() {
            self.turn.take(card)
        } else if !self.river.is_dealt() {
            self.river.take(card)
        } else {
            false
        }
    }

    fn fold(&self) -> PlayingCards {
        let poker_cards = self.to_playing_cards();
        self.flop.fold();
        self.turn.fold();
        self.river.fold();
        poker_cards
    }

    /// Returns if all of the holdem have been dealt into.
    fn is_dealt(&self) -> bool {
        self.flop.is_dealt() && self.turn.is_dealt() && self.river.is_dealt()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut cards = PlayingCards::default();
        if self.flop.is_dealt() {
            cards.append(&self.flop.to_playing_cards());
        }
        if self.turn.is_dealt() {
            cards.append(&self.turn.to_playing_cards());
        }
        if self.river.is_dealt() {
            cards.append(&self.river.to_playing_cards());
        }
        cards
    }
}

impl Display for Board {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        out.entry(&(format!("FLOP:  {}", self.flop)).fmt_display());
        out.entry(&(format!("TURN:  {}", self.turn)).fmt_display());
        out.entry(&(format!("RIVER: {}", self.river)).fmt_display());

        out.finish()
    }
}

/// Takes a Card index string and returns a `Board` entity with all
/// of the holdem filled in.
///
/// # Errors
///
/// Will return a `HandError::InvalidCard` error if an invalid index is passed in.
impl TryFrom<&'static str> for Board {
    type Error = HandError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let poker_cards = PlayingCards::try_from(value);

        // if poker_cards.is_err() {
        //     Err(poker_cards.unwrap_err())
        // } else {
        //     let cards = poker_cards.unwrap();
        //     let board = Board::default();
        //     for card in cards.iter(){
        //         board.take(PlayingCard::from(card));
        //     }
        //     Ok(board)
        // }

        match poker_cards {
            Ok(cards) => Ok(Board::from(cards)),
            Err(e) => Err(e),
        }
    }
}

impl From<PlayingCards> for Board {
    fn from(value: PlayingCards) -> Self {
        let board = Board::default();
        for card in value.iter() {
            board.take(*card);
        }
        board
    }
}

impl From<Vec<PlayingCard>> for Board {
    fn from(value: Vec<PlayingCard>) -> Self {
        let board = Board::default();
        for card in &value {
            board.take(*card);
        }
        board
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_board_tests {
    use super::*;

    #[test]
    fn from_index() {
        let index = "K♣ 5♥ 3♠ 8♦ 9♠";

        let board = Board::from_index(index);

        assert!(board.is_ok());
        let board = board.unwrap();
        assert_eq!("K♣ 5♥ 3♠ 8♦ 9♠", board.to_playing_cards().to_string());
        assert!(board.is_dealt());
    }

    #[test]
    fn from_index__invalid_card() {
        // Who put a joker in the deck?
        let index = "9♥ 9♦ 2♦ 5♠ JK";

        let board = Board::from_index(index);

        assert!(board.is_err());
        assert_eq!(board.unwrap_err(), HandError::InvalidCard);
    }

    #[test]
    fn from_index__not_enough_cards() {
        let index = "4♥ K♣ 5♥ 3♠";

        let board = Board::from_index(index).unwrap();

        assert!(!board.is_dealt());
    }

    #[test]
    fn from_index__too_many_cards() {
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦ 9♥ 9♦ 2♦ 5♠ 7♦";

        let board = Board::from_index(index).unwrap();

        assert!(board.is_dealt());
        assert_eq!("4♥ K♣ 5♥ 3♠ 8♦", board.to_playing_cards().to_string());
    }

    #[test]
    fn from_poker_cards() {
        let poker_cards = PlayingCards::try_from("A♠ A♥ Q♠ J♠ T♠").unwrap();
        let board = Board::from(poker_cards);

        assert!(board.is_dealt());
        assert_eq!("A♠ A♥ Q♠", board.flop.to_string());
        assert_eq!("J♠", board.turn.to_string());
        assert_eq!("T♠", board.river.to_string());
        assert_eq!(5, board.len());
    }

    #[test]
    fn from_poker_cards__short() {
        let poker_cards = PlayingCards::try_from("A♠ A♥ 5♥").unwrap();
        let board = Board::from(poker_cards);

        assert!(!board.is_dealt());
        assert_eq!("A♠ A♥ 5♥", board.flop.to_string());
        assert_eq!("__", board.turn.to_string());
        assert_eq!("__", board.river.to_string());
        assert_eq!(3, board.len());
    }

    #[test]
    fn eval_against_flop() {
        let board = Board::from_index("K♣ 5♥ 3♠ 8♦ 9♠").unwrap();
        let hole_cards = HoleCards::from("KD 5D");

        assert_eq!(
            "HandRank { value: 2686, name: TwoPair, class: KingsAndFives }",
            board.eval_against_flop(&hole_cards).to_string()
        );
    }

    #[test]
    fn fold() {
        let index = "9♥ 9♦ 2♦ 5♠ 7♦";
        let board = Board::from_index(index).unwrap();

        let folded = board.fold();

        assert_eq!("[FLOP:  __ __ __, TURN:  __, RIVER: __]", board.to_string());
        assert_eq!(index, folded.to_string());
        assert!(!board.is_dealt());
        assert!(board.is_blank())
    }

    #[test]
    fn is_empty() {
        assert!(Board::default().is_blank());
        let board = Board::from(vec![
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
            PlayingCard::default(),
        ]);
        assert!(board.is_blank());
    }
}
