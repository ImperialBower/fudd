use crate::analysis::eval::Eval;
use crate::analysis::evals::Evals;
use crate::types::arrays::five_card::FiveCard;
use crate::types::bitvec::bit_cards::BitCards;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use cardpack::Pile;
use ckc_rs::hand_rank::HandRank;
use ckc_rs::{CKCNumber, CardNumber};

pub mod five_card;
pub mod four_card;
pub mod range_vector;
pub mod seven_card;
pub mod six_card;
pub mod three_card;
pub mod two_card;

pub trait Evaluable {
    /// Returns the best `FiveCards` hand and `HandRank` without
    /// passing it into eval where the hand will be sorted.
    fn evaluate(&self) -> (FiveCard, HandRank) {
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
    fn to_vec(&self) -> Vec<CKCNumber>;

    fn contains(&self, poker_card: &CKCNumber) -> bool {
        self.to_vec().contains(poker_card)
    }

    fn is_blank(&self) -> bool {
        self.contains(&CardNumber::BLANK)
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
