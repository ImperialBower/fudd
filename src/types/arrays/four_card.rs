use crate::analysis::eval::Eval;
use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::six_card::SixCard;
use crate::types::arrays::three_card::ThreeCard;
use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::{Evaluable, Vectorable};
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use crate::types::slots::flop::Flop;
use crate::types::slots::omaha_hand::OmahaHand;
use ckc_rs::hand_rank::{HandRank, HandRankName};
use ckc_rs::{CKCNumber, HandError, PokerCard};
use std::fmt;

/// [Omaha hold 'em](https://en.wikipedia.org/wiki/Omaha_hold_%27em) starting
/// hand.
///
/// **TODO:** Count [straight outs](https://en.wikipedia.org/wiki/Omaha_hold_%27em#Wraps)
///
/// * [Pot-Limit Omaha: What Are the Best Starting Hands?](https://www.pokerlistings.com/strategy/potlimit-omaha-starting-hands)
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FourCard(pub [CKCNumber; 4]);

impl FourCard {
    /// permutations to evaluate all `Omaha` style combinations.
    pub const PERMUTATIONS: [[u8; 2]; 6] = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];

    pub fn best_at_flop(&self, flop: &Flop) -> Eval {
        let three = ThreeCard::from(flop);
        let mut best_hand = FiveCard::default();
        let mut best_rank = HandRank::default();
        for i in 0..FourCard::PERMUTATIONS.len() {
            let hand = FiveCard::from_2_and_3(self.permutation(i), three);
            let (_, rank) = hand.evaluate();
            if rank > best_rank {
                best_rank = rank;
                best_hand = hand;
            }
        }
        Eval::new(best_hand, best_rank)
    }

    /// Calculates the straight outs for a particular flop.
    pub fn straight_outs_at_flop(&self, flop: &Flop) -> PlayingCards {
        // This feels sick to me
        let mut outs = PlayingCards::default();
        let three = ThreeCard::from(flop);

        for card in self.remaining().to_vec() {
            for i in 0..FourCard::PERMUTATIONS.len() {
                let hand = SixCard::from_1_and_2_and_3(card.as_u32(), self.permutation(i), three);
                let evals = hand.evals();
                for e in evals.to_vec() {
                    if e.hand.contains(&card.as_u32())
                        && (e.rank.name == HandRankName::Straight
                            || e.rank.name == HandRankName::StraightFlush)
                    {
                        outs.insert(card);
                    }
                }
            }
        }

        outs
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
    //endregion

    #[must_use]
    pub fn is_double_suited(&self) -> bool {
        self.suit_count() == 2
    }

    #[must_use]
    pub fn permutation(&self, i: usize) -> TwoCard {
        if i >= FourCard::PERMUTATIONS.len() {
            TwoCard::default()
        } else {
            TwoCard::from([
                self.0[FourCard::PERMUTATIONS[i][0] as usize],
                self.0[FourCard::PERMUTATIONS[i][1] as usize],
            ])
        }
    }

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 4] {
        self.0
    }
}

impl fmt::Display for FourCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_poker_cards())
    }
}

impl From<OmahaHand> for FourCard {
    fn from(hand: OmahaHand) -> Self {
        FourCard::from(hand.to_array())
    }
}

impl From<[PlayingCard; 4]> for FourCard {
    fn from(array: [PlayingCard; 4]) -> Self {
        FourCard::from([
            array[0].as_u32(),
            array[1].as_u32(),
            array[2].as_u32(),
            array[3].as_u32(),
        ])
    }
}

impl From<[CKCNumber; 4]> for FourCard {
    fn from(value: [CKCNumber; 4]) -> Self {
        FourCard(value)
    }
}

impl TryFrom<&PokerCards> for FourCard {
    type Error = HandError;

    fn try_from(value: &PokerCards) -> Result<Self, Self::Error> {
        FourCard::try_from(value.to_vec())
    }
}

impl TryFrom<&'static str> for FourCard {
    type Error = HandError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match PokerCards::try_from(value) {
            Ok(cards) => FourCard::try_from(&cards),
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<Vec<CKCNumber>> for FourCard {
    type Error = HandError;

    fn try_from(value: Vec<CKCNumber>) -> Result<Self, Self::Error> {
        match value.len() {
            0..=3 => Err(HandError::NotEnoughCards),
            4 => {
                let cards: [CKCNumber; 4] = value.try_into().unwrap_or_else(|v: Vec<CKCNumber>| {
                    panic!("Expected a Vec of length {} but it was {}", 4, v.len())
                });
                Ok(FourCard::from(cards))
            }
            _ => Err(HandError::TooManyCards),
        }
    }
}

impl Vectorable for FourCard {
    #[must_use]
    fn to_vec(&self) -> Vec<CKCNumber> {
        self.0.to_vec()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_arrays_four_card_tests {
    use super::*;
    use ckc_rs::hand_rank::HandRankClass;

    #[test]
    fn best_at_flop() {
        let four = FourCard::try_from("KS AH AS KH").unwrap();
        let flop = Flop::try_from("TS JS QS").unwrap();

        assert_eq!(
            four.best_at_flop(&flop).rank.class,
            HandRankClass::RoyalFlush
        );
    }

    #[test]
    fn is_double_suited() {
        assert!(FourCard::try_from("KS AH AS KH")
            .unwrap()
            .is_double_suited());
        assert!(FourCard::try_from("KD 8C 9D TC")
            .unwrap()
            .is_double_suited());
        assert!(!FourCard::try_from("KS 8C 9D TC")
            .unwrap()
            .is_double_suited());
    }

    #[test]
    fn permutation() {
        let four = FourCard::try_from("5♠ Q♥ 6♠ AC").unwrap();

        assert_eq!("5♠ Q♥", four.permutation(0).to_string());
        assert_eq!("5♠ 6♠", four.permutation(1).to_string());
        assert_eq!("5♠ A♣", four.permutation(2).to_string());
        assert_eq!("Q♥ 6♠", four.permutation(3).to_string());
        assert_eq!("Q♥ A♣", four.permutation(4).to_string());
        assert_eq!("6♠ A♣", four.permutation(5).to_string());
        assert_eq!(TwoCard::default(), four.permutation(6));
    }

    #[test]
    fn remaining() {
        let four = FourCard::try_from("AS KS QS JS").unwrap();

        let remaining = four.remaining();

        assert_eq!(remaining.len(), 48);
        assert!(!remaining.contains(&PlayingCard::from("AS")));
        assert!(!remaining.contains(&PlayingCard::from("KS")));
        assert!(!remaining.contains(&PlayingCard::from("QS")));
        assert!(!remaining.contains(&PlayingCard::from("JS")));
        assert!(remaining.contains(&PlayingCard::from("TS")));
    }
}
