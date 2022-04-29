use crate::games::holdem::heads_up::HeadsUp;
use crate::types::arrays::Vectorable;
use crate::types::playing_cards::PlayingCards;
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HeadsUpOdds {
    hands: HeadsUp,
    pub first_win: f32,
    pub second_win: f32,
    pub tie: f32,
}

impl HeadsUpOdds {
    #[must_use]
    pub fn calculate(hands: HeadsUp) -> HeadsUpOdds {
        if hands.is_blank() {
            return HeadsUpOdds::default();
        }

        for v in hands.remaining().combinations(5) {
            let (_first_seven, _second_seven) = hands.best_from_seven(&PlayingCards::from(v));
        }

        HeadsUpOdds::default()
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.hands.is_blank()
    }

    #[must_use]
    pub fn is_calculated(&self) -> bool {
        (self.first_win != 0.0) || (self.second_win != 0.0) || (self.tie != 0.0)
    }
}

impl fmt::Display for HeadsUpOdds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {:.1}% - {}: {:.1}% - TIE: {:.1}%",
            self.hands.first, self.first_win, self.hands.second, self.second_win, self.tie,
        )
    }
}

impl From<HeadsUp> for HeadsUpOdds {
    fn from(_heads_up: HeadsUp) -> Self {
        HeadsUpOdds::default()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod analysis_store_holdem_heads_up_odds {
    use super::*;

    #[test]
    #[ignore]
    fn calculate() {
        let odds = HeadsUpOdds::calculate(HeadsUp::from("6♠ 6♥ 5♦ 5♣"));
        assert!(odds.is_calculated());
        assert_eq!("6♠ 6♥: 79.7% - 5♦ 5♣: 18.4% - TIE: 1.9%", odds.to_string());
    }

    #[test]
    fn is_blank() {
        assert!(HeadsUpOdds::default().is_blank());
        assert!(!uncalculatedHand().is_blank());
    }

    #[test]
    fn is_calculated() {
        assert!(calculatedHand().is_calculated());
        assert!(!HeadsUpOdds::default().is_calculated());
        assert!(!uncalculatedHand().is_calculated());
    }

    #[test]
    fn display() {
        assert_eq!(
            "6♠ 6♥: 79.7% - 5♦ 5♣: 18.4% - TIE: 1.9%",
            calculatedHand().to_string()
        );
    }

    fn uncalculatedHand() -> HeadsUpOdds {
        HeadsUpOdds {
            hands: HeadsUp::from("A♥ Q♥ K♥ J♥"),
            first_win: 0.0,
            second_win: 0.0,
            tie: 0.0,
        }
    }

    fn calculatedHand() -> HeadsUpOdds {
        HeadsUpOdds {
            hands: HeadsUp::from("6♠ 6♥ 5♦ 5♣"),
            first_win: 79.7,
            second_win: 18.4,
            tie: 1.9,
        }
    }
}
