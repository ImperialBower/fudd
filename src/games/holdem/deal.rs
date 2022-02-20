use crate::analysis::eval::Eval;
use crate::games::holdem::seats::Seats;
use crate::games::holdem::table::Table;
use crate::types::card_slot;
use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use rand::Rng;
use std::cmp::Ordering;
use strum_macros::Display;
use strum_macros::EnumIter;

/// TODO:
///     * Run it X number of times.
#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StageInTheHand {
    ShuffleUp,
    Deal,
    Flop,
    Turn,
    TheRiver,
}

#[derive(Clone, Debug)]
pub struct Deal {
    stage: StageInTheHand,
    pub deck: PlayingCards,
    pub table: Table,
    burned: PlayingCards,
}

impl Deal {
    #[must_use]
    pub fn new() -> Deal {
        Deal {
            stage: StageInTheHand::ShuffleUp,
            deck: PlayingCards::deck_shuffled(),
            table: Table::default(),
            burned: PlayingCards::default(),
        }
    }

    #[must_use]
    pub fn from_deck(deck: PlayingCards) -> Option<Deal> {
        let mut table = Deal::new();
        table.deck = deck;

        if table.deck.len() == 52 {
            Some(table)
        } else {
            None
        }
    }

    /// Returns a random `Deal` with a random number of `Players`,
    /// between 2 to 11.
    #[must_use]
    pub fn sample() -> (Deal, PlayingCards) {
        let mut rng = rand::thread_rng();
        let mut deal = Deal::default();
        let poker_cards = deal.deck.clone();
        deal.deal(rng.gen_range(2..11));
        deal.fast_forward();
        (deal, poker_cards)
    }

    /// Removes a card from the deck, and places it in the burned collection of `PokerCards`.
    pub fn burn(&mut self) {
        self.burned.insert(self.deck.draw_one());
    }

    pub fn deal(&mut self, seats: usize) {
        self.burn();
        self.table.players = Seats::seat(seats);

        for _ in 0..(seats * 2) {
            self.table.players.take(self.deck.draw_one());
        }

        self.stage = StageInTheHand::Deal;
    }

    pub fn fast_forward(&mut self) -> PlayingCards {
        self.flop();
        self.turn();
        self.river();
        self.table.board.to_playing_cards()
    }

    pub fn flop(&mut self) -> PlayingCards {
        self.burn();
        self.table.board.flop.take(self.deck.draw_one());
        self.table.board.flop.take(self.deck.draw_one());
        self.table.board.flop.take(self.deck.draw_one());
        self.stage = StageInTheHand::Flop;
        self.table.board.flop.to_playing_cards()
    }

    pub fn fold(&mut self, seat: usize) -> bool {
        let folded = self.table.players.fold_player(seat);

        match folded {
            Some(cards) => {
                for card in cards.iter() {
                    self.burned.insert(*card);
                }
                true
            }
            None => false,
        }
    }

    pub fn get_flop(&self) -> PlayingCards {
        self.table.board.flop.to_playing_cards()
    }

    pub fn get_stage(&self) -> StageInTheHand {
        self.stage
    }

    pub fn get_turn(&self) -> PlayingCard {
        self.table.board.turn.get()
    }

    pub fn get_river(&self) -> PlayingCard {
        self.table.board.river.get()
    }

    pub fn hand_by_seat(&self, seat: usize) -> Option<PlayingCards> {
        if !self.table.players.is_active(seat) {
            return None;
        }
        let seat = self.table.players.get(seat);
        match seat {
            Some(s) => {
                let mut cards = s.to_playing_cards();
                if self.stage == StageInTheHand::Flop {
                    cards.append(&self.table.board.flop.to_playing_cards());
                } else if self.stage == StageInTheHand::Turn {
                    cards.append(&self.table.board.flop.to_playing_cards());
                    cards.insert(self.get_turn());
                } else if self.stage == StageInTheHand::TheRiver {
                    cards.append(&self.table.board.flop.to_playing_cards());
                    cards.insert(self.get_turn());
                    cards.insert(self.get_river());
                }

                Some(cards)
            }
            None => None,
        }
    }

    pub fn number_of_players(&self) -> usize {
        self.table.players.len()
    }

    pub fn number_of_remaining_cards(&self) -> usize {
        self.deck.len()
    }

    /// Returns the `PokerCards` for the player in the passed in seat number.
    pub fn peak(&self, player: usize) -> Option<PlayingCards> {
        // Before clippy:
        // match self.players.get(player) {
        //     Some(hole_cards) => Some(hole_cards.to_poker_cards()),
        //     None => None,
        // }
        self.table
            .players
            .get(player)
            .map(card_slot::CardSlot::to_playing_cards)
    }

    pub fn turn(&mut self) -> PlayingCard {
        self.burn();
        self.table.board.turn.take(self.deck.draw_one());
        self.stage = StageInTheHand::Turn;
        self.table.board.turn.get()
    }

    pub fn river(&mut self) -> PlayingCard {
        self.burn();
        self.table.board.river.take(self.deck.draw_one());
        self.stage = StageInTheHand::TheRiver;
        self.table.board.river.get()
    }

    /// # Panics
    ///
    /// Will panic if the `StageInTheHand` is set to `TheRiver` even though the deal isn't
    /// at the river.
    pub fn winner(&self) -> (Vec<usize>, Eval) {
        if self.stage != StageInTheHand::TheRiver {
            return (Vec::default(), Eval::default());
        }
        let mut winning_seats: Vec<usize> = Vec::default();
        let mut winner: Eval = Eval::default();

        for c in 0..self.table.players.len() {
            // Make sure they haven't folded.
            if self.table.players.is_active(c) {
                let cards = self.hand_by_seat(c).unwrap();

                let eval = cards.eval_7cards().unwrap();

                match eval.rank.cmp(&winner.rank) {
                    Ordering::Less => {}
                    Ordering::Greater => {
                        winning_seats = vec![c];
                        winner = eval;
                    }
                    Ordering::Equal => {
                        winning_seats.push(c);
                    }
                }
            }
        }
        (winning_seats, winner)
    }
}

impl Default for Deal {
    fn default() -> Deal {
        Deal::new()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_deal_tests {
    use super::*;
    use crate::types::arrays::Vectorable;
    use ckc_rs::PokerCard;

    #[test]
    fn preset() {
        let _index_string = "2S 3D 3S QS QD KH 3C TC 9H 3H 6H 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";
    }

    #[test]
    fn burn() {
        let mut table = Deal::default();

        table.burn();

        assert_eq!(table.burned.len(), 1);
        assert_eq!(table.number_of_remaining_cards(), 51);
    }

    #[test]
    fn deal() {
        let mut deal = Deal::default();

        deal.deal(8);

        assert_eq!(deal.burned.len(), 1);
        assert_eq!(deal.number_of_remaining_cards(), 35);
        assert_eq!(deal.number_of_players(), 8);
        for cards in deal.table.players.iter() {
            assert!(cards.is_dealt());
        }
    }

    #[test]
    fn default() {
        let deal = Deal::default();

        assert_eq!(deal.burned.len(), 0);
        assert_eq!(deal.number_of_players(), 0);
        assert!(!deal.table.board.flop.is_dealt());
        assert_eq!(deal.table.board.turn.get().as_u32(), 0);
        assert_eq!(deal.table.board.river.get().as_u32(), 0);
        assert_eq!(deal.get_stage(), StageInTheHand::ShuffleUp);
    }

    #[test]
    fn flop() {
        let mut deal = Deal::default();

        deal.deal(2);
        let flop = deal.flop();

        assert_eq!(deal.burned.len(), 2);
        assert_eq!(deal.number_of_remaining_cards(), 43);
        for cards in deal.table.players.iter() {
            assert!(cards.is_dealt());
        }
        assert!(!flop.is_empty());
        assert!(deal.table.board.flop.is_dealt());
        assert_eq!(deal.get_stage(), StageInTheHand::Flop);
        assert_eq!(flop, deal.get_flop());
    }

    #[test]
    fn hand_by_seat() {
        let index_string = "2S 3D QS 3S QD KH 3C TC 9H 3H 6H 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";
        let mut table = Deal::from_deck(PlayingCards::try_from(index_string).unwrap()).unwrap();

        table.deal(2);

        assert_eq!(format!("{}", table.hand_by_seat(0).unwrap()), "3♦ 3♠");
        assert_eq!(format!("{}", table.hand_by_seat(1).unwrap()), "Q♠ Q♦");

        table.flop();

        assert_eq!(
            format!("{}", table.hand_by_seat(0).unwrap()),
            "3♦ 3♠ 3♣ T♣ 9♥"
        );
        assert_eq!(
            format!("{}", table.hand_by_seat(1).unwrap()),
            "Q♠ Q♦ 3♣ T♣ 9♥"
        );

        table.turn();

        assert_eq!(
            format!("{}", table.hand_by_seat(0).unwrap()),
            "3♦ 3♠ 3♣ T♣ 9♥ 6♥"
        );
        assert_eq!(
            format!("{}", table.hand_by_seat(1).unwrap()),
            "Q♠ Q♦ 3♣ T♣ 9♥ 6♥"
        );

        table.river();

        assert_eq!(
            format!("{}", table.hand_by_seat(0).unwrap()),
            "3♦ 3♠ 3♣ T♣ 9♥ 6♥ 2♥"
        );
        assert_eq!(
            format!("{}", table.hand_by_seat(1).unwrap()),
            "Q♠ Q♦ 3♣ T♣ 9♥ 6♥ 2♥"
        );

        let (winning_seats, winning_hand) = table.winner();
        assert_eq!(winning_seats, vec![0]);
        assert_eq!(winning_hand.hand.to_string(), "3♠ 3♦ 3♣ T♣ 9♥");
    }

    #[test]
    fn number_of_players() {
        let mut table = Deal::default();

        table.deal(4);

        assert_eq!(table.number_of_players(), 4);
    }

    #[test]
    fn turn() {
        let mut deal = Deal::default();
        deal.deal(2);
        deal.flop();

        let turn = deal.turn();

        assert_eq!(deal.burned.len(), 3);
        assert_eq!(deal.number_of_remaining_cards(), 41);
        assert!(deal.table.board.flop.is_dealt());
        assert!(deal.table.board.turn.is_dealt());
        assert_ne!(turn, PlayingCard::default());
        assert_eq!(deal.get_stage(), StageInTheHand::Turn);
    }

    #[test]
    fn river() {
        let mut deal = Deal::default();
        deal.deal(2);
        deal.flop();
        deal.turn();

        let river = deal.river();

        assert_eq!(deal.burned.len(), 4);
        assert_eq!(deal.number_of_remaining_cards(), 39);
        assert!(deal.table.board.flop.is_dealt());
        assert!(deal.table.board.turn.is_dealt());
        assert!(deal.table.board.river.is_dealt());
        assert_ne!(river, PlayingCard::default());
        assert_eq!(deal.get_stage(), StageInTheHand::TheRiver);
    }

    #[test]
    fn winner__tie() {
        let index_string = "2S 3D 3S QS QD KH 3C 9H 3H 6H 4H 2H 5S 6D 9S 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD TC AD";
        let mut table = Deal::from_deck(PlayingCards::try_from(index_string).unwrap()).unwrap();
        table.deal(2);
        table.flop();
        table.turn();
        table.river();

        let (winning_seats, winning_hand) = table.winner();
        assert_eq!(winning_hand.hand.to_string(), "3♥ 3♦ 3♣ Q♠ 9♥");
        assert_eq!(winning_seats, vec![0, 1]);
    }

    #[test]
    #[ignore]
    fn winner__player_folded() {
        let index_string = "2S 3D 3S QS QD KH 3C 9H 3H 6H 4H 2H 5S 6D 9S 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD TC AD";
        let mut deal = Deal::from_deck(PlayingCards::try_from(index_string).unwrap()).unwrap();
        deal.deal(2);
        deal.flop();
        deal.fold(1);
        deal.turn();
        deal.river();

        let (winning_seats, winning_hand) = deal.winner();
        assert_eq!(
            format!("{}", winning_hand.hand.to_poker_cards()),
            "3♦ Q♠ 3♣ 9♥ 3♥"
        );
        assert_eq!(winning_seats, vec![0]);

        println!("{}", deal.table.board);
    }
}
