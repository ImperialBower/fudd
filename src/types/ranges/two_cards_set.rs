use crate::types::arrays::two_cards::TwoCards;
use crate::types::playing_cards::PlayingCards;
use crate::types::ranges::two_cards_vec::TwoCardsVec;
use ckc_rs::PokerCard;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::Iter;
use std::collections::HashSet;

/// * [Texas hold 'em starting hands](https://en.wikipedia.org/wiki/Texas_hold_%27em_starting_hands)
/// * [Starting Hands](https://betandbeat.com/poker/strategy/preflop/starting-hands/)
///
/// TODO: Scenario: AK JTs 22 HSK S06E02
/// TODO: Scenario: A2 T9s HSK S06E03
#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct TwoCardsSet {
    pub hands: HashSet<TwoCards>,
}

impl TwoCardsSet {
    /// Returns a `HoleCardRange` consisting of every possible combination of `TwoCards`.
    #[must_use]
    pub fn every() -> TwoCardsSet {
        TwoCardsSet::from(PlayingCards::deck())
    }

    #[must_use]
    pub fn contains(&self, cards: &TwoCards) -> bool {
        self.hands.contains(cards)
    }

    /// Returns a `HoleCardRange` of all the `TwoCards` that are in self and not in other.
    #[must_use]
    pub fn difference(&self, other: &TwoCardsSet) -> TwoCardsSet {
        TwoCardsSet {
            hands: self.hands.difference(&other.hands).copied().collect(),
        }
    }

    /// # Errors
    ///
    /// Throws a `HandError::DuplicateCard` error if a `PokerCard` passed in
    /// already exists in the `Range`.
    pub fn insert(&mut self, cards: TwoCards) -> bool {
        if self.contains(&cards) {
            false
        } else {
            self.hands.insert(cards)
        }
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

    /// Returns a `HoleCardRange` of all `TwoCards` contained in both entities.
    #[must_use]
    pub fn overlap(&self, other: &TwoCardsSet) -> TwoCardsSet {
        TwoCardsSet::from(
            self.hands
                .intersection(&other.hands)
                .copied()
                .collect::<HashSet<TwoCards>>(),
        )
    }

    #[must_use]
    pub fn pairs(&self) -> TwoCardsSet {
        TwoCardsSet::from(&self.two_cards_vec().pairs().hands)
    }

    pub fn remove(&mut self, cards: &TwoCards) -> bool {
        self.hands.remove(cards)
    }

    pub fn sample(&mut self) -> Option<TwoCards> {
        let sampled = self.two_cards_vec().sample();
        match sampled {
            Some(s) => {
                if self.remove(&s) {
                    Some(s)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    #[must_use]
    pub fn sampler(&mut self, number: usize) -> TwoCardsSet {
        let mut sampler = TwoCardsSet::default();
        for _ in 0..number {
            match self.sample() {
                Some(s) => sampler.insert(s),
                None => false,
            };
        }
        sampler
    }

    #[must_use]
    pub fn suited(&self) -> TwoCardsSet {
        TwoCardsSet::from(&self.two_cards_vec().suited().hands)
    }

    #[must_use]
    pub fn union(&self, other: &TwoCardsSet) -> TwoCardsSet {
        TwoCardsSet {
            hands: self.hands.union(&other.hands).copied().collect(),
        }
    }

    /// Returns a sorted vector of the range.
    #[must_use]
    pub fn two_cards_vec(&self) -> TwoCardsVec {
        TwoCardsVec::from(self.clone())
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<TwoCards> {
        self.hands.clone().into_iter().collect()
    }
}

impl From<HashSet<TwoCards>> for TwoCardsSet {
    fn from(hands: HashSet<TwoCards>) -> Self {
        TwoCardsSet { hands }
    }
}

impl From<PlayingCards> for TwoCardsSet {
    fn from(deck: PlayingCards) -> Self {
        if deck.len() < 2 {
            return TwoCardsSet::default();
        }
        let mut range = TwoCardsSet::default();
        for i1 in 0..deck.len() {
            for i2 in (i1 + 1)..deck.len() {
                range.insert(
                    TwoCards::new(
                        deck.get_index(i1).unwrap().as_u32(),
                        deck.get_index(i2).unwrap().as_u32(),
                    )
                    .unwrap(),
                );
            }
        }
        range
    }
}

impl From<&Vec<TwoCards>> for TwoCardsSet {
    fn from(hands: &Vec<TwoCards>) -> Self {
        TwoCardsSet::from(hands.iter().copied().collect::<HashSet<_>>())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_ranges_two_cards_set_tests {
    use super::*;

    #[test]
    fn every() {
        assert_eq!(TwoCardsSet::every().len(), 1326);
    }

    #[test]
    fn difference() {
        let mut range1 = TwoCardsSet::default();
        let mut range2 = TwoCardsSet::default();
        let aces = TwoCards::try_from("AS AC").unwrap();
        let kings = TwoCards::try_from("KS KC").unwrap();
        let queens = TwoCards::try_from("QS QC").unwrap();

        range1.insert(aces);
        range1.insert(kings);
        range2.insert(kings);
        range2.insert(queens);
    }

    #[test]
    fn insert() {
        let mut range = TwoCardsSet::default();
        let two = TwoCards::try_from("AS AC").unwrap();

        let actual = range.insert(two);

        assert!(actual);
        assert!(!range.insert(two));
    }

    #[test]
    fn is_empty() {
        assert!(TwoCardsSet::default().is_empty());
    }

    #[test]
    fn len() {
        assert_eq!(0, TwoCardsSet::default().len());
    }

    #[test]
    fn pairs() {
        assert_eq!(TwoCardsSet::every().pairs().len(), 78);
    }

    #[test]
    fn remove() {
        let mut range = TwoCardsSet::default();
        let aces = TwoCards::try_from("AS AC").unwrap();
        let kings = TwoCards::try_from("KS KC").unwrap();
        range.insert(aces);
        range.insert(kings);

        assert!(range.remove(&kings));
        assert!(range.remove(&aces));
        assert!(!range.remove(&kings));
        assert!(!range.remove(&aces));
    }

    #[test]
    fn sample() {
        let mut sampled = TwoCardsSet::every();

        let sample = sampled.sample().unwrap();

        assert!(!sampled.contains(&sample));
        assert!(sampled.insert(sample));
        assert!(sampled.contains(&sample));
    }

    #[test]
    fn sample__empty() {
        let sample = TwoCardsSet::default().sample();

        assert!(sample.is_none());
    }

    #[test]
    fn sampler__empty() {
        let sampler = TwoCardsSet::default().sampler(2);

        assert!(sampler.is_empty());
    }

    #[test]
    fn suited() {
        assert_eq!(TwoCardsSet::every().suited().len(), 312);
    }

    #[test]
    fn default() {
        let range = TwoCardsSet::default();

        assert!(range.is_empty());
    }

    #[test]
    fn from__playing_cards() {
        let cards = PlayingCards::try_from("AS KS AH KH").unwrap();

        let range = TwoCardsSet::from(cards);

        assert_eq!(range.len(), 6);
        assert!(range.contains(&TwoCards::try_from("AS KS").unwrap()));
        assert!(range.contains(&TwoCards::try_from("AS KH").unwrap()));
        assert!(!range.contains(&TwoCards::try_from("AS JH").unwrap()));
    }

    #[test]
    fn from__playing_cards__not_enough() {
        let cards = PlayingCards::try_from("AS").unwrap();

        let range = TwoCardsSet::from(cards);

        assert!(range.is_empty());
    }
}
