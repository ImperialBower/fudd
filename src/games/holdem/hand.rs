use crate::types::arrays::Vectorable;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::{PileOfCards, U32Card};
use ckc_rs::cards::two::Two;
use ckc_rs::PokerCard;
use log::warn;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Hand(PlayingCard, PlayingCard);

impl Hand {
    #[must_use]
    pub fn new(first: PlayingCard, second: PlayingCard) -> Hand {
        if !Hand::is_valid(Hand(first, second)) {
            warn!("Invalid HoldemHand: {} {}", first, second);
            return Hand::blank();
        }
        if first > second {
            Hand(first, second)
        } else {
            Hand(second, first)
        }
    }

    #[must_use]
    pub fn blank() -> Hand {
        Hand(PlayingCard::default(), PlayingCard::default())
    }

    //region accessors
    #[must_use]
    pub fn first(&self) -> PlayingCard {
        self.0
    }

    #[must_use]
    pub fn second(&self) -> PlayingCard {
        self.1
    }
    //endregion accessors

    #[must_use]
    pub fn as_playing_cards(&self) -> PlayingCards {
        PlayingCards::from(vec![self.0, self.1])
    }

    #[must_use]
    pub fn as_two(&self) -> Two {
        Two::new(self.first().as_u32(), self.second().as_u32())
    }

    // #[must_use]
    // pub fn contains(&self, card: PlayingCard) -> bool {
    //     (self.0 == card) || (self.1 == card)
    // }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.has(PlayingCard::BLANK)
    }

    #[must_use]
    pub fn is_compatible(&self, other: &Hand) -> bool {
        !self.has(other.first())
            && !self.has(other.second())
            && !self.is_blank()
            && !other.is_blank()
    }

    #[must_use]
    fn is_valid(hand: Hand) -> bool {
        !hand.first().is_blank() && !hand.second().is_blank() && hand.first() != hand.second()
    }
}

impl PileOfCards<PlayingCard> for Hand {
    fn has(&self, playing_card: PlayingCard) -> bool {
        (self.0 == playing_card) || (self.1 == playing_card)
    }
}

impl PileOfCards<U32Card> for Hand {
    fn has(&self, card_number: U32Card) -> bool {
        (self.0.as_u32() == card_number) || (self.1.as_u32() == card_number)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl From<&'static str> for Hand {
    fn from(s: &'static str) -> Self {
        let v: Vec<&str> = s.split_whitespace().collect();
        if v.len() == 2 {
            Hand::new(
                PlayingCard::from(*v.first().unwrap()),
                PlayingCard::from(*v.get(1).unwrap()),
            )
        } else {
            Hand::default()
        }
    }
}

impl Vectorable for Hand {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        vec![self.first().as_u32(), self.second().as_u32()]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_hands_holdem_hand {
    use super::*;
    use ckc_rs::CardNumber;

    #[test]
    fn new() {
        let inverted = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);
        let same = Hand::new(PlayingCard::ACE_SPADES, PlayingCard::KING_SPADES);

        let expected = Hand::new(PlayingCard::ACE_SPADES, PlayingCard::KING_SPADES);

        assert_eq!(inverted, expected);
        assert_eq!(same, expected);
    }

    #[test]
    fn new__blank() {
        assert!(Hand::new(PlayingCard::default(), PlayingCard::ACE_SPADES).is_blank());
        assert!(Hand::new(PlayingCard::ACE_SPADES, PlayingCard::default()).is_blank());
        assert!(Hand::new(PlayingCard::ACE_SPADES, PlayingCard::ACE_SPADES).is_blank());
        assert!(Hand::new(PlayingCard::default(), PlayingCard::default()).is_blank());
        assert!(!Hand::new(PlayingCard::ACE_SPADES, PlayingCard::KING_SPADES).is_blank());
    }

    #[test]
    fn accessors() {
        let actual = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);

        assert_eq!(actual.first(), PlayingCard::ACE_SPADES);
        assert_eq!(actual.second(), PlayingCard::KING_SPADES);
    }

    #[test]
    fn as_two() {
        let hand = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);

        let expected = Two::new(CardNumber::ACE_SPADES, CardNumber::KING_SPADES);

        assert_eq!(hand.as_two(), expected);
    }

    #[test]
    fn contains() {
        let hand = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);

        assert!(hand.has(CardNumber::ACE_SPADES));
        assert!(hand.has(PlayingCard::KING_SPADES));
        assert!(!hand.has(PlayingCard::QUEEN_SPADES));
    }

    #[test]
    fn is_blank() {
        assert!(Hand::blank().is_blank());
    }

    #[test]
    fn is_compatible() {
        let hand = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);

        assert!(hand.is_compatible(&Hand::new(
            PlayingCard::ACE_CLUBS,
            PlayingCard::KING_DIAMONDS
        )));
        assert!(!hand.is_compatible(&Hand::new(
            PlayingCard::ACE_SPADES,
            PlayingCard::KING_SPADES
        )));
        assert!(!hand.is_compatible(&Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::KING_SPADES)));
        assert!(!hand.is_compatible(&Hand::new(
            PlayingCard::ACE_SPADES,
            PlayingCard::KING_DIAMONDS
        )));
        assert!(!hand.is_compatible(&Hand::new(PlayingCard::ACE_SPADES, PlayingCard::ACE_SPADES)));
    }

    #[test]
    fn fmt__to_string() {
        let actual = Hand::new(PlayingCard::KING_SPADES, PlayingCard::ACE_SPADES);

        assert_eq!(actual.to_string(), "A♠ K♠");
    }
}
