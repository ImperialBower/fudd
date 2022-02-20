use crate::types::arrays::five_cards::FiveCards;
use crate::types::arrays::{Evaluable, Vectorable};
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use serde::{Deserialize, Serialize};

use cardpack::Pile;
use ckc_rs::hand_rank::{HandRank, HandRankClass};
use ckc_rs::HandError;
use std::cmp::Ordering;
use std::fmt;

/// `Eval` is a struct made up of `FiveCards` and its `HandRank`. It
/// performs a poker style sort on the `FiveCards` hand, giving priority to the most
/// valuable collection of cards.
///
/// The [cardpack.rs](https://github.com/ContractBridge/cardpack.rs) library supports two
/// kinds of sorting for a `cardpack::Pile` of `Cards`:
///
/// `Pile.sort()`, which prioritizes `Suit` over `Rank`:
///
/// ```
/// use std::convert::TryFrom;
/// use ckc_rs::PokerCard;
/// use fudd::types::arrays::Vectorable;
/// use fudd::types::arrays::five_cards::FiveCards;
/// use fudd::types::poker_cards::PokerCards;
///
/// let full_house = FiveCards::try_from("AS AH AD KS KH").unwrap();
/// let pile = full_house.to_pile().sort();
///
/// let s = format!("{}", PokerCards::from(pile));
///
/// assert_eq!("A♠ K♠ A♥ K♥ A♦", s);
/// ```
///
/// And `Pile.sort_by_frequency()`, which prioritizes how many of the same
/// `Rank` there are over `Suit`:
///
/// ```
/// use std::convert::TryFrom;
/// use fudd::types::arrays::five_cards::FiveCards;
/// use fudd::types::arrays::Vectorable;
/// use fudd::types::poker_cards::PokerCards;
///
/// let full_house = FiveCards::try_from("AS AH AD KS KH").unwrap();
/// let pile = full_house.to_pile().sort_by_frequency();
///
/// let s = format!("{}", PokerCards::from(pile));
///
/// assert_eq!("A♠ A♥ A♦ K♠ K♥", s);
/// ```
///
/// `Pile.sort_by_frequency()` works for sorting every kind of traditional poker hand except a
/// wheel, where the ace needs to be at the end of the hand:
///
/// ```
/// use std::convert::TryFrom;
/// use fudd::types::arrays::Vectorable;
/// use fudd::types::arrays::five_cards::FiveCards;
/// use fudd::types::poker_cards::PokerCards;
///
/// let wheel = FiveCards::try_from("AS 2S 3S 4S 5C").unwrap();
/// let pile = wheel.to_pile().sort_by_frequency();
///
/// let s = format!("{}", PokerCards::from(pile));
///
/// assert_eq!("A♠ 5♣ 4♠ 3♠ 2♠", s);
/// ```
///
/// `EvaluatedHand` deals with this specific exception:
///
/// ```
/// use std::convert::TryFrom;
/// use fudd::analysis::eval::Eval;
///
/// let wheel = Eval::try_from("AS 2S 3S 4S 5C").unwrap();
///
/// assert_eq!("5♣ 4♠ 3♠ 2♠ A♠", wheel.hand.to_string());
/// ```
///
/// # Panics
///
/// Shouldn't be possible.
///
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Eval {
    pub rank: HandRank,
    pub hand: FiveCards,
}

impl Eval {
    #[must_use]
    pub fn new(hand: FiveCards, rank: HandRank) -> Eval {
        Eval {
            hand: Eval::sort(hand, rank),
            rank,
        }
    }

    #[must_use]
    pub fn raw(hand: FiveCards, rank: HandRank) -> Eval {
        Eval { rank, hand }
    }

    #[must_use]
    pub fn sorted(&self) -> Eval {
        Eval::new(self.hand, self.rank)
    }

    /// TODO: This is really, really slow and could be improved.
    /// We need to create a frequency sort with just `PokerCards`
    ///
    /// The [cardpile.rs](https://github.com/ContractBridge/cardpack.rs) library
    /// supports the ability to sort a `Pile` of cards giving priority to its `Rank`,
    /// its `Suit`. or the frequency with which a specific Rank appears. This allows
    /// us to generate displays of hands that highlight their strengths.
    ///
    /// The only thing that isn't covered is a wheel, which we handle here.
    #[must_use]
    fn sort(five_cards: FiveCards, hand_rank: HandRank) -> FiveCards {
        match hand_rank.class {
            // If it's a wheel we need to do some work.
            HandRankClass::FiveHighStraight | HandRankClass::FiveHighStraightFlush => {
                let mut pile = five_cards.to_pile().convert_to_rank_weighted();
                pile.sort_in_place();
                let ace = pile.draw_first().unwrap();
                pile.push(ace);
                FiveCards::try_from(pile).unwrap()
            }
            _ => {
                let pile = five_cards.to_pile().sort_by_frequency();
                FiveCards::try_from(pile).unwrap()
            }
        }
    }

    #[must_use]
    pub fn to_pile(&self) -> Pile {
        PokerCards::from(self.hand.to_vec()).to_pile()
    }

    #[must_use]
    pub fn to_poker_cards(&self) -> PokerCards {
        PokerCards::from(self.hand.to_vec())
    }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.hand, self.rank)
    }
}

impl From<[U32Card; 5]> for Eval {
    fn from(array: [U32Card; 5]) -> Self {
        Eval::from(FiveCards::from(array))
    }
}

impl From<FiveCards> for Eval {
    fn from(five_cards: FiveCards) -> Self {
        let (_, hand_rank) = five_cards.evaluate();

        Eval {
            hand: Eval::sort(five_cards, hand_rank),
            rank: hand_rank,
        }
    }
}

impl PartialOrd<Self> for Eval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The lower the `HandRankValue` the higher the value of the `HandRank`, unless it's invalid.
#[allow(clippy::if_same_then_else)]
impl Ord for Eval {
    fn cmp(&self, other: &Eval) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl TryFrom<&PokerCards> for Eval {
    type Error = HandError;

    fn try_from(value: &PokerCards) -> Result<Self, Self::Error> {
        match FiveCards::try_from(value) {
            Ok(cards) => Ok(Eval::from(cards)),
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<&'static str> for Eval {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match FiveCards::try_from(index) {
            Ok(five_cards) => Ok(Eval::from(five_cards)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_tests {
    use super::*;
    use ckc_rs::CardNumber;
    use rstest::rstest;

    #[test]
    fn sort() {
        let hand = Eval::try_from("K♠ A♣ Q♠ J♠ T♠").unwrap();
        let ex = FiveCards::try_from("A♣ K♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(hand.hand, ex);
    }

    #[rstest]
    #[case("A♠ A♦ K♠ K♦ K♥", "K♠ K♥ K♦ A♠ A♦")]
    #[case("2♣ 4♣ 3♣ A♣ 5♣", "5♣ 4♣ 3♣ 2♣ A♣")]
    #[case("2♣ 4♣ 3♣ 6♣ 5♣", "6♣ 5♣ 4♣ 3♣ 2♣")]
    #[case("2♣ 4♣ 3♣ A♠ 5♣", "5♣ 4♣ 3♣ 2♣ A♠")]
    #[case("K♠ A♥ A♦ A♣ A♠", "A♠ A♥ A♦ A♣ K♠")]
    #[case("3♠ 2♠ 2♦ 2♥ 2♣", "2♠ 2♥ 2♦ 2♣ 3♠")]
    #[case("K♦ K♠ A♦ A♠ A♥", "A♠ A♥ A♦ K♠ K♦")]
    #[case("3♠ 2♥ 2♠ 2♦ 3♦", "2♠ 2♥ 2♦ 3♠ 3♦")]
    #[case("9♠ A♠ K♠ Q♠ J♠", "A♠ K♠ Q♠ J♠ 9♠")]
    #[case("2♣ 3♣ 4♣ 5♣ 7♣", "7♣ 5♣ 4♣ 3♣ 2♣")]
    #[case("K♠ A♣ Q♠ J♠ T♠", "A♣ K♠ Q♠ J♠ T♠")]
    #[case("Q♠ A♣ K♠ T♠ J♠", "A♣ K♠ Q♠ J♠ T♠")]
    #[case("A♥ 2♣ 3♣ 4♣ 5♣", "5♣ 4♣ 3♣ 2♣ A♥")]
    #[case("K♠ A♦ A♥ Q♣ A♠", "A♠ A♥ A♦ K♠ Q♣")]
    #[case("2♠ 2♦ 2♥ 3♠ 4♣", "2♠ 2♥ 2♦ 4♣ 3♠")]
    #[case("K♦ Q♣ K♠ A♥ A♠", "A♠ A♥ K♠ K♦ Q♣")]
    #[case("3♠ 3♥ 2♦ 2♠ 4♣", "3♠ 3♥ 2♠ 2♦ 4♣")]
    #[case("K♠ A♠ J♠ A♥ Q♠", "A♠ A♥ K♠ Q♠ J♠")]
    #[case("2♠ 2♥ 3♠ 4♠ 5♠", "2♠ 2♥ 5♠ 4♠ 3♠")]
    #[case("Q♠ K♠ 9♣ A♠ J♠", "A♠ K♠ Q♠ J♠ 9♣")]
    #[case("2♣ 3♣ 4♣ 5♥ 7♣", "7♣ 5♥ 4♣ 3♣ 2♣")]
    #[case("A♠ 5♣ 4♣ 3♠ 2♠", "5♣ 4♣ 3♠ 2♠ A♠")]
    #[case("5♣ 4♣ 3♠ 2♠ 7H", "7♥ 5♣ 4♣ 3♠ 2♠")]
    #[case("K♥ K♣ A♦ 6♥ 6♦", "K♥ K♣ 6♥ 6♦ A♦")]
    #[case("A♠ K♠ T♠ Q♠ J♠", "A♠ K♠ Q♠ J♠ T♠")]
    #[case("K♣ K♥ A♦ 6♦ 6♥", "K♥ K♣ 6♥ 6♦ A♦")]
    fn sort_many(#[case] index: &'static str, #[case] expected: &'static str) {
        let hand = Eval::try_from(index).unwrap();
        let ex = FiveCards::try_from(expected).unwrap();
        let index = FiveCards::try_from(index).unwrap();

        assert_eq!(hand.hand, ex);
        assert_ne!(index, ex);
    }

    #[test]
    fn display() {
        let hand = Eval::try_from("A♠ A♦ K♠ K♦ K♥").unwrap();

        assert_eq!(
            "K♠ K♥ K♦ A♠ A♦ HandRank { value: 179, name: FullHouse, class: KingsOverAces }",
            hand.to_string()
        );
    }

    #[test]
    fn from__poker_card_array() {
        let raw = [
            CardNumber::JACK_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::KING_DIAMONDS,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
        ];

        let expected = Eval {
            hand: FiveCards::from([
                CardNumber::KING_SPADES,
                CardNumber::KING_DIAMONDS,
                CardNumber::JACK_SPADES,
                CardNumber::JACK_CLUBS,
                CardNumber::ACE_CLUBS,
            ]),
            rank: HandRank::from(2611),
        };

        let actual = Eval::from(raw);

        assert_eq!(actual, expected);
    }

    #[test]
    fn from__five_cards() {
        let raw = FiveCards::from([
            CardNumber::JACK_CLUBS,
            CardNumber::JACK_SPADES,
            CardNumber::KING_DIAMONDS,
            CardNumber::KING_SPADES,
            CardNumber::ACE_CLUBS,
        ]);

        let expected = Eval {
            hand: FiveCards::from([
                CardNumber::KING_SPADES,
                CardNumber::KING_DIAMONDS,
                CardNumber::JACK_SPADES,
                CardNumber::JACK_CLUBS,
                CardNumber::ACE_CLUBS,
            ]),
            rank: HandRank::from(2611),
        };

        let actual = Eval::from(raw);

        assert_eq!(actual, expected);
    }
}
