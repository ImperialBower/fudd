use crate::analysis::eval::Eval;
use crate::analysis::evals::Evals;
use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::{Evaluable, Vectorable};
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use ckc_rs::{CardNumber, HandError, PokerCard};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct SevenCard([U32Card; 7]);

impl SevenCard {
    #[must_use]
    pub fn new(two: TwoCard, five: FiveCard) -> SevenCard {
        SevenCard([
            two.first(),
            two.second(),
            five.first(),
            five.second(),
            five.third(),
            five.forth(),
            five.fifth(),
        ])
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
    pub fn sixth(&self) -> U32Card {
        self.0[5]
    }

    #[must_use]
    pub fn seventh(&self) -> U32Card {
        self.0[6]
    }
    //endregion

    #[must_use]
    pub fn to_arr(&self) -> [U32Card; 7] {
        self.0
    }
}

impl Evaluable for SevenCard {
    fn evals(&self) -> Evals {
        let mut evals = Evals::default();
        let mut subhand: [U32Card; 5] = [CardNumber::BLANK; 5];

        for ids in &ckc_rs::cards::seven::Seven::FIVE_CARD_PERMUTATIONS {
            for i in 0..5 {
                subhand[i] = self.0[ids[i] as usize];
            }
            let (hand, eval) = FiveCard::from(subhand).evaluate();
            evals.push(Eval::raw(hand, eval));
        }
        evals
    }
}

impl From<[PlayingCard; 7]> for SevenCard {
    fn from(array: [PlayingCard; 7]) -> Self {
        SevenCard::from([
            array[0].as_u32(),
            array[1].as_u32(),
            array[2].as_u32(),
            array[3].as_u32(),
            array[4].as_u32(),
            array[5].as_u32(),
            array[6].as_u32(),
        ])
    }
}

impl From<[U32Card; 7]> for SevenCard {
    fn from(array: [U32Card; 7]) -> Self {
        SevenCard(array)
    }
}

impl TryFrom<&PlayingCards> for SevenCard {
    type Error = HandError;

    fn try_from(playing_cards: &PlayingCards) -> Result<Self, Self::Error> {
        SevenCard::try_from(&PokerCards::from(playing_cards))
    }
}

impl TryFrom<&PokerCards> for SevenCard {
    type Error = HandError;

    fn try_from(poker_cards: &PokerCards) -> Result<Self, Self::Error> {
        match poker_cards.len() {
            0..=6 => Err(HandError::NotEnoughCards),
            7 => Ok(SevenCard::from([
                *poker_cards.get(0).unwrap(),
                *poker_cards.get(1).unwrap(),
                *poker_cards.get(2).unwrap(),
                *poker_cards.get(3).unwrap(),
                *poker_cards.get(4).unwrap(),
                *poker_cards.get(5).unwrap(),
                *poker_cards.get(6).unwrap(),
            ])),
            _ => Err(HandError::TooManyCards),
        }
    }
}

impl TryFrom<&'static str> for SevenCard {
    type Error = HandError;

    /// Returns a valid `CactusKevHand` if the entered index string splits out into exactly
    /// five valid `Card`
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_card::FiveCard;
    ///
    /// let royal_flush = FiveCard::try_from("AS KS QS JS TS").unwrap();
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
    /// use fudd::types::arrays::five_card::FiveCard;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCard::try_from("AR KE QS JS TS");
    ///
    /// assert!(invalid_hand.is_err());
    /// assert_eq!(invalid_hand.unwrap_err(), HandError::InvalidCard);
    /// ```
    ///
    /// Will return a `HandError::NotEnoughCards` if there are less than five cards passed in.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_card::FiveCard;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCard::try_from("A♠ K♦ Q♣ J♥");
    ///
    /// assert!(invalid_hand.is_err());
    /// assert_eq!(invalid_hand.unwrap_err(), HandError::NotEnoughCards);
    /// ```
    ///
    /// Will return a `HandError::TooManyCards` if there are more than five cards passed in.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use fudd::types::arrays::five_card::FiveCard;
    /// use ckc_rs::HandError;
    ///
    /// let invalid_hand = FiveCard::try_from("A♠ K♦ Q♣ J♥ T♦ 2♣");
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
            Ok(cards) => SevenCard::try_from(&cards),
            Err(e) => Err(e),
        }
    }
}

impl Vectorable for SevenCard {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.to_vec()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_arrays_seven_card_tests {
    use super::*;
    use ckc_rs::hand_rank::HandRank;
    use ckc_rs::CardNumber;

    #[test]
    fn from__array() {
        let expected = SevenCard([
            CardNumber::ACE_CLUBS,
            CardNumber::KING_CLUBS,
            CardNumber::QUEEN_CLUBS,
            CardNumber::JACK_CLUBS,
            CardNumber::TEN_CLUBS,
            CardNumber::NINE_CLUBS,
            CardNumber::EIGHT_CLUBS,
        ]);

        let actual = SevenCard::from([
            CardNumber::ACE_CLUBS,
            CardNumber::KING_CLUBS,
            CardNumber::QUEEN_CLUBS,
            CardNumber::JACK_CLUBS,
            CardNumber::TEN_CLUBS,
            CardNumber::NINE_CLUBS,
            CardNumber::EIGHT_CLUBS,
        ]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn eval() {
        let five_cards = FiveCard::try_from("AC KC QC JC TC").unwrap();
        let two_cards = TwoCard::try_from("9C 8C").unwrap();

        let seven = SevenCard::new(two_cards, five_cards);

        assert_eq!(five_cards.eval(), seven.eval());
    }

    #[test]
    fn evaluable() {
        let five_cards = FiveCard::try_from("AC KC QC JC TC").unwrap();
        let two_cards = TwoCard::try_from("9C 8C").unwrap();

        let seven = SevenCard::new(two_cards, five_cards);

        let (hand, hand_rank) = seven.evaluate();

        assert_eq!(HandRank::from(1), hand_rank);
        assert_eq!(five_cards, hand);
    }

    #[test]
    fn try_from__playing_cards() {
        let playing_cards = PlayingCards::try_from("9H AS KS QS JS TS 2H").unwrap();

        let a = SevenCard::try_from(&playing_cards).unwrap();

        assert_eq!(playing_cards.get_index(0).unwrap().as_u32(), a.first());
        assert_eq!(playing_cards.get_index(1).unwrap().as_u32(), a.second());
        assert_eq!(playing_cards.get_index(2).unwrap().as_u32(), a.third());
        assert_eq!(playing_cards.get_index(3).unwrap().as_u32(), a.forth());
        assert_eq!(playing_cards.get_index(4).unwrap().as_u32(), a.fifth());
        assert_eq!(playing_cards.get_index(5).unwrap().as_u32(), a.sixth());
        assert_eq!(playing_cards.get_index(6).unwrap().as_u32(), a.seventh());
    }

    #[test]
    fn try_from__poker_cards() {
        let poker_cards = PokerCards::try_from("9H AS KS QS JS TS 2H").unwrap();

        let a = SevenCard::try_from(&poker_cards).unwrap();

        assert_eq!(*poker_cards.get(0).unwrap(), a.first());
        assert_eq!(*poker_cards.get(1).unwrap(), a.second());
        assert_eq!(*poker_cards.get(2).unwrap(), a.third());
        assert_eq!(*poker_cards.get(3).unwrap(), a.forth());
        assert_eq!(*poker_cards.get(4).unwrap(), a.fifth());
        assert_eq!(*poker_cards.get(5).unwrap(), a.sixth());
        assert_eq!(*poker_cards.get(6).unwrap(), a.seventh());
    }
}
