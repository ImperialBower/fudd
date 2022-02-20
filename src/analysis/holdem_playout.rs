use crate::games::holdem::board::Board;
use crate::games::holdem::case_eval::CaseEval;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HoldemPlayout {
    board: Board,
    eval: CaseEval,
}

impl HoldemPlayout {
    // pub fn new(hands: Vec<HoleCards>, board: Board) -> HoldemPlayout {
    //     let mut eval = CaseEval::default();
    //     for hand in hands {}
    //
    //     HoldemPlayout::default()
    // }
}
