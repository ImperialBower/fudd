use crate::analysis::eval::Eval;
use crate::games::holdem::hand::Hand;
use crate::types::arrays::Vectorable;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::{PileOfCards, U32Card};
use ckc_rs::PokerCard;
use itertools::Itertools;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::{fmt, mem};
use wincounter::{Win, Wins};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct HeadsUp {
    pub first: Hand,
    pub second: Hand,
}

impl HeadsUp {
    const PREFLOP_COMBO_COUNT: usize = 1_712_304;
    const DEFAULT_WORKER_COUNT: usize = 10;

    #[must_use]
    pub fn new(first: Hand, second: Hand) -> HeadsUp {
        if first.is_blank() || second.is_blank() {
            HeadsUp::default()
        } else if first > second {
            HeadsUp { first, second }
        } else {
            HeadsUp {
                first: second,
                second: first,
            }
        }
    }

    #[must_use]
    pub fn from_index(index: &'static str) -> HeadsUp {
        HeadsUp::from(index)
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.has(PlayingCard::BLANK)
    }

    #[must_use]
    pub fn seven_from(&self, five: &PlayingCards) -> (PlayingCards, PlayingCards) {
        (
            five.combine(&self.first.to_playing_cards()),
            five.combine(&self.second.to_playing_cards()),
        )
    }

    /// # Panics
    ///
    /// Shouldn't ever.
    #[must_use]
    pub fn best_from_seven(&self, five: &PlayingCards) -> (Eval, Eval) {
        let (first_seven, second_seven) = self.seven_from(five);
        (
            first_seven.eval_7cards().unwrap(),
            second_seven.eval_7cards().unwrap(),
        )
    }

    //region -> Result Preflop <-

    #[must_use]
    pub fn odds_preflop(&self) -> wincounter::result::HeadsUp {
        self.odds_preflop_with_worker_count(HeadsUp::DEFAULT_WORKER_COUNT)
    }

    #[must_use]
    pub fn odds_preflop_with_worker_count(
        &self,
        worker_count: usize,
    ) -> wincounter::result::HeadsUp {
        let wins = self.wins_preflop_with_worker_count(worker_count);
        let (first, ties) = wins.wins_for(Win::FIRST);
        let (second, _) = wins.wins_for(Win::SECOND);
        wincounter::result::HeadsUp::new(first - ties, second - ties, ties)
    }

    #[must_use]
    pub fn odds_to_string(&self, heads_up: wincounter::result::HeadsUp) -> String {
        format!(
            "{} {}, {:.2}% ({}), {:.2}% ({}), {:.2}% ({})",
            self.first,
            self.second,
            heads_up.percentage_first(),
            heads_up.first_wins,
            heads_up.percentage_second(),
            heads_up.second_wins,
            heads_up.percentage_ties(),
            heads_up.ties
        )
    }

    #[must_use]
    pub fn wins_preflop(&self) -> Wins {
        self.wins_preflop_with_worker_count(HeadsUp::DEFAULT_WORKER_COUNT)
    }

    #[allow(unused_must_use, clippy::comparison_chain)]
    #[must_use]
    pub fn wins_preflop_with_worker_count(&self, worker_count: usize) -> Wins {
        let mut wins = Wins::default();
        let remaining = self.remaining();
        let combos = remaining.combinations(5);

        let chunks = combos.chunks((HeadsUp::PREFLOP_COMBO_COUNT / worker_count).max(1));
        let (sender, receiver) = mpsc::channel();

        for chunk in &chunks {
            for combo in chunk {
                let sender = sender.clone();

                let board = PlayingCards::from(combo);
                let (eval1, eval2) = self.best_from_seven(&board);

                if eval1.rank > eval2.rank {
                    sender.send(Win::FIRST);
                } else if eval2.rank > eval1.rank {
                    debug!("   Player 2 Wins: {} - {}", board, eval2);
                    sender.send(Win::SECOND);
                } else {
                    debug!("   Tie: {} - {} / {}", board, eval1, eval2);
                    sender.send(Win::FIRST | Win::SECOND);
                }
            }
        }

        mem::drop(sender);

        for received in receiver {
            wins.add_win(received);
        }

        wins
    }

    //endregion -> Result Preflop <-

    // pub fn types() -> Vec<&str> {
    //     vec![
    //         "A♠ A♥ A♦ A♣",  // EQUALS
    //         "A♠ A♥ A♦ K♦",  // Dominated / Connector / Suited
    //         "A♠ A♥ A♦ K♠",  // Dominated / Partially Covered / Connector / Off
    //         "A♠ A♥ K♠ K♥",  // Dominated / Covered / Connector / Off
    //         "A♠ A♥ K♠ K♥",  // Dominated / Covered / Connector / Off
    //     ]
    // }
}

impl fmt::Display for HeadsUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first, self.second,)
    }
}

impl From<&'static str> for HeadsUp {
    fn from(s: &'static str) -> Self {
        let v: Vec<&str> = s.split_whitespace().collect();
        if v.len() == 4 {
            HeadsUp::new(
                Hand::new(
                    PlayingCard::from(*v.get(0).unwrap()),
                    PlayingCard::from(*v.get(1).unwrap()),
                ),
                Hand::new(
                    PlayingCard::from(*v.get(2).unwrap()),
                    PlayingCard::from(*v.get(3).unwrap()),
                ),
            )
        } else {
            HeadsUp::default()
        }
    }
}

impl PileOfCards<PlayingCard> for HeadsUp {
    fn has(&self, playing_card: PlayingCard) -> bool {
        self.first.has(playing_card) || self.second.has(playing_card)
    }
}

impl PileOfCards<U32Card> for HeadsUp {
    fn has(&self, card_number: U32Card) -> bool {
        self.first.has(card_number) || self.second.has(card_number)
    }
}

impl Vectorable for HeadsUp {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        vec![
            self.first.first().as_u32(),
            self.first.second().as_u32(),
            self.second.first().as_u32(),
            self.second.second().as_u32(),
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod games_holdem_heads_up_tests {
    use super::*;
    use crate::types::playing_card::PlayingCard;
    use ckc_rs::CardNumber;

    #[test]
    fn new() {
        let aces = Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES);
        let kq = Hand::new(PlayingCard::QUEEN_DIAMONDS, PlayingCard::KING_DIAMONDS);

        let headsup = HeadsUp::new(kq, aces);

        assert_eq!(
            headsup.first,
            Hand::new(PlayingCard::ACE_SPADES, PlayingCard::ACE_CLUBS)
        );
        assert_eq!(
            headsup.second,
            Hand::new(PlayingCard::KING_DIAMONDS, PlayingCard::QUEEN_DIAMONDS)
        );
        assert_eq!(
            headsup,
            HeadsUp {
                first: Hand::new(PlayingCard::ACE_SPADES, PlayingCard::ACE_CLUBS),
                second: Hand::new(PlayingCard::KING_DIAMONDS, PlayingCard::QUEEN_DIAMONDS)
            }
        )
    }

    #[test]
    fn is_blank() {
        assert!(HeadsUp::default().is_blank());
        assert!(HeadsUp::new(
            Hand::default(),
            Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES)
        )
        .is_blank());
        assert!(HeadsUp::new(
            Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES),
            Hand::default()
        )
        .is_blank());
        assert!(!HeadsUp::new(
            Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES),
            Hand::new(PlayingCard::ACE_DIAMONDS, PlayingCard::JACK_CLUBS)
        )
        .is_blank());
    }

    #[test]
    fn display() {
        assert_eq!(
            "A♥ Q♥ K♥ J♥",
            HeadsUp::new(
                Hand::new(PlayingCard::JACK_HEARTS, PlayingCard::KING_HEARTS),
                Hand::new(PlayingCard::QUEEN_HEARTS, PlayingCard::ACE_HEARTS)
            )
            .to_string()
        );
    }

    #[test]
    fn has() {
        let aces = Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES);
        let kq = Hand::new(PlayingCard::QUEEN_DIAMONDS, PlayingCard::KING_DIAMONDS);
        let headsup = HeadsUp::new(kq, aces);

        assert!(headsup.has(PlayingCard::ACE_CLUBS));
        assert!(headsup.has(CardNumber::QUEEN_DIAMONDS));
        assert!(!headsup.has(PlayingCard::JACK_CLUBS));
        assert!(!headsup.has(CardNumber::NINE_SPADES));
    }

    #[test]
    fn to_vec() {
        let aces = Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::ACE_SPADES);
        let kq = Hand::new(PlayingCard::QUEEN_DIAMONDS, PlayingCard::KING_DIAMONDS);

        let headsup = HeadsUp::new(kq, aces);

        assert_eq!(
            headsup.to_vec(),
            vec![
                CardNumber::ACE_SPADES,
                CardNumber::ACE_CLUBS,
                CardNumber::KING_DIAMONDS,
                CardNumber::QUEEN_DIAMONDS
            ]
        );
    }

    #[test]
    fn odds_to_string() {
        let hup = HeadsUp::from("6♠ 6♥ 5♦ 5♣");
        let result = wincounter::result::HeadsUp::new(1365284, 314904, 32116);

        assert_eq!(
            "6♠ 6♥ 5♦ 5♣, 79.73% (1365284), 18.39% (314904), 1.88% (32116)",
            hup.odds_to_string(result).to_string()
        );
    }

    #[test]
    fn vectorable__remaining() {
        let hup = HeadsUp::from("6♠ 6♥ 5♦ 5♣");

        assert_eq!(48, hup.remaining().len());
        assert_eq!(
            HeadsUp::PREFLOP_COMBO_COUNT,
            hup.remaining().combinations(5).count()
        );
    }
}
