use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use log::warn;

/// Trait for a collection of one or more `PlayingCards`. `CardSlots` take
/// `PlayingCards` one at a time, report if all of their slots are full, and
/// return their holdings as `PlayingCards`.
pub trait CardSlot {
    fn take(&self, card: PlayingCard) -> bool;

    fn take_from_index(&self, index: &'static str) -> bool {
        if let Ok(cards) = PlayingCards::try_from(index) {
            let mut is_ok = false;
            for card in cards.iter() {
                is_ok = self.take(*card);
            }
            is_ok
        } else {
            warn!("Invalid Index: {}", index);
            false
        }
    }

    fn take_from_playing_cards(&self, playing_cards: &PlayingCards) -> bool {
        let mut is_ok = false;
        for card in playing_cards.iter() {
            is_ok = self.take(*card);
        }
        is_ok
    }

    fn take_from_poker_cards(&self, poker_cards: &PokerCards) -> bool {
        self.take_from_playing_cards(&PlayingCards::from(poker_cards))
    }

    fn dealt(&self) -> PlayingCards {
        let mut playing_cards = PlayingCards::default();
        for card in self.to_playing_cards().iter() {
            playing_cards.insert(*card);
        }
        playing_cards
    }

    fn fold(&self) -> PlayingCards;

    fn is_dealt(&self) -> bool;

    fn to_playing_cards(&self) -> PlayingCards;

    fn is_blank(&self) -> bool {
        self.to_playing_cards().is_empty()
    }

    fn is_empty(&self) -> bool {
        self.is_blank()
    }

    fn len(&self) -> usize {
        self.to_playing_cards().len()
    }

    /// Returns all the `PokerCards` from a complete deck that aren't in the `CardSlot`.
    fn remaining(&self) -> PlayingCards {
        PlayingCards::deck_minus(&self.to_playing_cards())
    }
}
