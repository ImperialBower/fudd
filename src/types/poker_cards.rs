use crate::types::arrays::Vectorable;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_deck::PokerDeck;
use crate::types::U32Card;
use cardpack::{Pile, Standard52};
use ckc_rs::cards::two::Two;
use ckc_rs::{CKCNumber, CardNumber, CardRank, HandError, PokerCard};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

pub const POSSIBLE_COMBINATIONS: usize = 7937;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PokerCards(Vec<U32Card>);

impl PokerCards {
    #[must_use]
    pub fn deck() -> PokerCards {
        PokerDeck::poker_cards()
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[must_use]
    pub fn from_index(index: &'static str) -> Result<PokerCards, HandError> {
        let mut cards = PokerCards::default();

        for s in index.split_whitespace() {
            let card = CKCNumber::from_index(s);
            if card.is_blank() {
                return Err(HandError::InvalidCard);
            }
            cards.push(card);
        }
        Ok(cards)
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_index_string(index: String) -> Result<PokerCards, HandError> {
        let mut cards = PokerCards::default();

        for s in index.split_whitespace() {
            let card = CKCNumber::from_index(s);
            if card.is_blank() {
                return Err(HandError::InvalidCard);
            }
            cards.push(card);
        }
        Ok(cards)
    }

    /// Appends a clone of the passed in collection of `PokerCards` to the existing one.
    pub fn append(&mut self, other: &PokerCards) {
        self.0.append(&mut other.0.clone());
    }

    #[must_use]
    pub fn combine(self, other: &PokerCards) -> PokerCards {
        let mut r = self;
        r.append(&other.filter_blank());
        r
    }

    #[must_use]
    pub fn contains(&self, card: &U32Card) -> bool {
        self.0.contains(card)
    }

    #[must_use]
    pub fn deal(&mut self, number: usize) -> PokerCards {
        if self.len() >= number {
            let v: Vec<U32Card> = self.0.drain(0..number).collect();
            PokerCards(v)
        } else {
            PokerCards::default()
        }
    }

    /// # Panics
    ///
    /// Will panic if there aren't five cards available in the passed in `Standard52`
    /// deck.
    ///
    /// TODO: Improve me
    #[must_use]
    pub fn deal_from_standard52(standard52: &mut Standard52, number: usize) -> PokerCards {
        let pile = standard52.draw(number).unwrap();
        let mut cards = PokerCards::default();
        for card in pile {
            cards.push(PlayingCard::from(&card).as_u32());
        }
        cards
    }

    #[must_use]
    pub fn deal_from_the_bottom(&mut self, num: usize) -> PokerCards {
        let mut dealt = PokerCards::default();
        for _ in 0..num {
            let popped = self.pop();
            if let Some(card) = popped {
                dealt.push(card);
            }
        }
        dealt.0.reverse();
        dealt
    }

    #[must_use]
    pub fn divisible_by(&self, x: usize) -> bool {
        (self.len() % x) == 0
    }

    #[must_use]
    pub fn filter_blank(&self) -> PokerCards {
        PokerCards::from(
            self.0
                .clone()
                .into_iter()
                .filter(|c| *c != CardNumber::BLANK)
                .collect::<Vec<U32Card>>(),
        )
    }

    #[must_use]
    pub fn filter_eqgt_on_rank(&self, rank: CardRank) -> PokerCards {
        PokerCards::from(
            self.0
                .clone()
                .into_iter()
                .filter(|c| c.get_card_rank() as u8 >= rank as u8)
                .collect::<Vec<U32Card>>(),
        )
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&U32Card> {
        self.0.get(index)
    }

    #[must_use]
    pub fn is_complete_hand(&self) -> bool {
        self.len() == 5
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.len() == PlayingCards::from(self).len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &U32Card> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<U32Card> {
        self.0.pop()
    }

    /// Returns a vector of all the prime bits of the CKC.
    #[must_use]
    pub fn primes(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for c in self.iter() {
            v.push(c & 0xff);
        }
        v
    }

    pub fn pull(&mut self) -> U32Card {
        if self.is_empty() {
            return CardNumber::BLANK;
        }
        self.0.remove(0)
    }

    /// Appends an `PokerCard` to the back of a collection..
    pub fn push(&mut self, ckc: U32Card) {
        self.0.push(ckc);
    }

    #[must_use]
    pub fn shuffle(&self) -> PokerCards {
        let mut shuffled = self.clone();
        shuffled.shuffle_in_place();
        shuffled
    }

    pub fn shuffle_in_place(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    #[must_use]
    pub fn sort(&self) -> PokerCards {
        let mut cards = self.clone();
        cards.sort_in_place();
        cards
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

    /// Converts a `PokerCards` collection to a `cardpack::Pile` collection
    /// of `cardpack::Card` entities.
    #[must_use]
    pub fn to_pile(&self) -> Pile {
        let mut pile = Pile::default();

        for card in &self.0 {
            pile.push(PlayingCard::from(*card).as_card());
        }

        pile
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCardCount` for an invalid index.
    #[must_use]
    pub fn try_into_twos(&self) -> Result<Vec<Two>, HandError> {
        if !self.divisible_by(2) {
            return Err(HandError::InvalidCardCount);
        }
        let mut v: Vec<Two> = Vec::new();
        let mut cards = self.clone();
        loop {
            let c1 = cards.pull();
            if c1.is_blank() {
                break;
            }
            v.push(Two::new(c1, cards.pull()));
        }
        Ok(v)
    }
}

impl Default for PokerCards {
    fn default() -> Self {
        PokerCards::from(Vec::new())
    }
}

impl fmt::Display for PokerCards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pile().to_symbol_index())
    }
}

impl From<Pile> for PokerCards {
    fn from(value: Pile) -> Self {
        let mut pcks = PokerCards::default();
        for card in value {
            pcks.push(PlayingCard::from(&card).as_u32());
        }
        pcks
    }
}

impl From<&PlayingCards> for PokerCards {
    fn from(cards: &PlayingCards) -> Self {
        PokerCards(cards.iter().map(PlayingCard::as_u32).collect())
    }
}

impl From<Vec<U32Card>> for PokerCards {
    fn from(value: Vec<U32Card>) -> Self {
        PokerCards(value)
    }
}

impl TryFrom<&'static str> for PokerCards {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        PokerCards::from_index(value)
    }
}

impl TryFrom<String> for PokerCards {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        PokerCards::from_index_string(value)
    }
}

impl Vectorable for PokerCards {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.clone()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod poker_cards_tests {
    use super::*;
    use crate::types::poker_deck::PokerDeck;

    #[test]
    fn append() {
        let standard52 = &mut Standard52::new_shuffled();
        let hole_cards = PokerCards::deal_from_standard52(standard52, 2);
        let flop = PokerCards::deal_from_standard52(standard52, 3);

        let mut five_cards = hole_cards.clone();
        five_cards.append(&flop);

        assert_eq!(
            format!("{}", five_cards),
            format!("{} {}", hole_cards, flop)
        );
    }

    #[test]
    fn combine() {
        let standard52 = &mut Standard52::new_shuffled();
        let hole_cards = PokerCards::deal_from_standard52(standard52, 2);
        let flop = PokerCards::deal_from_standard52(standard52, 3);

        let five_cards = hole_cards.clone().combine(&flop);

        assert_eq!(
            format!("{}", five_cards),
            format!("{} {}", hole_cards, flop)
        );
    }

    #[test]
    fn contains() {
        let poker_cards = PokerCards::try_from("AS KS").unwrap();

        assert!(poker_cards.contains(&CardNumber::ACE_SPADES));
        assert!(poker_cards.contains(&CardNumber::KING_SPADES));
        assert!(!poker_cards.contains(&CardNumber::QUEEN_SPADES));
    }

    #[test]
    fn deal() {
        let mut deck = PokerDeck::poker_cards_shuffled();
        let hand1 = deck.deal(5);
        let hand2 = deck.deal(5);

        assert_eq!(deck.len(), 42);
        assert_eq!(hand1.len(), 5);
        assert_eq!(hand2.len(), 5);
    }

    #[test]
    fn deal_overflow() {
        let mut deck = PokerDeck::poker_cards_shuffled();
        let hand1 = deck.deal(25);
        let hand2 = deck.deal(25);
        let hand3 = deck.deal(3);

        assert_eq!(deck.len(), 2);
        assert_eq!(hand1.len(), 25);
        assert_eq!(hand2.len(), 25);
        assert!(hand3.is_empty());
    }

    #[test]
    fn filter_blank() {
        let mut poker_cards = PokerCards::try_from("AS KS").unwrap();
        poker_cards.push(CardNumber::BLANK);
        poker_cards.push(CardNumber::BLANK);
        poker_cards.push(CardNumber::TEN_SPADES);

        assert_eq!(poker_cards.len(), 5);
        assert_eq!("A♠ K♠ __ __ T♠", poker_cards.to_string());
        assert_eq!("A♠ K♠ T♠", poker_cards.filter_blank().to_string());
    }

    #[test]
    fn filter_gt_on_rank() {
        let deck = PokerCards::deck();

        let filtered = deck.filter_eqgt_on_rank(CardRank::KING);

        assert_eq!(filtered.len(), 8);
        for card in filtered.iter() {
            println!("{}", card.to_string());
        }
    }

    #[test]
    fn deal_from_the_bottom() {
        let mut hand = PokerCards::try_from("A♠ A♠ Q♠ J♠ T♠").unwrap();

        let dealt = hand.deal_from_the_bottom(3);

        assert_eq!("Q♠ J♠ T♠", dealt.to_string());
        assert_eq!("A♠ A♠", hand.to_string());
    }

    #[test]
    fn deal_from_the_bottom__some() {
        let mut hand = PokerCards::try_from("J♠ T♠").unwrap();

        let dealt = hand.deal_from_the_bottom(3);

        assert_eq!(dealt.len(), 2);
        assert_eq!("J♠ T♠", dealt.to_string());
        assert!(hand.is_empty());
    }

    #[test]
    fn deal_from_the_bottom__none() {
        let mut hand = PokerCards::default();

        let dealt = hand.deal_from_the_bottom(3);

        assert_eq!(dealt, hand);
        assert!(dealt.is_empty());
    }

    #[test]
    fn is_valid() {
        assert!(PokerCards::try_from("  A♠ Q♠   J♠    T♠ ")
            .unwrap()
            .is_valid());
        assert!(!PokerCards::try_from("A♠ A♠ Q♠ J♠ T♠").unwrap().is_valid());
    }

    #[test]
    fn pop() {
        let mut hand = PokerCards::try_from("A♠ A♠ Q♠ J♠ T♠").unwrap();

        let last = hand.pop().unwrap();

        assert_eq!(last, CardNumber::TEN_SPADES);
        assert_eq!("A♠ A♠ Q♠ J♠", hand.to_string());
    }

    #[test]
    fn pop__empty() {
        let mut hand = PokerCards::default();

        let last = hand.pop();

        assert!(last.is_none());
    }

    #[test]
    #[ignore]
    fn sort() {
        let hand = PokerCards::try_from("KC AD KH KD AS").unwrap();
        let sorted = hand.sort();
        println!("{}", sorted);

        let hand = PokerCards::try_from("KC AD KH KD AS").unwrap();
        let sorted = hand.sort();
        println!("{}", sorted);
    }

    #[test]
    fn try_into_twos() {
        let two = vec![
            Two::try_from("J♠ T♠").unwrap(),
            Two::try_from("A♠ K♠").unwrap(),
        ];

        let cards = PokerCards::try_from("JS TS AS KS")
            .unwrap()
            .try_into_twos()
            .unwrap();

        assert_eq!(two, cards);
    }

    #[test]
    fn try_into_twos__not_divisible_into_two() {
        let cards = PokerCards::try_from("KC AD KH KD AS")
            .unwrap()
            .try_into_twos();

        assert!(cards.is_err());
    }

    #[test]
    fn try_from__index_str() {
        assert_eq!(
            "K♣ A♦ K♥ K♦ A♠",
            PokerCards::try_from("KC AD KH KD AS").unwrap().to_string()
        );
    }

    #[test]
    fn try_from__index_str__invalid() {
        let cards = PokerCards::try_from("KX AD KH KD AS");
        assert!(cards.is_err());
        assert_eq!(HandError::InvalidCard, cards.unwrap_err());
    }
}
