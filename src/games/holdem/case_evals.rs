use crate::analysis::chances::Chances;
use crate::analysis::seat_calc::SeatCalc;
use crate::games::holdem::case_eval::CaseEval;
use ckc_rs::hand_rank::HandRank;

/// While `CaseEval` is able to determine the winners for a specific collection of `PokerCards`
/// or case, `CaseEvals` is able to determine the winning hand for all of the cases in the
/// collection, as well as the `Chances` for each seat to win once all cards are dealt.
#[derive(Clone, Debug, Default)]
pub struct CaseEvals(Vec<CaseEval>);

impl CaseEvals {
    pub fn push(&mut self, case_eval: CaseEval) {
        self.0.push(case_eval);
    }

    #[must_use]
    pub fn chances(&self) -> Chances {
        let mut calc = self.primed_seat_calc();
        for case in &self.0 {
            for winner in case.winners().iter() {
                calc.increment(winner.seat.number);
            }
        }
        calc.chances(self.0.len())
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
    pub fn winners(&self) -> CaseEval {
        let mut winners = CaseEval::default();
        let winning_rank: HandRank = HandRank::default();
        for case in &self.0 {
            if case.winning_rank() > winning_rank {
                winners = case.winners();
            }
        }
        winners
    }

    /// Prime the calculations so that zero chance seats aren't excluded.
    fn primed_seat_calc(&self) -> SeatCalc {
        match self.0.first() {
            Some(case) => {
                let mut calc = SeatCalc::default();
                for seat in case.seats() {
                    calc.touch(seat.seat.number);
                }
                calc
            }
            None => SeatCalc::default(),
        }
    }
}
