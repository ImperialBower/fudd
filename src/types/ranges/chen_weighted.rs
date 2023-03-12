use crate::types::arrays::two_card::TwoCard;
use crate::types::ranges::two_cards::TwoCards;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[allow(clippy::module_name_repetitions)]
pub struct ChenWeightedPair {
    pub weight: i8,
    pub pair: TwoCard,
}

impl From<TwoCard> for ChenWeightedPair {
    fn from(pair: TwoCard) -> Self {
        ChenWeightedPair {
            weight: pair.chen_formula(),
            pair,
        }
    }
}

impl fmt::Display for ChenWeightedPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} chen# {}", self.pair, self.weight)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChenWeighted(Vec<ChenWeightedPair>);

impl ChenWeighted {
    #[must_use]
    pub fn all() -> ChenWeighted {
        ChenWeighted::from(TwoCards::all())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, ChenWeightedPair> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: ChenWeightedPair) {
        self.0.push(value);
    }

    pub fn push_two_card(&mut self, two: TwoCard) {
        self.push(ChenWeightedPair::from(two));
    }

    #[must_use]
    pub fn sort(&self) -> ChenWeighted {
        let mut c = self.clone();
        c.sort_in_place();
        c
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort();
        self.0.reverse();
    }
}

impl From<Vec<ChenWeightedPair>> for ChenWeighted {
    fn from(v: Vec<ChenWeightedPair>) -> Self {
        ChenWeighted(v)
    }
}

impl From<Vec<TwoCard>> for ChenWeighted {
    fn from(v: Vec<TwoCard>) -> Self {
        ChenWeighted::from(
            v.into_iter()
                .map(ChenWeightedPair::from)
                .collect::<Vec<ChenWeightedPair>>(),
        )
    }
}

impl From<TwoCards> for ChenWeighted {
    fn from(two_cards: TwoCards) -> Self {
        ChenWeighted::from(two_cards.hands)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types_arrays_chen_weighted {
    use super::*;

    #[test]
    fn sort() {
        let _all = ChenWeighted::all().sort();
        // for pair in all.iter() {
        //     // println!("\n>>>>>> {}", pair);
        //     // println!(">>>>>>");
        // }
    }
}
