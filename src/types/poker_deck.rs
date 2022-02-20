use crate::types::playing_card::PlayingCard;
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use ckc_rs::CardNumber;
// use crate::poker_cards_set::PokerCardsSet;
use std::array::IntoIter;

/// Represents a Standard52 deck as an immutable array of
/// Cactus Kev Cards (`PokerCard`).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PokerDeck([U32Card; 52]);

pub const POKER_DECK: PokerDeck = PokerDeck([
    CardNumber::ACE_SPADES,
    CardNumber::KING_SPADES,
    CardNumber::QUEEN_SPADES,
    CardNumber::JACK_SPADES,
    CardNumber::TEN_SPADES,
    CardNumber::NINE_SPADES,
    CardNumber::EIGHT_SPADES,
    CardNumber::SEVEN_SPADES,
    CardNumber::SIX_SPADES,
    CardNumber::FIVE_SPADES,
    CardNumber::FOUR_SPADES,
    CardNumber::TREY_SPADES,
    CardNumber::DEUCE_SPADES,
    CardNumber::ACE_HEARTS,
    CardNumber::KING_HEARTS,
    CardNumber::QUEEN_HEARTS,
    CardNumber::JACK_HEARTS,
    CardNumber::TEN_HEARTS,
    CardNumber::NINE_HEARTS,
    CardNumber::EIGHT_HEARTS,
    CardNumber::SEVEN_HEARTS,
    CardNumber::SIX_HEARTS,
    CardNumber::FIVE_HEARTS,
    CardNumber::FOUR_HEARTS,
    CardNumber::TREY_HEARTS,
    CardNumber::DEUCE_HEARTS,
    CardNumber::ACE_DIAMONDS,
    CardNumber::KING_DIAMONDS,
    CardNumber::QUEEN_DIAMONDS,
    CardNumber::JACK_DIAMONDS,
    CardNumber::TEN_DIAMONDS,
    CardNumber::NINE_DIAMONDS,
    CardNumber::EIGHT_DIAMONDS,
    CardNumber::SEVEN_DIAMONDS,
    CardNumber::SIX_DIAMONDS,
    CardNumber::FIVE_DIAMONDS,
    CardNumber::FOUR_DIAMONDS,
    CardNumber::TREY_DIAMONDS,
    CardNumber::DEUCE_DIAMONDS,
    CardNumber::ACE_CLUBS,
    CardNumber::KING_CLUBS,
    CardNumber::QUEEN_CLUBS,
    CardNumber::JACK_CLUBS,
    CardNumber::TEN_CLUBS,
    CardNumber::NINE_CLUBS,
    CardNumber::EIGHT_CLUBS,
    CardNumber::SEVEN_CLUBS,
    CardNumber::SIX_CLUBS,
    CardNumber::FIVE_CLUBS,
    CardNumber::FOUR_CLUBS,
    CardNumber::TREY_CLUBS,
    CardNumber::DEUCE_CLUBS,
]);

impl PokerDeck {
    #[must_use]
    pub fn get(index: usize) -> U32Card {
        POKER_DECK.0[index]
    }

    pub fn iter() -> impl Iterator<Item = &'static U32Card> {
        POKER_DECK.0.iter()
    }

    #[must_use]
    pub fn array_iter(&self) -> IntoIter<U32Card, 52> {
        self.0.into_iter()
    }

    #[must_use]
    pub fn len() -> usize {
        POKER_DECK.0.len()
    }

    #[must_use]
    pub fn pile() -> cardpack::Pile {
        let mut pile = cardpack::Pile::default();
        for card in PokerDeck::iter() {
            pile.push(PlayingCard::from(card).as_card());
        }
        pile
    }

    #[must_use]
    pub fn poker_cards() -> PokerCards {
        let mut cards = PokerCards::default();
        for card in PokerDeck::iter() {
            cards.push(*card);
        }
        cards
    }

    #[must_use]
    pub fn poker_cards_shuffled() -> PokerCards {
        let mut cards = PokerDeck::poker_cards();
        cards.shuffle_in_place();
        cards
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod poker_deck_tests {
    use super::*;

    #[test]
    fn pile() {
        let pile = PokerDeck::pile();

        assert_eq!(pile, cardpack::Pile::french_deck())
    }

    #[test]
    fn poker_cards() {
        let cards = PokerDeck::poker_cards();

        for (i, card) in PokerDeck::iter().enumerate() {
            let got = cards.get(i);
            assert!(got.is_some());
            assert_eq!(got.unwrap(), card);
        }
        assert_eq!(cards.len(), PokerDeck::len());
    }
}