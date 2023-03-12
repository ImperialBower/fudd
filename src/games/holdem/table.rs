use crate::analysis::chances::Chances;
use crate::analysis::eval::Eval;
use crate::analysis::outs::Outs;
use crate::games::holdem::board::Board;
use crate::games::holdem::case_eval::CaseEval;
use crate::games::holdem::case_evals::CaseEvals;
use crate::games::holdem::seat_eval::SeatEval;
use crate::games::holdem::seats::Seats;
use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use ckc_rs::HandError;
use rand::Rng;
// use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Table {
    pub players: Seats,
    pub board: Board,
}

impl Table {
    #[must_use]
    pub fn seat(number: usize) -> Table {
        Table {
            players: Seats::seat(number),
            board: Board::default(),
        }
    }

    /// Takes a Card index string and returns a `Table` entity with the `Players`
    /// and `Board` entities all having cards dealt to them.
    ///
    /// # Errors
    ///
    /// Throws a `HandError` if the cards passed in aren't valid and at least nine;
    /// five for the `Board` and the remaining for the `Players`, which must be
    /// divisible by 2.
    ///
    pub fn from_index(index: &'static str) -> Result<Table, HandError> {
        Table::try_from(index)
    }

    /// Static factory method that generates a sample Table entity prepopulated
    /// with randomly generated data for between 2 to 9 players.
    #[must_use]
    pub fn sample() -> Table {
        let mut rng = rand::thread_rng();
        let player_count: usize = rng.gen_range(2..9);
        Table::sample_number(player_count)
    }

    #[must_use]
    pub fn sample_number(player_count: usize) -> Table {
        let table = Table::seat(player_count);
        let mut cards = PlayingCards::deck_shuffled();

        for _ in 0..(player_count * 2) + 5 {
            table.take(cards.draw_one());
        }
        table
    }

    pub fn chances_at_deal(&self) -> Chances {
        self.eval_at_deal().chances()
    }

    pub fn chances_at_flop(&self) -> Chances {
        self.eval_at_flop().chances()
    }

    pub fn chances_at_turn(&self) -> Chances {
        self.eval_at_turn().chances()
    }

    pub fn chances_at_river(&self) -> Chances {
        self.eval_at_river().chances()
    }

    pub fn dealt(&self) -> PlayingCards {
        PlayingCards::default()
            .combine(&self.players.dealt())
            .combine(&self.board.dealt())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn eval_at_deal(&self) -> CaseEvals {
        if !self.players.is_dealt() {
            return CaseEvals::default();
        }
        let mut evals = CaseEvals::default();
        for v in self.remaining_at_deal().combinations(5) {
            let cycle = PlayingCards::from(v);
            evals.push(self.players.case_eval(&cycle));
        }
        evals
    }

    /// Permutates through every possible combination of `PokerCards` based upon a specific
    /// `Flop` and all the remaining `PokerCards` that are not in play.
    ///
    /// **NOTE** This method does not take into account any burned cards, only dealing with
    /// `PokerCards` that are not actively being held by players at the `Table` or the `Flop`.
    pub fn eval_at_flop(&self) -> CaseEvals {
        if !self.board.flop.is_dealt() {
            return CaseEvals::default();
        }
        let mut evals = CaseEvals::default();
        for v in self.remaining_at_flop().combinations(2) {
            let mut cycle = PlayingCards::from(v);
            cycle.append(&self.board.flop.to_playing_cards());
            evals.push(self.players.case_eval(&cycle));
        }
        evals
    }

    pub fn eval_at_turn(&self) -> CaseEvals {
        let (_, case_evals) = self.eval_at_turn_with_outs();
        case_evals
    }

    /// # Panics
    ///
    /// Will only panic if the remaining() function is seriously out of whack.
    pub fn eval_at_turn_with_outs(&self) -> (Outs, CaseEvals) {
        if !self.board.turn.is_dealt() {
            return (Outs::default(), CaseEvals::default());
        }
        let mut evals = CaseEvals::default();
        let mut outs = Outs::default();
        let rem = self.remaining_at_turn();
        for i1 in 0..rem.len() {
            let card = *rem.get_index(i1).unwrap();
            let mut cycle = PlayingCards::from(card);
            cycle.append(&self.board.flop.to_playing_cards());
            cycle.append(&self.board.turn.to_playing_cards());

            let (out, case) = self.players.case_eval_with_outs(card, &cycle);
            outs.extend(&out);
            evals.push(case);
        }
        (outs, evals)
    }

    pub fn eval_at_river(&self) -> CaseEvals {
        if !self.board.flop.is_dealt() {
            return CaseEvals::default();
        }
        let mut evals = CaseEvals::default();

        evals.push(self.players.case_eval(&self.board.to_playing_cards()));

        evals
    }

    pub fn format_calc(&self) -> String {
        format!(
            "❯ cargo run --example calc -- -d \"{}\" -b \"{}\"",
            self.players.to_playing_cards(),
            self.board.to_playing_cards()
        )
    }

    pub fn format_indexer(&self) -> String {
        format!(
            "❯ cargo run --example indexer -- --index \"{}\"",
            self.dealt()
        )
    }

    pub fn flop_seat_evals(&self) -> CaseEval {
        let mut evals = CaseEval::default();
        for seat in self.players.iter() {
            evals.push(SeatEval::new_from_flop(seat.clone(), &self.board.flop));
        }
        evals
    }

    pub fn nuts_at_flop(&self) -> Eval {
        self.board.flop.the_nuts()
    }

    pub fn player_cards_at_flop(&self, player: usize) -> PlayingCards {
        match self.players.get(player) {
            Some(player) => player
                .to_playing_cards()
                .combine(&self.board.flop.to_playing_cards()),
            None => PlayingCards::default(),
        }
    }

    pub fn player_eval_at_flop(&self, player: usize) -> Eval {
        match self.player_cards_at_flop(player).to_five_cards() {
            Ok(hand) => Eval::from(hand),
            Err(_) => Eval::default(),
        }
    }

    pub fn remaining_at_deal(&self) -> PlayingCards {
        self.remaining()
            .combine(&self.board.flop.to_playing_cards())
            .combine(&self.board.turn.to_playing_cards())
            .combine(&self.board.river.to_playing_cards())
    }

    pub fn remaining_at_flop(&self) -> PlayingCards {
        self.remaining()
            .combine(&self.board.turn.to_playing_cards())
            .combine(&self.board.river.to_playing_cards())
    }

    pub fn remaining_at_turn(&self) -> PlayingCards {
        self.remaining()
            .combine(&self.board.river.to_playing_cards())
    }

    pub fn tied_or_better_at_flop(&self, eval: &Eval) -> CaseEval {
        let mut evals = CaseEval::default();
        for seat in self.players.iter() {
            let seat_eval = SeatEval::new_from_flop(seat.clone(), &self.board.flop);
            if seat_eval.eval.rank >= eval.rank {
                evals.push(seat_eval);
            }
        }
        evals
    }

    /// Used for printing out a demo of the `Table`. See the examples folder for it in action.
    #[allow(clippy::missing_panics_doc)]
    pub fn play_out(&self) {
        println!("Cards Dealt: {}\n", self.to_playing_cards());
        println!("{}", self.players);
        println!("{}", self.board);

        if self.play_out_flop() && self.play_out_turn() {
            self.play_out_river();
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_detailed(&self) {
        println!("Cards Dealt: {}\n", self.to_playing_cards());
        println!("{}", self.players);
        println!("{}", self.board);

        if self.play_out_flop() && self.play_out_possible_hands_at_flop() && self.play_out_turn() {
            self.play_out_river();
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_deal(&self) -> bool {
        if self.players.len() < 2 {
            return false;
        }
        println!("{}", self.play_out_deal_fmt());
        true
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_deal_fmt(&self) -> String {
        if self.players.len() < 2 {
            "Not enough players".to_string()
        } else {
            let mut s = String::new();
            let chances = self.chances_at_deal();
            for k in chances.keys() {
                let row = format!(
                    "Seat #{} {}: {:.1}% ",
                    k,
                    self.players.get(*k).unwrap(),
                    chances.get(*k)
                );
                s.push_str(row.as_str());
            }
            s
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_flop(&self) -> bool {
        if self.players.len() < 2 || !self.board.flop.is_dealt() {
            return false;
        }

        let chances = self.chances_at_flop();
        println!("\nThe Flop: {}", self.board.flop);
        println!("Chances of winning:");
        for k in chances.keys() {
            println!(
                "Seat #{} {}: {:.1}% - CURRENT HAND: {}",
                k,
                self.players.get(*k).unwrap(),
                chances.get(*k),
                self.player_eval_at_flop(*k)
            );
        }

        let the_nuts = self.nuts_at_flop();

        println!("\nThe Nuts would be: {the_nuts}");

        for seat_eval in self.who_flopped_the_nuts().iter() {
            println!(
                "!! Player {} flopped the nuts {} !!",
                seat_eval.seat.number, seat_eval.eval
            );
        }

        true
    }

    /// Prints out all possible hands at the flop, in order of hand strength.
    pub fn play_out_possible_hands_at_flop(&self) -> bool {
        if self.players.len() < 2 || !self.board.flop.is_dealt() {
            false
        } else {
            println!("\nPossible hands at the flop, sorted by strength:");

            for (v, e) in self.board.flop.all_possible().indexed().index_map() {
                println!("CKC #{v} {e}");
            }
            println!("See https://suffe.cool/poker/7462.html for a listing of all CKC numbers.");

            true
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_turn(&self) -> bool {
        if self.board.turn.is_dealt() {
            let (outs, case_evals) = self.eval_at_turn_with_outs();
            let chances = case_evals.chances();
            println!("\nThe Turn: {}", self.board.turn);
            println!("Chances of winning:");
            //
            // let winner = case_evals.winners().winners();
            //
            // for k in chances.keys() {
            //     if winner.has_seat(*k) {
            //         println!(
            //             "Seat {}: {:.1}% - Best Hand: {}",
            //             k,
            //             chances.get(*k),
            //             winner.get_seat(*k).unwrap().eval
            //         );
            //     } else {
            //         let player_outs = outs.get_unless_most(*k);
            //         match player_outs {
            //             Some(o) => println!("Seat {}: {:.1}% - Outs: {}", k, chances.get(*k), o),
            //             None => println!("Seat {}: {:.1}%", k, chances.get(*k)),
            //         };
            //     }
            // }

            chances.playout_with_outs(&outs);

            true
        } else {
            false
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn play_out_river(&self) {
        if self.board.river.is_dealt() {
            let case_evals = self.eval_at_river();
            let chances = case_evals.chances();

            println!("\nThe River: {}", self.board.river);

            // TODO: Tighter display at river
            // for k in chances.keys() {
            //
            // }

            chances.playout();

            let winners = case_evals.winners();
            println!("\nWinners:");
            for winner in winners.iter() {
                println!("   Seat {}: {}", winner.seat.number, winner.eval);
            }
        }
    }

    pub fn who_flopped_the_nuts(&self) -> CaseEval {
        self.tied_or_better_at_flop(&self.nuts_at_flop())
    }
}

impl CardSlot for Table {
    fn take(&self, card: PlayingCard) -> bool {
        if self.players.is_dealt() {
            self.board.take(card)
        } else {
            self.players.take(card)
        }
    }

    fn fold(&self) -> PlayingCards {
        let mut cards = PlayingCards::default();
        let folded = self.players.fold();
        cards.append(&folded);
        let folded = self.board.fold();
        cards.append(&folded);
        cards
    }

    fn is_dealt(&self) -> bool {
        self.players.is_dealt() && self.board.is_dealt()
    }

    fn to_playing_cards(&self) -> PlayingCards {
        let mut cards = PlayingCards::default();
        cards.append(&self.players.to_playing_cards());
        cards.append(&self.board.to_playing_cards());
        cards
    }
}

impl Display for Table {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        out.entry(&(format!("PLAYERS: {}", self.players)).fmt_display());
        out.entry(&(format!("BOARD: {}", self.board)).fmt_display());

        out.finish()
    }
}

impl TryFrom<&'static str> for Table {
    type Error = HandError;

    /// Takes a Card index string and returns a `Table` entity with the `Players`
    /// and `Board` entities all having cards dealt to them.
    ///
    /// # Errors
    ///
    /// Throws a `HandError` if the cards passed in aren't valid and at least nine;
    /// five for the `Board` and the remaining for the `Players`, which must be
    /// divisible by 2.
    ///
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let playing_cards = PlayingCards::try_from(value);
        match playing_cards {
            Ok(mut cards) => {
                // There have to be at least 9 cards to be a valid Table.
                if cards.len() < 9 {
                    Err(HandError::NotEnoughCards)
                } else if cards.len() % 2 != 1 {
                    Err(HandError::InvalidCardCount)
                } else {
                    let mut table = Table::default();
                    table
                        .board
                        .take_from_playing_cards(&cards.draw_from_the_bottom(5));
                    match Seats::try_from(cards) {
                        Ok(players) => {
                            table.players = players;
                            Ok(table)
                        }
                        _ => Err(HandError::InvalidIndex),
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_table_tests {
    use super::*;

    #[test]
    fn eval_from_flop() {
        let player_count: usize = 3;
        let table = Table::seat(player_count);
        let index = "Q♠ 4♦ 8♥ 5♦ 2♠ 4♠ J♦ 7♠ 9♣ 8♠ 6♦ 2♦ J♥ Q♥ 6♠ 4♥ T♣ 4♣ 6♣ 8♦ A♣ 3♣ A♥ 8♣ J♠ 9♠ 7♣ T♠ Q♣ T♦ 9♦ 5♠ 7♥ 2♣ 3♥ 5♣ 7♦ 3♦ 3♠ Q♦ K♥ A♠ K♦ 9♥ K♠ 5♥ K♣ 2♥ T♥ 6♥ A♦ J♣";
        let mut cards = PlayingCards::try_from(index).unwrap();

        for _ in 0..(player_count * 2) + 5 {
            table.take(cards.draw_one());
        }

        let evals = table.eval_at_flop();
        assert_eq!(903, evals.len())
    }

    #[test]
    fn from_index() {
        let index = "6♣ J♥ 7♠ 8♠ 9♦ 2♣ 6♠ T♥ 3♥ 9♥ 9♠";

        let table = Table::from_index(index).unwrap();

        assert_eq!(index, table.dealt().to_string());
        assert_eq!(
            "[Seat 0: 6♣ J♥, Seat 1: 7♠ 8♠, Seat 2: 9♦ 2♣]",
            table.players.to_string()
        );
        assert_eq!(
            "[FLOP:  6♠ T♥ 3♥, TURN:  9♥, RIVER: 9♠]",
            table.board.to_string()
        );
    }

    #[test]
    fn from_index__invalid_card() {
        let index = "66 J♥ 7♠ 8♠ 9♦ 2♣ 6♠ T♥ 3♥ 9♥ 9♠";

        let table = Table::from_index(index);

        assert!(table.is_err());
        assert_eq!(HandError::InvalidCard, table.unwrap_err());
    }

    #[test]
    fn from_index__not_enough_cards() {
        let index = "9♦ 2♣ 6♠ T♥ 3♥ 9♥ 9♠";

        let table = Table::from_index(index);

        assert!(table.is_err());
        assert_eq!(HandError::NotEnoughCards, table.unwrap_err());
    }

    #[test]
    fn from_index__invalid_card_count() {
        let index = "J♥ 7♠ 8♠ 9♦ 2♣ 6♠ T♥ 3♥ 9♥ 9♠";

        let table = Table::from_index(index);

        assert!(table.is_err());
        assert_eq!(HandError::InvalidCardCount, table.unwrap_err());
    }

    #[test]
    fn take() {
        let table = Table::seat(3);
        let index = "Q♠ 4♦ 8♥ 5♦ 2♠ 4♠ J♦ 7♠ 9♣ 8♠ 6♦ 2♦ J♥ Q♥ 6♠ 4♥ T♣ 4♣ 6♣ 8♦ A♣ 3♣ A♥ 8♣ J♠ 9♠ 7♣ T♠ Q♣ T♦ 9♦ 5♠ 7♥ 2♣ 3♥ 5♣ 7♦ 3♦ 3♠ Q♦ K♥ A♠ K♦ 9♥ K♠ 5♥ K♣ 2♥ T♥ 6♥ A♦ J♣";
        let mut cards = PlayingCards::try_from(index).unwrap();

        table.take(cards.draw_one());
        table.take(cards.draw_one());
        table.take(cards.draw_one());
        assert_eq!(
            "[Seat 0: Q♠ __, Seat 1: 4♦ __, Seat 2: 8♥ __]",
            table.players.to_string()
        );
        assert_eq!("Q♠ 4♦ 8♥", table.players.dealt().to_string());
        assert_eq!("", table.board.dealt().to_string());
        assert!(!table.players.is_dealt());
        assert!(!table.board.is_dealt());

        table.take(cards.draw_one());
        table.take(cards.draw_one());
        table.take(cards.draw_one());
        assert_eq!(
            "[Seat 0: Q♠ 5♦, Seat 1: 4♦ 2♠, Seat 2: 8♥ 4♠]",
            table.players.to_string()
        );
        assert_eq!("Q♠ 5♦ 4♦ 2♠ 8♥ 4♠", table.players.dealt().to_string());
        assert_eq!("", table.board.dealt().to_string());
        assert_eq!("Q♠ 5♦ 4♦ 2♠ 8♥ 4♠", table.to_playing_cards().to_string());
        assert!(table.players.is_dealt());
        assert!(!table.board.is_dealt());

        table.take(cards.draw_one());
        table.take(cards.draw_one());
        table.take(cards.draw_one());
        table.take(cards.draw_one());
        table.take(cards.draw_one());
        assert_eq!("J♦ 7♠ 9♣ 8♠ 6♦", table.board.dealt().to_string());
        assert_eq!(
            "Q♠ 5♦ 4♦ 2♠ 8♥ 4♠ J♦ 7♠ 9♣ 8♠ 6♦",
            table.to_playing_cards().to_string()
        );
        assert!(table.players.is_dealt());
        assert!(table.board.is_dealt());
    }

    #[test]
    fn display() {
        assert_eq!(
            "[PLAYERS: [], BOARD: [FLOP:  __ __ __, TURN:  __, RIVER: __]]",
            Table::default().to_string()
        );
    }

    #[test]
    #[ignore]
    fn playout_from_turn() {
        let mut table = Table::default();
        table.players = Seats::from_index("4♥ 3♥ A♦ J♣ 8♦ 8♣").unwrap();
        table.board = Board::from_index("T♥ J♠ J♥ 3♦").unwrap();

        table.play_out_flop();
        table.play_out_turn();
        table.play_out_river();
    }
}
