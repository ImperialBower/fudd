use crate::analysis::store::holdem::heads_up_odds::HeadsUpOdds;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct HeadsUpCsv(HashMap<String, HeadsUpOdds>);

impl HeadsUpCsv {
    // pub fn add(&mut self, _row: HeadsUpOdds) {}
}

#[cfg(test)]
#[allow(non_snake_case)]
mod analysis_store_holdem_heads_up_csv {
    // use super::*;
}
