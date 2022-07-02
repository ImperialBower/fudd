use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::seven_card::SevenCard;
use crate::types::arrays::Vectorable;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use ckc_rs::cards::binary_card::{BinaryCard, BC64};
use ckc_rs::cards::five::Five;
use ckc_rs::cards::seven::Seven;
use ckc_rs::cards::{HandRanker, HandValidator};
use ckc_rs::hand_rank::HandRankValue;
use csv::Reader;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

lazy_static! {
    /// This is a lazy loaded `HashMap` of every combination of `Five` and `Seven` cards combinations
    /// with their resulting `HandRankValue` and the `Five` cards that make up the best hand.
    pub static ref BC_RANK: HashMap<BinaryCard, SimpleBinaryCardMap> = {
        let mut m = HashMap::new();
        let file_path = "logs/bcm.csv";
        let file = File::open(file_path).unwrap();
        let mut rdr = Reader::from_reader(file);

        for result in rdr.deserialize() {
            let bcm: BinaryCardMap = result.unwrap();
            m.insert(bcm.bc, SimpleBinaryCardMap::from(bcm));
        }
        m
    };
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SimpleBinaryCardMap {
    pub bc: BinaryCard,
    pub rank: HandRankValue,
}

impl SimpleBinaryCardMap {
    #[must_use]
    pub fn new(bc: BinaryCard, rank: HandRankValue) -> SimpleBinaryCardMap {
        SimpleBinaryCardMap { bc, rank }
    }
}

impl From<BinaryCardMap> for SimpleBinaryCardMap {
    fn from(bcm: BinaryCardMap) -> Self {
        SimpleBinaryCardMap::new(bcm.best, bcm.rank)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BinaryCardMap {
    pub bc: BinaryCard,
    pub best: BinaryCard,
    pub rank: HandRankValue,
}

impl BinaryCardMap {
    /// # Errors
    ///
    /// Not sure why it would return one TBH
    pub fn generate(path: &str) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(path)?;
        let deck = PlayingCards::deck();

        for b in deck.combinations(5) {
            wtr.serialize(BinaryCardMap::from(b))?;
        }

        for b in deck.combinations(7) {
            wtr.serialize(BinaryCardMap::from(b))?;
        }

        wtr.flush()?;
        Ok(())
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        *self == BinaryCardMap::default()
    }
}

impl From<Five> for BinaryCardMap {
    fn from(five: Five) -> Self {
        if five.is_blank() {
            return BinaryCardMap::default();
        }
        let (rank, five) = five.hand_rank_value_and_hand();
        let bc = BinaryCard::from_five(five);
        let best = bc;

        BinaryCardMap { bc, best, rank }
    }
}

impl From<Seven> for BinaryCardMap {
    fn from(seven: Seven) -> Self {
        if !seven.is_valid() {
            return BinaryCardMap::default();
        }
        let (rank, five) = seven.hand_rank_value_and_hand();
        let bc = BinaryCard::from_seven(seven);
        let best = BinaryCard::from_five(five);

        BinaryCardMap { bc, best, rank }
    }
}

impl From<Vec<&PlayingCard>> for BinaryCardMap {
    fn from(v: Vec<&PlayingCard>) -> Self {
        match v.len() {
            5 => {
                let five_card_result = FiveCard::try_from(v);
                let five = match five_card_result {
                    Ok(five_card) => five_card.to_five(),
                    Err(_) => Five::default(),
                };
                BinaryCardMap::from(five)
            }
            7 => {
                let seven_card_result = SevenCard::try_from(v);
                let seven = match seven_card_result {
                    Ok(seven_card) => seven_card.to_seven(),
                    Err(_) => Seven::default(),
                };
                BinaryCardMap::from(seven)
            }
            _ => BinaryCardMap::default(),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod generic_tests {
    use super::*;

    #[test]
    fn from_five() {
        let five = Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap();

        let sut = BinaryCardMap::from(five);

        assert_eq!(sut.rank, 1);
        assert_eq!(sut.bc, 4_362_862_139_015_168);
        assert_eq!(sut.best, 4_362_862_139_015_168);
    }

    #[test]
    fn from_five__default() {
        assert_eq!(
            BinaryCardMap::default(),
            BinaryCardMap::from(Five::default())
        );
    }

    #[test]
    fn is_blank() {
        assert!(BinaryCardMap::default().is_blank());
        assert!(!BinaryCardMap::from(Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap()).is_blank());
    }
}
