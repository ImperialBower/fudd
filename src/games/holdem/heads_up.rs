use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::Vectorable;
use crate::types::U32Card;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct HeadsUp {
    pub first: TwoCard,
    pub second: TwoCard,
}

impl HeadsUp {
    #[must_use]
    pub fn new(first: TwoCard, second: TwoCard) -> HeadsUp {
        if first > second {
            HeadsUp { first, second }
        } else {
            HeadsUp {
                first: second,
                second: first,
            }
        }
    }
}

impl Vectorable for HeadsUp {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        vec![
            self.first.first(),
            self.first.second(),
            self.second.first(),
            self.second.second(),
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod games_holdem_heads_up_tests {
    use super::*;
    use ckc_rs::CardNumber;

    #[test]
    fn new() {
        let aces = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES).unwrap();
        let kq = TwoCard::new(CardNumber::QUEEN_DIAMONDS, CardNumber::KING_DIAMONDS).unwrap();

        let headsup = HeadsUp::new(kq, aces);

        assert_eq!(
            headsup.first,
            TwoCard::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).unwrap()
        );
        assert_eq!(
            headsup.second,
            TwoCard::new(CardNumber::KING_DIAMONDS, CardNumber::QUEEN_DIAMONDS).unwrap()
        );
        assert_eq!(
            headsup,
            HeadsUp {
                first: TwoCard::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).unwrap(),
                second: TwoCard::new(CardNumber::KING_DIAMONDS, CardNumber::QUEEN_DIAMONDS)
                    .unwrap()
            }
        )
    }

    #[test]
    fn to_vec() {
        let aces = TwoCard::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES).unwrap();
        let kq = TwoCard::new(CardNumber::QUEEN_DIAMONDS, CardNumber::KING_DIAMONDS).unwrap();

        let headsup = HeadsUp::new(kq, aces);

        assert_eq!(
            headsup.to_vec(),
            vec![
                CardNumber::ACE_SPADES,
                CardNumber::ACE_CLUBS,
                CardNumber::KING_DIAMONDS,
                CardNumber::QUEEN_DIAMONDS
            ]
        );
    }
}
