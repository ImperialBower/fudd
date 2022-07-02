pub mod arrays;
pub mod bitvec;
pub mod card_slot;
pub mod hands;
pub mod playing_card;
pub mod playing_cards;
pub mod poker_cards;
pub mod poker_deck;
pub mod ranges;
pub mod sample;
pub mod slots;

pub trait PileOfCards<T> {
    fn has(&self, i: T) -> bool;
}
