use crate::analysis::eval::Eval;
use crate::analysis::indexed::Indexed;
use ckc_rs::hand_rank::HandRankClass;
use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Evals(Vec<Eval>);

impl Evals {
    pub fn push(&mut self, evaluated_hand: Eval) {
        self.0.push(evaluated_hand);
    }

    #[must_use]
    pub fn best(&self) -> Eval {
        let mut best = Eval::default();
        for eval in &self.0 {
            if *eval > best {
                best = *eval;
            }
        }
        best
    }

    #[must_use]
    pub fn sort(&self) -> Evals {
        let mut cards = self.clone();
        cards.sort_in_place();
        cards
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

    #[must_use]
    pub fn to_vec(&self) -> &Vec<Eval> {
        &self.0
    }
}

impl From<Eval> for Evals {
    fn from(eval: Eval) -> Self {
        Evals(vec![eval])
    }
}

/// `EvalsPerClass` is a tuple struct used to hold unique hand evaluations.
#[derive(Clone, Debug, Default, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct EvalsPerClass(Vec<Eval>, HashSet<HandRankClass>);

impl EvalsPerClass {
    pub fn push(&mut self, evaluated_hand: Eval) {
        if self.1.insert(evaluated_hand.rank.class) {
            self.0.push(evaluated_hand);
        }
    }

    #[must_use]
    pub fn sort(&self) -> EvalsPerClass {
        let mut cards = self.clone();
        cards.sort_in_place();
        cards
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

    #[must_use]
    pub fn to_vec(&self) -> &Vec<Eval> {
        &self.0
    }

    #[must_use]
    pub fn indexed(&self) -> Indexed {
        Indexed::from(self)
    }
}
