use crate::analysis::eval::Eval;
use crate::analysis::evals::Evals;
use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::three_card::ThreeCard;
use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::{Evaluable, Vectorable};
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use ckc_rs::{CKCNumber, CardNumber, HandError, PokerCard};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SixCard([CKCNumber; 6]);

impl SixCard {
    /// permutations to evaluate all 6 card combinations.
    pub const PERMUTATIONS: [[u8; 5]; 6] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 4, 5],
        [0, 1, 3, 4, 5],
        [0, 2, 3, 4, 5],
        [1, 2, 3, 4, 5],
    ];

    #[must_use]
    pub fn from_1_and_2_and_3(one: CKCNumber, two: TwoCard, three: ThreeCard) -> SixCard {
        SixCard::from([
            one,
            two.first(),
            two.second(),
            three.first(),
            three.second(),
            three.third(),
        ])
    }

    //region getters
    #[must_use]
    pub fn first(&self) -> CKCNumber {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> CKCNumber {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> CKCNumber {
        self.0[2]
    }

    #[must_use]
    pub fn forth(&self) -> CKCNumber {
        self.0[3]
    }

    #[must_use]
    pub fn fifth(&self) -> CKCNumber {
        self.0[4]
    }

    #[must_use]
    pub fn sixth(&self) -> CKCNumber {
        self.0[5]
    }
    //endregion
}

impl Evaluable for SixCard {
    fn evals(&self) -> Evals {
        let mut evals = Evals::default();
        let mut subhand: [CKCNumber; 5] = [CardNumber::BLANK; 5];

        for ids in &crate::types::arrays::six_card::SixCard::PERMUTATIONS {
            for i in 0..5 {
                subhand[i] = self.0[ids[i] as usize];
            }
            let (hand, eval) = FiveCard::from(subhand).evaluate();
            evals.push(Eval::raw(hand, eval));
        }
        evals
    }
}

impl From<[PlayingCard; 6]> for SixCard {
    fn from(array: [PlayingCard; 6]) -> Self {
        SixCard::from([
            array[0].as_u32(),
            array[1].as_u32(),
            array[2].as_u32(),
            array[3].as_u32(),
            array[4].as_u32(),
            array[5].as_u32(),
        ])
    }
}

impl From<[CKCNumber; 6]> for SixCard {
    fn from(value: [CKCNumber; 6]) -> Self {
        SixCard(value)
    }
}

impl TryFrom<&PlayingCards> for SixCard {
    type Error = HandError;

    fn try_from(playing_cards: &PlayingCards) -> Result<Self, Self::Error> {
        match playing_cards.len() {
            0..=5 => Err(HandError::NotEnoughCards),
            6 => Ok(SixCard::from([
                *playing_cards.get_index(0).unwrap(),
                *playing_cards.get_index(1).unwrap(),
                *playing_cards.get_index(2).unwrap(),
                *playing_cards.get_index(3).unwrap(),
                *playing_cards.get_index(4).unwrap(),
                *playing_cards.get_index(5).unwrap(),
            ])),
            _ => Err(HandError::TooManyCards),
        }
    }
}

impl TryFrom<&PokerCards> for SixCard {
    type Error = HandError;

    fn try_from(poker_cards: &PokerCards) -> Result<Self, Self::Error> {
        SixCard::try_from(&PlayingCards::from(poker_cards))
    }
}

impl TryFrom<&'static str> for SixCard {
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
            Ok(cards) => SixCard::try_from(&cards),
            Err(e) => Err(e),
        }
    }
}

impl Vectorable for SixCard {
    #[must_use]
    fn to_vec(&self) -> Vec<CKCNumber> {
        self.0.to_vec()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_arrays_six_card_tests {
    use super::*;
    use ckc_rs::hand_rank::HandRankClass;
    use rstest::rstest;

    #[test]
    fn eval() {
        let ckcs = SixCard::try_from("6H AH KH QH JH TH").unwrap().eval();

        assert_eq!(ckcs.rank.class, HandRankClass::RoyalFlush);
    }

    #[rstest]
    #[case("9H AH KH QH JH TH", "AH KH QH JH TH")]
    #[case("9H AH KS QH JD TH", "A♥ K♠ Q♥ J♦ T♥")]
    #[case("9H AH KS QH JD TH", "A♥ K♠ Q♥ J♦ T♥")]
    #[case("9H TD KS QH 9D TH", "TH TD 9H 9D KS")]
    fn eval__many(#[case] index: &'static str, #[case] best_index: &'static str) {
        let hand = SixCard::try_from(index).unwrap().eval();

        let expected = FiveCard::try_from(best_index).unwrap();

        assert_eq!(hand.hand, expected);
    }

    #[test]
    fn try_from__poker_cards() {
        let poker_cards = PokerCards::try_from("9H AS KS QS JS TS").unwrap();

        let a = SixCard::try_from(&poker_cards).unwrap();

        assert_eq!(*poker_cards.get(0).unwrap(), a.first());
        assert_eq!(*poker_cards.get(1).unwrap(), a.second());
        assert_eq!(*poker_cards.get(2).unwrap(), a.third());
        assert_eq!(*poker_cards.get(3).unwrap(), a.forth());
        assert_eq!(*poker_cards.get(4).unwrap(), a.fifth());
        assert_eq!(*poker_cards.get(5).unwrap(), a.sixth());
    }
}
