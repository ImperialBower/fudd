use crate::types::arrays::two_cards::TwoCards;
use crate::types::arrays::Vectorable;
use crate::types::ranges::two_cards_set::TwoCardsSet;
use crate::types::sample::Sample;
use crate::types::U32Card;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct TwoCardsVec {
    pub hands: Vec<TwoCards>,
}

impl TwoCardsVec {
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TwoCards> {
        self.hands.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hands.is_empty()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, TwoCards> {
        self.hands.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.hands.len()
    }

    #[must_use]
    pub fn pairs(&self) -> TwoCardsVec {
        let pairs: Vec<TwoCards> = self
            .hands
            .clone()
            .into_iter()
            .filter(TwoCards::is_pocket_pair)
            .collect();
        TwoCardsVec::from(pairs)
    }

    pub fn pop(&mut self) -> Option<TwoCards> {
        self.hands.pop()
    }

    #[must_use]
    pub fn position(&self, two_cards: TwoCards) -> Option<usize> {
        self.hands.iter().position(|&r| r == two_cards)
    }

    pub fn push(&mut self, two_cards: TwoCards) {
        self.hands.push(two_cards);
    }

    pub fn sample(&mut self) -> Option<TwoCards> {
        self.hands.sample()
    }

    /// Sorts the vector in place..
    pub fn sort(&mut self) {
        self.hands.sort_unstable();
        self.hands.reverse();
    }

    #[must_use]
    pub fn suited(&self) -> TwoCardsVec {
        TwoCardsVec::from(
            self.hands
                .clone()
                .into_iter()
                .filter(TwoCards::is_suited)
                .collect::<Vec<TwoCards>>(),
        )
    }

    /// Returns a sorted vector of the range.
    #[must_use]
    pub fn sorted(&self) -> TwoCardsVec {
        let mut v = self.clone();
        v.sort();
        v
    }
}

impl From<Vec<[U32Card; 2]>> for TwoCardsVec {
    fn from(raw: Vec<[U32Card; 2]>) -> Self {
        TwoCardsVec::from(raw.iter().map(TwoCards::from).collect::<Vec<TwoCards>>())
    }
}

impl From<Vec<TwoCards>> for TwoCardsVec {
    fn from(hands: Vec<TwoCards>) -> Self {
        TwoCardsVec { hands }
    }
}

impl From<TwoCardsSet> for TwoCardsVec {
    fn from(set: TwoCardsSet) -> Self {
        let mut v = TwoCardsVec::from(set.to_vec());
        v.sort();
        v
    }
}

impl Vectorable for TwoCardsVec {
    fn to_vec(&self) -> Vec<U32Card> {
        self.hands
            .iter()
            .flat_map(|x| x.iter().map(Clone::clone))
            .collect()
    }
}

pub const POCKET_PAIRS: [[U32Card; 2]; 13] = [
    TwoCards::POCKET_ROCKETS,
    TwoCards::COWBOYS,
    TwoCards::LADIES,
    TwoCards::FISHHOOKS,
    TwoCards::DIMES,
    TwoCards::POPEYES,
    TwoCards::SNOWMEN,
    TwoCards::WALKING_STICKS,
    TwoCards::ROUTE_66,
    TwoCards::SPEED_LIMIT,
    TwoCards::SAILBOATS,
    TwoCards::CRABS,
    TwoCards::DUCKS,
];

#[cfg(test)]
#[allow(non_snake_case)]
mod types_ranges_two_cards_vec_tests {
    use super::*;
    use ckc_rs::CardNumber;

    #[test]
    fn position() {
        let v = TwoCardsVec::from(vec![TwoCards::POCKET_ROCKETS, TwoCards::COWBOYS]);
        assert_eq!(1, v.position(TwoCards::from(TwoCards::COWBOYS)).unwrap());
    }

    #[test]
    fn to_vec() {
        let v = TwoCardsVec::from(vec![TwoCards::POCKET_ROCKETS, TwoCards::COWBOYS]);

        let expected: Vec<U32Card> = vec![
            CardNumber::ACE_DIAMONDS,
            CardNumber::ACE_CLUBS,
            CardNumber::KING_DIAMONDS,
            CardNumber::KING_CLUBS,
        ];

        assert_eq!(expected, v.to_vec());
    }
}
