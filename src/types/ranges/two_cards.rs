use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::Vectorable;
use crate::types::poker_deck::POKER_DECK;
use crate::types::ranges::two_cards_set::TwoCardsSet;
use crate::types::sample::Sample;
use crate::types::U32Card;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct TwoCards {
    pub hands: Vec<TwoCard>,
}

impl TwoCards {
    #[must_use]
    pub fn all() -> TwoCards {
        TwoCards::from(
            POKER_DECK
                .combinations(2)
                .map(TwoCard::from)
                .collect::<Vec<TwoCard>>(),
        )
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TwoCard> {
        self.hands.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hands.is_empty()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, TwoCard> {
        self.hands.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.hands.len()
    }

    #[must_use]
    pub fn pairs(&self) -> TwoCards {
        let pairs: Vec<TwoCard> = self
            .hands
            .clone()
            .into_iter()
            .filter(TwoCard::is_pocket_pair)
            .collect();
        TwoCards::from(pairs)
    }

    pub fn pop(&mut self) -> Option<TwoCard> {
        self.hands.pop()
    }

    #[must_use]
    pub fn position(&self, two_cards: TwoCard) -> Option<usize> {
        self.hands.iter().position(|&r| r == two_cards)
    }

    pub fn push(&mut self, two_cards: TwoCard) {
        self.hands.push(two_cards);
    }

    pub fn sample(&mut self) -> Option<TwoCard> {
        self.hands.sample()
    }

    /// Sorts the vector in place..
    pub fn sort(&mut self) {
        self.hands.sort_unstable();
        self.hands.reverse();
    }

    #[must_use]
    pub fn suited(&self) -> TwoCards {
        TwoCards::from(
            self.hands
                .clone()
                .into_iter()
                .filter(TwoCard::is_suited)
                .collect::<Vec<TwoCard>>(),
        )
    }

    /// Returns a sorted vector of the range.
    #[must_use]
    pub fn sorted(&self) -> TwoCards {
        let mut v = self.clone();
        v.sort();
        v
    }
}

impl From<Vec<[U32Card; 2]>> for TwoCards {
    fn from(raw: Vec<[U32Card; 2]>) -> Self {
        TwoCards::from(raw.iter().map(TwoCard::from).collect::<Vec<TwoCard>>())
    }
}

impl From<Vec<TwoCard>> for TwoCards {
    fn from(hands: Vec<TwoCard>) -> Self {
        TwoCards { hands }
    }
}

impl From<TwoCardsSet> for TwoCards {
    fn from(set: TwoCardsSet) -> Self {
        let mut v = TwoCards::from(set.to_vec());
        v.sort();
        v
    }
}

impl Vectorable for TwoCards {
    fn to_vec(&self) -> Vec<U32Card> {
        self.hands
            .iter()
            .flat_map(|x| x.iter().map(Clone::clone))
            .collect()
    }
}

pub const POCKET_PAIRS: [[U32Card; 2]; 13] = [
    TwoCard::POCKET_ROCKETS,
    TwoCard::COWBOYS,
    TwoCard::LADIES,
    TwoCard::FISHHOOKS,
    TwoCard::DIMES,
    TwoCard::POPEYES,
    TwoCard::SNOWMEN,
    TwoCard::WALKING_STICKS,
    TwoCard::ROUTE_66,
    TwoCard::SPEED_LIMIT,
    TwoCard::SAILBOATS,
    TwoCard::CRABS,
    TwoCard::DUCKS,
];

#[cfg(test)]
#[allow(non_snake_case)]
mod types_ranges_two_cards_vec_tests {
    use super::*;
    use ckc_rs::CardNumber;

    #[test]
    fn position() {
        let v = TwoCards::from(vec![TwoCard::POCKET_ROCKETS, TwoCard::COWBOYS]);
        assert_eq!(1, v.position(TwoCard::from(TwoCard::COWBOYS)).unwrap());
    }

    #[test]
    fn to_vec() {
        let v = TwoCards::from(vec![TwoCard::POCKET_ROCKETS, TwoCard::COWBOYS]);

        let expected: Vec<U32Card> = vec![
            CardNumber::ACE_DIAMONDS,
            CardNumber::ACE_CLUBS,
            CardNumber::KING_DIAMONDS,
            CardNumber::KING_CLUBS,
        ];

        assert_eq!(expected, v.to_vec());
    }
}
