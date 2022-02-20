use crate::analysis::eval::Eval;
use crate::analysis::evals::Evals;
use crate::types::arrays::five_cards::FiveCards;
use crate::types::bitvec::bit_cards::BitCards;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use cardpack::Pile;
use ckc_rs::hand_rank::HandRank;

pub mod five_cards;
pub mod four_cards;
pub mod range_vector;
pub mod seven_cards;
pub mod six_cards;
pub mod three_cards;
pub mod two_cards;

pub trait Evaluable {
    /// Returns the best `FiveCards` hand and `HandRank` without
    /// passing it into eval where the hand will be sorted.
    fn evaluate(&self) -> (FiveCards, HandRank) {
        let evals = self.evals();
        let best = evals.best();
        (best.hand, best.rank)
    }

    /// Returns an Eval struct containing the best `FiveCards` hand, sorted,
    /// and `HandRank`.
    fn eval(&self) -> Eval {
        let (hand, rank) = self.evaluate();
        Eval::new(hand, rank)
    }

    /// Returns all possible `Evals` for the hand
    fn evals(&self) -> Evals;
}

pub trait Vectorable {
    fn to_vec(&self) -> Vec<U32Card>;

    fn contains(&self, poker_card: &U32Card) -> bool {
        self.to_vec().contains(poker_card)
    }

    fn rank_count(&self) -> u8 {
        BitCards::from(self.to_vec()).rank_count()
    }

    fn remaining(&self) -> PlayingCards {
        PlayingCards::deck_minus(&self.to_playing_cards())
    }

    fn suit_count(&self) -> u8 {
        BitCards::from(self.to_vec()).suit_count()
    }

    fn to_pile(&self) -> Pile {
        PokerCards::from(self.to_vec()).to_pile()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        PlayingCards::from(&self.to_vec())
    }

    fn to_poker_cards(&self) -> PokerCards {
        PokerCards::from(self.to_vec())
    }
}
