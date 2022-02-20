use crate::analysis::eval::Eval;
use crate::analysis::evals::Evals;
use crate::analysis::Evaluate;
use crate::games::holdem::board::Board;
use crate::types::arrays::three_cards::ThreeCards;
use crate::types::arrays::two_cards::TwoCards;
use crate::types::arrays::{Evaluable, Vectorable};
use crate::types::playing_card::PlayingCard;
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use cardpack::Pile;
use ckc_rs::cards::five::Five;
use ckc_rs::hand_rank::HandRank;
use ckc_rs::{HandError, PokerCard};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct FiveCards(pub [U32Card; 5]);

impl FiveCards {
    #[must_use]
    pub fn from_2_and_3(two: TwoCards, three: ThreeCards) -> FiveCards {
        FiveCards::from([
            two.first(),
            two.second(),
            three.first(),
            three.second(),
            three.third(),
        ])
    }

    #[must_use]
    pub fn sort(&self) -> FiveCards {
        let mut array = self.to_arr();
        array.sort_unstable();
        array.reverse();
        FiveCards(array)
    }

    #[must_use]
    pub fn first(&self) -> U32Card {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> U32Card {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> U32Card {
        self.0[2]
    }

    #[must_use]
    pub fn forth(&self) -> U32Card {
        self.0[3]
    }

    #[must_use]
    pub fn fifth(&self) -> U32Card {
        self.0[4]
    }

    #[must_use]
    pub fn to_arr(&self) -> [U32Card; 5] {
        self.0
    }
}

impl Evaluable for FiveCards {
    fn evaluate(&self) -> (FiveCards, HandRank) {
        (*self, Evaluate::five_cards(*self))
    }

    fn eval(&self) -> Eval {
        Eval::from(*self)
    }

    fn evals(&self) -> Evals {
        Evals::from(self.eval())
    }
}

impl Evaluable for Five {
    fn evaluate(&self) -> (FiveCards, HandRank) {
        let hand = FiveCards::from(self.to_arr());
        (hand, Evaluate::five_cards(hand))
    }

    fn eval(&self) -> Eval {
        Eval::from(FiveCards::from(self.to_arr()))
    }

    fn evals(&self) -> Evals {
        Evals::from(FiveCards::from(self.to_arr()).eval())
    }
}

impl Vectorable for Five {
    fn to_vec(&self) -> Vec<U32Card> {
        self.to_arr().to_vec()
    }
}

impl Vectorable for FiveCards {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.to_vec()
    }
}

impl fmt::Display for FiveCards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_poker_cards())
    }
}

impl From<[U32Card; 5]> for FiveCards {
    fn from(array: [U32Card; 5]) -> Self {
        FiveCards(array)
    }
}

impl From<[PlayingCard; 5]> for FiveCards {
    fn from(value: [PlayingCard; 5]) -> Self {
        FiveCards::from([
            value[0].as_u32(),
            value[1].as_u32(),
            value[2].as_u32(),
            value[3].as_u32(),
            value[4].as_u32(),
        ])
    }
}

impl From<Board> for FiveCards {
    fn from(board: Board) -> Self {
        FiveCards::from([
            board.flop.get_first_card().as_u32(),
            board.flop.get_second_card().as_u32(),
            board.flop.get_third_card().as_u32(),
            board.turn.get().as_u32(),
            board.river.get().as_u32(),
        ])
    }
}

impl TryFrom<cardpack::Pile> for FiveCards {
    type Error = HandError;

    fn try_from(pile: Pile) -> Result<Self, Self::Error> {
        FiveCards::try_from(&PokerCards::from(pile))
    }
}

impl TryFrom<&PokerCards> for FiveCards {
    type Error = HandError;

    fn try_from(value: &PokerCards) -> Result<Self, Self::Error> {
        FiveCards::try_from(value.to_vec())
    }
}

impl TryFrom<&'static str> for FiveCards {
    type Error = HandError;

    /// Returns a valid `CactusKevHand` if the entered index string splits out into exactly
    /// five valid `Card`
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_cards::FiveCards;
    ///
    /// let royal_flush = FiveCards::try_from("AS KS QS JS TS").unwrap();
    /// let s = format!("{}", royal_flush);
    ///
    /// assert_eq!(s, "A♠ K♠ Q♠ J♠ T♠");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `HandError::InvalidCard` error if it doesn't recognize the cards in the passed in
    /// index string:
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_cards::FiveCards;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCards::try_from("AR KE QS JS TS");
    ///
    /// assert!(invalid_hand.is_err());
    /// assert_eq!(invalid_hand.unwrap_err(), HandError::InvalidCard);
    /// ```
    ///
    /// Will return a `HandError::NotEnoughCards` if there are less than five cards passed in.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_cards::FiveCards;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCards::try_from("A♠ K♦ Q♣ J♥");
    ///
    /// assert!(invalid_hand.is_err());
    /// assert_eq!(invalid_hand.unwrap_err(), HandError::NotEnoughCards);
    /// ```
    ///
    /// Will return a `HandError::TooManyCards` if there are more than five cards passed in.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_cards::FiveCards;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCards::try_from("A♠ K♦ Q♣ J♥ T♦ 2♣");
    ///
    /// assert!(invalid_hand.is_err());
    /// assert_eq!(invalid_hand.unwrap_err(), HandError::TooManyCards);
    /// ```
    ///
    /// # Panics
    ///
    /// Shouldn't be able to panic. (fingers crossed)
    ///
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match PokerCards::try_from(value) {
            Ok(cards) => FiveCards::try_from(&cards),
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<Vec<U32Card>> for FiveCards {
    type Error = HandError;

    fn try_from(value: Vec<U32Card>) -> Result<Self, Self::Error> {
        match value.len() {
            0..=4 => Err(HandError::NotEnoughCards),
            5 => {
                let cards: [U32Card; 5] = value.try_into().unwrap_or_else(|v: Vec<U32Card>| {
                    panic!("Expected a Vec of length {} but it was {}", 5, v.len())
                });
                Ok(FiveCards::from(cards))
            }
            _ => Err(HandError::TooManyCards),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_arrays_five_cards_tests {
    use super::*;
    use crate::types::playing_card::*;
    use ckc_rs::CardNumber;

    #[test]
    fn sort() {
        let raw = [
            PlayingCard::FIVE_SPADES,
            PlayingCard::KING_SPADES,
            PlayingCard::ACE_CLUBS,
            PlayingCard::JACK_SPADES,
            PlayingCard::SEVEN_DIAMONDS,
        ];

        let expected = FiveCards([
            CardNumber::ACE_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
            CardNumber::FIVE_SPADES,
        ]);

        assert_eq!(FiveCards::from(raw).sort(), expected);
    }

    #[test]
    fn display() {
        let hand = FiveCards::try_from("QS AS KS JS T♠").unwrap();

        assert_eq!("Q♠ A♠ K♠ J♠ T♠", hand.to_string())
    }

    #[test]
    fn from__array_playing_cards() {
        let raw = [
            PlayingCard::FIVE_SPADES,
            PlayingCard::KING_SPADES,
            PlayingCard::ACE_CLUBS,
            PlayingCard::JACK_SPADES,
            PlayingCard::SEVEN_DIAMONDS,
        ];

        let expected = FiveCards([
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
        ]);

        assert_eq!(FiveCards::from(raw), expected);
    }

    #[test]
    fn from__array_poker_cards() {
        let raw = [
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
        ];

        let expected = FiveCards([
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
        ]);

        assert_eq!(FiveCards::from(raw), expected);
    }

    #[test]
    fn try_from__index() {
        let hand = FiveCards::try_from("A♠ K♠ Q♠ J♠ T♠");

        assert!(hand.is_ok());
    }

    #[test]
    fn try_from__index__not_enough_cards() {
        let hand = FiveCards::try_from("K♠ Q♠ J♠ T♠");

        assert!(hand.is_err());
        assert_eq!(hand.unwrap_err(), HandError::NotEnoughCards);
    }

    #[test]
    fn try_from__index__invalid_card() {
        let hand = FiveCards::try_from("AX K♠ Q♠ J♠ T♠");

        assert!(hand.is_err());
        assert_eq!(hand.unwrap_err(), HandError::InvalidCard);
    }

    #[test]
    fn try_from__poker_cards() {
        let poker_cards = PokerCards::try_from("AS KS QS JS TS").unwrap();

        let a = FiveCards::try_from(&poker_cards).unwrap();

        assert_eq!(poker_cards.get(0).unwrap(), &a.first());
        assert_eq!(poker_cards.get(1).unwrap(), &a.second());
        assert_eq!(poker_cards.get(2).unwrap(), &a.third());
        assert_eq!(poker_cards.get(3).unwrap(), &a.forth());
        assert_eq!(poker_cards.get(4).unwrap(), &a.fifth());
    }

    #[test]
    fn try_from__vector_poker_cards() {
        let raw = vec![
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
        ];

        let expected = FiveCards([
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::SEVEN_DIAMONDS,
        ]);

        assert_eq!(FiveCards::try_from(raw).unwrap(), expected);
    }

    #[test]
    fn try_from__vector_poker_cards__not_enough() {
        let raw = FiveCards::try_from(vec![
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
        ]);

        assert!(raw.is_err());
        assert_eq!(raw.unwrap_err(), HandError::NotEnoughCards);
    }

    #[test]
    fn try_from__vector_poker_cards__too_many() {
        let raw = FiveCards::try_from(vec![
            CardNumber::FIVE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::JACK_DIAMONDS,
            CardNumber::JACK_CLUBS,
        ]);

        assert!(raw.is_err());
        assert_eq!(raw.unwrap_err(), HandError::TooManyCards);
    }
}
