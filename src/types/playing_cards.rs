use crate::analysis::eval::Eval;
use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::seven_card::SevenCard;
use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::Evaluable;
use crate::types::playing_card::PlayingCard;
use crate::types::poker_cards::PokerCards;
use crate::types::poker_deck::PokerDeck;
use crate::types::U32Card;
use crate::util::random_ordering::RandomOrdering;
use cardpack::{Pile, Standard52};
use ckc_rs::{HandError, PokerCard};
use core::fmt;
use indexmap::set::Iter;
use indexmap::IndexSet;
use itertools::{Combinations, Itertools};
use rayon::prelude::*;
use std::fmt::Formatter;

const NUMBER_OF_SHUFFLES: u8 = 5;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlayingCards(IndexSet<PlayingCard>);

impl PlayingCards {
    #[must_use]
    pub fn deck() -> PlayingCards {
        let set: Vec<PlayingCard> = PokerDeck::par_iter().map(PlayingCard::from).collect();
        PlayingCards::from(set)
    }

    /// Returns all the cards in a `PokerDeck` minus the ones passed in.
    ///
    /// TODO: :-P
    #[must_use]
    pub fn deck_minus(playing_cards: &PlayingCards) -> PlayingCards {
        let mut cards = PlayingCards::default();
        let deck = PlayingCards::deck();
        for card in deck.iter() {
            if playing_cards.get(card).is_none() {
                cards.insert(*card);
            }
        }
        cards
    }

    #[must_use]
    pub fn deck_shuffled() -> PlayingCards {
        let mut deck = PlayingCards(PokerDeck::iter().map(PlayingCard::from).collect());
        deck.shuffle_in_place();
        deck
    }

    pub fn append(&mut self, playing_cards: &PlayingCards) {
        for card in playing_cards.iter() {
            self.insert(*card);
        }
    }

    pub fn combinations(&self, k: usize) -> Combinations<Iter<'_, PlayingCard>> {
        self.iter().combinations(k)
    }

    #[must_use]
    pub fn combine(&self, playing_cards: &PlayingCards) -> PlayingCards {
        let mut cards = self.clone();
        cards.append(playing_cards);
        cards
    }

    #[must_use]
    pub fn contains(&self, playing_card: &PlayingCard) -> bool {
        self.0.contains(playing_card)
    }

    pub fn deal_from_the_bottom(&mut self) -> Option<PlayingCard> {
        self.0.pop()
    }

    #[must_use]
    pub fn draw(&mut self, number: usize) -> PlayingCards {
        PlayingCards(self.0.drain(0..number).collect())
    }

    #[must_use]
    pub fn draw_from_the_bottom(&mut self, number: usize) -> PlayingCards {
        let l = self.len();
        PlayingCards(self.0.drain(l - number..l).collect())
    }

    /// # Panics
    ///
    /// Will panic if the entity is empty.
    pub fn draw_one(&mut self) -> PlayingCard {
        self.draw(1).deal_from_the_bottom().unwrap()
    }

    /// # Errors
    ///
    /// Will throw a `HandError` if there are not exactly 7 cards.
    #[allow(clippy::unnecessary_unwrap)]
    pub fn eval_7cards(&self) -> Result<Eval, HandError> {
        match self.to_seven_array() {
            Ok(array) => Ok(array.eval()),
            Err(e) => Err(e),
        }
    }

    #[must_use]
    pub fn get(&self, playing_card: &PlayingCard) -> Option<&PlayingCard> {
        self.0.get(playing_card)
    }

    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<&PlayingCard> {
        self.0.get_index(index)
    }

    /// Allows you to insert a `PlayingCard` provided it isn't blank.
    pub fn insert(&mut self, playing_card: PlayingCard) -> bool {
        if playing_card.is_blank() {
            false
        } else {
            self.0.insert(playing_card)
        }
    }

    #[must_use]
    pub fn is_disjoint(&self, other: &PlayingCards) -> bool {
        self.0.is_disjoint(&other.0)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn is_subset(&self, other: &PlayingCards) -> bool {
        self.0.is_subset(&other.0)
    }

    #[must_use]
    pub fn is_superset(&self, other: &PlayingCards) -> bool {
        self.0.is_superset(&other.0)
    }

    #[must_use]
    pub fn iter(&self) -> indexmap::set::Iter<'_, PlayingCard> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn peak(&self) -> Option<&PlayingCard> {
        self.0.first()
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    #[must_use]
    pub fn shuffle(&self) -> PlayingCards {
        let mut shuffled = self.clone();
        shuffled.shuffle_in_place();
        shuffled
    }

    pub fn shuffle_in_place(&mut self) {
        for _ in 0..NUMBER_OF_SHUFFLES {
            self.0
                .sort_by(|_, _| rand::random::<RandomOrdering>().into());
        }
    }

    #[must_use]
    pub fn sort(&self) -> PlayingCards {
        let mut c = self.clone();
        c.sort_in_place();
        c
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    /// # Errors
    ///
    /// Throws `HandError` if not exactly 5 cards.
    #[allow(clippy::missing_panics_doc)]
    pub fn to_five_array(&self) -> Result<[PlayingCard; 5], HandError> {
        match self.len() {
            0..=4 => Err(HandError::NotEnoughCards),
            5 => Ok([
                *self.get_index(0).unwrap(),
                *self.get_index(1).unwrap(),
                *self.get_index(2).unwrap(),
                *self.get_index(3).unwrap(),
                *self.get_index(4).unwrap(),
            ]),
            _ => Err(HandError::TooManyCards),
        }
    }

    /// # Errors
    ///
    /// Will throw a `HandError` if there are not exactly 5 cards.
    pub fn to_five_cards(&self) -> Result<FiveCard, HandError> {
        match self.to_five_array() {
            Ok(hand) => Ok(FiveCard::from(hand)),
            Err(e) => Err(e),
        }
    }

    /// # Errors
    ///
    /// Will throw a `HandError` if there are not exactly 7 cards.
    #[allow(clippy::missing_panics_doc)]
    pub fn to_seven_array(&self) -> Result<SevenCard, HandError> {
        SevenCard::try_from(self)
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<PlayingCard> {
        self.iter().copied().collect::<Vec<PlayingCard>>()
    }

    #[must_use]
    pub fn two_cards(&self) -> Vec<TwoCard> {
        self.combinations(2)
            .map(TwoCard::from)
            .collect::<Vec<TwoCard>>()
    }
}

impl fmt::Display for PlayingCards {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .iter()
            .map(PlayingCard::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{}", s)
    }
}

impl indexmap::Equivalent<PlayingCard> for IndexSet<PlayingCard> {
    fn equivalent(&self, key: &PlayingCard) -> bool {
        self.get(key).is_some()
    }
}

impl From<&Pile> for PlayingCards {
    /// Returns a `PlayingCardSet` from a `Pile` filtering out any `cardpack::Card` that
    /// isn't a valid, non blank `PlayingCard`.
    ///
    /// Idea from: <https://stackoverflow.com/a/63146008/1245251/>
    fn from(pile: &Pile) -> Self {
        let filtered = pile.clone().into_iter().filter_map(|c| {
            let pc = PlayingCard::from(&c);
            if pc.is_blank() {
                None
            } else {
                Some(pc)
            }
        });
        PlayingCards(filtered.collect())
    }
}

impl From<&PokerCards> for PlayingCards {
    fn from(poker_cards: &PokerCards) -> Self {
        PlayingCards::from(&poker_cards.to_vec())
    }
}

impl From<&Vec<U32Card>> for PlayingCards {
    fn from(v: &Vec<U32Card>) -> Self {
        let filtered = v.iter().filter_map(|c| {
            let pc = PlayingCard::from(*c);
            if pc.is_blank() {
                None
            } else {
                Some(pc)
            }
        });
        PlayingCards(filtered.collect())
    }
}

impl From<Vec<&PlayingCard>> for PlayingCards {
    fn from(v: Vec<&PlayingCard>) -> Self {
        let filtered = v.iter().filter_map(|c| {
            let pc = **c;
            if pc.is_blank() {
                None
            } else {
                Some(pc)
            }
        });
        PlayingCards(filtered.collect())
    }
}

impl From<PlayingCard> for PlayingCards {
    fn from(playing_card: PlayingCard) -> Self {
        PlayingCards::from(vec![playing_card])
    }
}

impl From<Vec<PlayingCard>> for PlayingCards {
    fn from(value: Vec<PlayingCard>) -> Self {
        PlayingCards(value.into_iter().collect::<IndexSet<_>>())
    }
}

impl TryFrom<&'static str> for PlayingCards {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match Standard52::pile_from_index(value) {
            Ok(pile) => Ok(PlayingCards::from(&pile)),
            Err(_) => Err(HandError::InvalidCard),
        }
    }
}

impl TryFrom<TwoCard> for PlayingCards {
    type Error = HandError;

    fn try_from(value: TwoCard) -> Result<Self, Self::Error> {
        if value.is_dealt() {
            let mut cards = PlayingCards::default();
            cards.insert(PlayingCard::from(value.first()));
            cards.insert(PlayingCard::from(value.second()));
            Ok(cards)
        } else {
            Err(HandError::Incomplete)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod playing_cards_tests {
    use super::*;

    fn royal_flush() -> PlayingCards {
        PlayingCards::deck().draw(5)
    }

    fn is_royal_flush(cards: &PlayingCards) -> bool {
        cards.contains(&PlayingCard::from("AS"))
            && cards.contains(&PlayingCard::from("KS"))
            && cards.contains(&PlayingCard::from("QS"))
            && cards.contains(&PlayingCard::from("JS"))
            && cards.contains(&PlayingCard::from("TS"))
            && cards.len() == 5
    }

    #[test]
    fn is_royal_flush__test() {
        let mut cards = royal_flush();

        assert!(is_royal_flush(&cards));

        cards.insert(PlayingCard::from("KD"));

        assert!(!is_royal_flush(&cards));
    }

    #[test]
    fn combinations() {
        let aces = PlayingCards::try_from("AS AH AD AC").unwrap();
        assert_eq!(6, aces.combinations(2).count());
        assert_eq!(2_598_960, PlayingCards::deck().combinations(5).count());
    }

    #[test]
    fn two_cards() {
        let aces = PlayingCards::try_from("AS AH AD AC").unwrap().two_cards();
        assert_eq!(6, aces.len());
    }

    #[test]
    fn contains() {
        let cards = royal_flush();
        assert!(cards.contains(&PlayingCard::from("AS")));
        assert!(!cards.contains(&PlayingCard::from("AD")));
    }

    #[test]
    fn deal_from_the_bottom() {
        let mut cards = PlayingCards::deck();

        let card = cards.deal_from_the_bottom().unwrap();

        assert_eq!(card, PlayingCard::from("2C"));
    }

    #[test]
    fn draw() {
        let mut cards = PlayingCards::deck();

        let drawn = cards.draw(5);

        assert!(is_royal_flush(&drawn));
        assert_eq!(cards.len(), 47);
    }

    #[test]
    fn draw_one() {
        let mut cards = PlayingCards::deck();

        let drawn = cards.draw_one();

        assert_eq!(drawn, PlayingCard::from("AS"));
        assert_eq!(cards.len(), 51);
    }

    #[test]
    fn get() {
        let cards = royal_flush();
        let ace_spades = PlayingCard::from("AS");

        assert_eq!(cards.get(&ace_spades).unwrap(), &ace_spades);
        assert!(cards.get(&PlayingCard::from("AD")).is_none());
    }

    #[test]
    fn get_index() {
        let cards = royal_flush();

        assert_eq!(cards.get_index(0).unwrap(), &PlayingCard::from("AS"));
        assert_eq!(cards.get_index(1).unwrap(), &PlayingCard::from("KS"));
        assert_eq!(cards.get_index(2).unwrap(), &PlayingCard::from("QS"));
        assert_eq!(cards.get_index(3).unwrap(), &PlayingCard::from("JS"));
        assert_eq!(cards.get_index(4).unwrap(), &PlayingCard::from("TS"));
        assert!(cards.get_index(5).is_none());
    }

    #[test]
    fn insert() {
        let mut cards = royal_flush();

        let result = cards.insert(PlayingCard::from("AS"));

        assert!(!result);
        assert!(is_royal_flush(&cards));
    }

    #[test]
    fn is_disjoint() {
        let mut deck = PlayingCards::deck();
        let royal_flush = deck.draw(5);
        let straight_flush = deck.draw(5);

        assert!(royal_flush.is_disjoint(&straight_flush));
        assert!(straight_flush.is_disjoint(&royal_flush));
        assert!(straight_flush.is_disjoint(&deck));
        assert!(royal_flush.is_disjoint(&deck));
        assert!(deck.is_disjoint(&royal_flush));
        assert!(deck.is_disjoint(&straight_flush));
        assert!(!royal_flush.is_disjoint(&PlayingCards::deck().draw(2)));
    }

    #[test]
    fn is_empty() {
        assert!(PlayingCards::default().is_empty())
    }

    #[test]
    fn is_subset() {
        let cards = royal_flush();
        let other = PlayingCards::deck_shuffled();

        assert!(cards.is_subset(&other));
        assert!(!other.is_subset(&cards));
    }

    #[test]
    fn is_superset() {
        let cards = royal_flush();
        let other = PlayingCards::deck_shuffled();

        assert!(other.is_superset(&cards));
        assert!(!cards.is_superset(&other));
    }

    #[test]
    fn len() {
        assert_eq!(5, royal_flush().len())
    }

    #[test]
    fn reverse() {
        let mut cards = royal_flush();
        cards.reverse();

        assert_eq!("T♠ J♠ Q♠ K♠ A♠", cards.to_string());
    }

    #[test]
    fn shuffle_in_place() {
        let mut cards = PlayingCards::deck().draw(5);

        cards.shuffle_in_place();

        assert!(is_royal_flush(&cards));
        cards.sort_in_place();
        assert_eq!("A♠ K♠ Q♠ J♠ T♠", cards.to_string());
    }

    #[test]
    fn to_vec() {
        let v = vec![
            PlayingCard::from("AS"),
            PlayingCard::from("KS"),
            PlayingCard::from("QS"),
            PlayingCard::from("JS"),
            PlayingCard::from("TS"),
        ];

        assert_eq!(royal_flush().to_vec(), v);
    }

    #[test]
    fn display() {
        let cards = royal_flush();

        assert_eq!("A♠ K♠ Q♠ J♠ T♠", cards.to_string());
    }

    #[test]
    fn from__pile() {
        let expected = PlayingCards::from(&Pile::french_deck());
        let actual = PlayingCards::from(&Pile::french_deck_with_jokers());

        assert_eq!(expected, actual);
        assert_ne!(royal_flush(), actual);
        assert!(PlayingCards::from(&Pile::skat_deck()).is_empty());
    }

    #[test]
    fn try_from__static_str() {
        let actual = PlayingCards::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(royal_flush(), actual);
    }

    #[test]
    fn try_from__two_cards() {
        let actual = PlayingCards::try_from(TwoCard::try_from("Q♠ J♠").unwrap()).unwrap();

        assert_eq!("Q♠ J♠", actual.to_string());
    }

    #[test]
    fn try_from__two_cards__invalid() {
        let actual = PlayingCards::try_from(TwoCard::default());

        assert!(actual.is_err());
    }
}
