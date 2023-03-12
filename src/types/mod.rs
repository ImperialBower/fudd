pub mod arrays;
pub mod card_slot;
pub mod hands;
pub mod playing_card;
pub mod playing_cards;
pub mod poker_cards;
pub mod poker_deck;
pub mod ranges;
pub mod sample;
pub mod slots;

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Base Entities

/// A `U32Card` is a u32 type alias of a variant of Cactus Kev's binary
/// representation of a poker card as designed for rapid hand evaluation as
/// documented [here](https://suffe.cool/poker/evaluator.html).
///
/// The variation being that the `Suit` bits order is inverted for easier sorting.
/// ```txt
/// +--------+--------+--------+--------+
/// |xxxbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// SHDC = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// ```
pub type U32Card = u32;

pub trait PileOfCards<T> {
    fn has(&self, i: T) -> bool;
}
