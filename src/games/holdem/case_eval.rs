use crate::games::holdem::seat_eval::SeatEval;
use ckc_rs::hand_rank::HandRank;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

/// A `CaseEval` is a collection of `SeatEvals` for a specific selection of `PokerCards`, or case.
/// While a `SeatEval` is able to return the best possible hand for a specific player given
/// a specific collection of cards, a `CaseEval` is able to compare the evaluations for all
/// the players in the collection and returns the ones that are winners. This needs to be a
/// collection because it is possible for more than one player to have the best hand.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CaseEval(Vec<SeatEval>);

impl CaseEval {
    #[must_use]
    pub fn get_seat(&self, seat_number: usize) -> Option<&SeatEval> {
        for seat_eval in self.iter() {
            if seat_eval.seat.number == seat_number {
                return Some(seat_eval);
            }
        }
        None
    }

    pub fn push(&mut self, seat_eval: SeatEval) {
        self.0.push(seat_eval);
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, SeatEval> {
        self.0.iter()
    }

    #[must_use]
    pub fn has_seat(&self, number: usize) -> bool {
        for s in &self.0 {
            if s.seat.number == number {
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn seats(&self) -> &Vec<SeatEval> {
        &self.0
    }

    #[must_use]
    pub fn winning_rank(&self) -> HandRank {
        let mut winning_rank = HandRank::default();
        for seat in &self.0 {
            if seat.eval.rank > winning_rank {
                winning_rank = seat.eval.rank;
            }
        }
        winning_rank
    }

    #[must_use]
    pub fn winners(&self) -> CaseEval {
        let winning_rank = self.winning_rank();
        let mut winners = CaseEval::default();
        for seat in &self.0 {
            if seat.eval.rank == winning_rank {
                winners.push(seat.clone());
            }
        }
        winners
    }
}
