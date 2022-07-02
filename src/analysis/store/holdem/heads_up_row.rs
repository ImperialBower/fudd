use crate::games::holdem::hand::Hand;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct HeadsUpRow {
    pub hands: String,
    pub first_wins: usize,
    pub second_wins: usize,
    pub ties: usize,
}

impl HeadsUpRow {
    #[must_use]
    pub fn new(
        first: Hand,
        second: Hand,
        first_wins: usize,
        second_wins: usize,
        ties: usize,
    ) -> HeadsUpRow {
        HeadsUpRow {
            hands: format!("{} {}", first, second),
            first_wins,
            second_wins,
            ties,
        }
    }

    #[must_use]
    pub fn calc(first: Hand, second: Hand) -> HeadsUpRow {
        HeadsUpRow::new(first, second, 0, 0, 0)
    }
}

impl fmt::Display for HeadsUpRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.hands, self.first_wins, self.second_wins, self.ties,
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod analysis_store_holdem_heads_up_row {
    use super::*;
    use crate::types::playing_card::PlayingCard;

    #[test]
    fn display() {
        let hand = HeadsUpRow::new(
            Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::KING_DIAMONDS),
            Hand::new(PlayingCard::ACE_SPADES, PlayingCard::KING_SPADES),
            0,
            0,
            0,
        );

        assert_eq!("A♣ K♦ A♠ K♠,0,0,0", hand.to_string());
    }
}
