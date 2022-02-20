use crate::analysis::outs::Outs;
use crate::games::holdem::case_eval::CaseEval;
use crate::games::holdem::seat::Seat;
use crate::games::holdem::seat_eval::SeatEval;
use crate::types::card_slot;
use crate::types::card_slot::CardSlot;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use crate::types::slots::hole_cards::HoleCards;
use ckc_rs::HandError;
use itertools::Itertools;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Seats(Vec<Seat>);

impl Seats {
    #[must_use]
    pub fn seat(number: usize) -> Seats {
        let mut players = Seats::default();
        for _ in 0..number {
            players.add(HoleCards::default());
        }
        players
    }

    /// This alias is needed because, for some reason, the method used in the examples
    /// of turning command line user input into static strings returns mutable static strings,
    /// which is something that I didn't know was possible. For some reason, Rust has no problem
    /// with that input here, but chokes when it comes directly from implementing the `TryFrom` trait.
    ///
    /// Takes a Card index string and returns a `Players` entity with all
    /// the cards dealt to them.
    ///
    /// # Errors
    ///
    /// Will throw a `HandError::InvalidCard` if an invalid index is passed in.
    ///
    /// Will throw a `HandError::InvalidIndex` if the number of cards passed in
    /// isn't divisible by 2. (There must be two cards for each `Player`.)
    pub fn from_index(index: &'static str) -> Result<Seats, HandError> {
        Seats::try_from(index)
    }

    /// A case is defined as a specific possible collection of cards for slots that haven't been
    /// dealt to yet, made up of the remaining cards in the deck. For instance: if the `Flop` is
    /// `T♦ 4♠ 6♠` and player 1 is holding `7♥ 8♥` and player 2 is holding `3♦ A♠`, one possible
    /// case would be `A♦ T♦ 4♠ 6♠ A♥`, made up of the three cards in the `Flop` and two of the
    /// cards that have not been dealt to a player, in this case the `A♦` and the `A♥`.
    ///
    /// The `case_eval` method returns a `CaseEval` entity with the best possible combination of
    /// cards for each player given that specific case. For instance, given the above example,
    /// the best combination for player 1 would be a pair of aces with `A♥ A♦ T♦ 8♥ 7♥`, while
    /// the best combination for player 2 would be three aces with `A♠ A♥ A♦ T♦ 6♠`.
    ///
    /// TODO: This can be tighted up to just accept a `PokerHand`.
    ///
    /// # Panics
    ///
    /// Not sure how this would be triggered :-P
    #[must_use]
    pub fn case_eval(&self, cycle: &PlayingCards) -> CaseEval {
        let mut case_eval = CaseEval::default();
        debug!("Case Eval: {}", cycle);
        for seat in &self.0 {
            let cards = cycle.clone().combine(&seat.to_playing_cards());
            let best_for_player = cards.eval_7cards();
            let eval = best_for_player.unwrap();
            debug!("   Player {} {}", seat.number, eval);
            case_eval.push(SeatEval::new_from_eval(seat.clone(), eval));
        }
        case_eval
    }

    #[must_use]
    pub fn case_eval_with_outs(&self, out: PlayingCard, cycle: &PlayingCards) -> (Outs, CaseEval) {
        let case_eval = self.case_eval(cycle);
        let outs = Seats::get_outs(out, &case_eval);

        (outs, case_eval)
    }

    #[must_use]
    pub fn get_outs(out: PlayingCard, case_eval: &CaseEval) -> Outs {
        let mut outs = Outs::default();
        for seat in case_eval.winners().iter() {
            outs.add(seat.seat.number, out);
        }
        outs
    }

    #[must_use]
    pub fn dealt(&self) -> PlayingCards {
        let mut cards = PlayingCards::default();
        for player in self.iter() {
            cards.append(&player.to_playing_cards());
        }
        cards
    }

    #[must_use]
    pub fn get(&self, player: usize) -> Option<&HoleCards> {
        match self.0.get(player) {
            Some(seat) => Some(&seat.hole_cards),
            None => None,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Seat> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn len_for_player(&self, player: usize) -> Option<usize> {
        let player = self.0.get(player);
        player.map(card_slot::CardSlot::len)
    }

    #[must_use]
    pub fn take_for_player(&self, player: usize, card: PlayingCard) -> bool {
        let player = self.0.get(player);
        match player {
            Some(p) => p.take(card),
            None => false,
        }
    }

    pub fn add(&mut self, hole_cards: HoleCards) {
        self.0
            .push(Seat::new_with_hole_cards(self.len(), hole_cards));
    }

    pub fn add_from_index(&mut self, index: &'static str) {
        self.0.push(Seat::from_index(self.len(), index));
    }

    #[must_use]
    pub fn is_active(&self, seat: usize) -> bool {
        match self.0.get(seat) {
            Some(hand) => hand.is_dealt(),
            None => false,
        }
    }

    /// Returns true if all seated players have had their cards dealt to them.
    #[must_use]
    pub fn is_dealt(&self) -> bool {
        for i in 0..self.0.len() {
            if !self.is_active(i) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn fold_player(&self, player: usize) -> Option<PlayingCards> {
        self.0.get(player).map(card_slot::CardSlot::fold)
    }
}

impl CardSlot for Seats {
    /// Attempts to take a `PokerCard` and place it in the first open slot for the players,
    /// iterating over each Player like a poker dealer would. So, if the first slot of the
    /// first player is filled, and the first slot of the second player is empty, the
    /// second player's slot will take the card first before the second slot of the first player
    /// is taken.
    fn take(&self, card: PlayingCard) -> bool {
        for i in 0..self.len() {
            if let Some(0) = self.len_for_player(i) {
                return self.take_for_player(i, card);
            }
        }
        for i in 0..self.len() {
            if let Some(1) = self.len_for_player(i) {
                return self.take_for_player(i, card);
            }
        }
        false
    }

    /// Folds all the `Players` and returns the `PokerCards` folded.
    fn fold(&self) -> PlayingCards {
        let mut poker_cards = PlayingCards::default();
        for i in 0..self.len() {
            poker_cards.append(&self.fold_player(i).unwrap());
        }
        poker_cards
    }

    /// Returns true of all the `Players` have been dealt their cards.
    fn is_dealt(&self) -> bool {
        for player in self.0.clone() {
            if !player.is_dealt() {
                return false;
            }
        }
        true
    }

    /// Returns the `PokerCards` for all the `Players`.
    fn to_playing_cards(&self) -> PlayingCards {
        let mut cards = PlayingCards::default();
        for player in self.0.clone() {
            cards.append(&player.to_playing_cards());
        }
        cards
    }
}

impl Default for Seats {
    fn default() -> Seats {
        Seats::from(Vec::default())
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = Itertools::join(&mut self.0.iter(), ", ");
        write!(f, "[{}]", joined)
    }
}

impl From<Vec<Seat>> for Seats {
    fn from(value: Vec<Seat>) -> Self {
        Seats(value)
    }
}

/// Takes a Card index string and returns a `Players` entity with all
/// the cards dealt to them.
///
/// # Errors
///
/// Will throw a `HandError::InvalidCard` if an invalid index is passed in.
///
/// Will throw a `HandError::InvalidIndex` if the number of cards passed in
/// isn't divisible by 2. (There must be two cards for each `Player`.)
impl TryFrom<&'static str> for Seats {
    type Error = HandError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let poker_cards = PokerCards::try_from(value);
        match poker_cards {
            Ok(cards) => Seats::try_from(cards),
            Err(e) => Err(e),
        }
    }
}

/// # Errors
///
/// Will throw a `HandError::InvalidIndex` if the number of cards passed in
/// isn't divisible by 2. (There must be two cards for each `Player`.)
impl TryFrom<PokerCards> for Seats {
    type Error = HandError;

    fn try_from(value: PokerCards) -> Result<Self, Self::Error> {
        let mut cards = value;
        if cards.len() % 2 == 0 {
            let num_of_players = cards.len() / 2;
            let players = Seats::seat(num_of_players);
            for i in 0..num_of_players {
                let _ = players.take_for_player(i, PlayingCard::from(cards.pull()));
                let _ = players.take_for_player(i, PlayingCard::from(cards.pull()));
            }
            Ok(players)
        } else {
            Err(HandError::InvalidIndex)
        }
    }
}

impl TryFrom<PlayingCards> for Seats {
    type Error = HandError;

    fn try_from(value: PlayingCards) -> Result<Self, Self::Error> {
        let mut cards = value;
        if cards.len() % 2 == 0 {
            let num_of_players = cards.len() / 2;
            let players = Seats::seat(num_of_players);
            for i in 0..num_of_players {
                let _ = players.take_for_player(i, cards.draw_one());
                let _ = players.take_for_player(i, cards.draw_one());
            }
            Ok(players)
        } else {
            Err(HandError::InvalidIndex)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_players_tests {
    use super::*;

    #[test]
    fn seat() {
        let players = Seats::seat(3);

        assert_eq!(
            "[Seat 0: __ __, Seat 1: __ __, Seat 2: __ __]",
            players.to_string()
        );
    }

    #[test]
    fn from_index() {
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦";

        let players = Seats::try_from(index);

        assert!(players.is_ok());

        assert_eq!(
            "[Seat 0: 4♥ K♣, Seat 1: 5♥ 3♠, Seat 2: 8♦ 9♠, Seat 3: 8♠ J♦]",
            players.unwrap().to_string()
        )
    }

    #[test]
    fn from_index__invalid_card() {
        // Who put a joker in the deck?
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦ 9♥ 9♦ 2♦ 5♠ JK";

        let players = Seats::from_index(index);

        assert!(players.is_err());
        assert_eq!(players.unwrap_err(), HandError::InvalidCard)
    }

    #[test]
    fn from_index__wrong_number_of_cards() {
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦ 9♥ 9♦ 2♦ 5♠ 7♦";

        let players = Seats::from_index(index);

        assert!(players.is_err());
        assert_eq!(players.unwrap_err(), HandError::InvalidIndex)
    }

    #[test]
    fn is_active() {
        let players = Seats::seat(2);
        assert!(!players.is_active(0));
        assert!(!players.is_active(1));
        assert!(!players.is_dealt());

        // deal first player
        let taken = players.take_for_player(0, PlayingCard::from(PlayingCard::ACE_DIAMONDS));
        assert!(taken);
        let taken = players.take_for_player(0, PlayingCard::from(PlayingCard::ACE_CLUBS));
        assert!(taken);
        assert!(players.is_active(0));
        assert!(!players.is_dealt());

        // deal second player
        let taken = players.take_for_player(1, PlayingCard::from(PlayingCard::KING_SPADES));
        assert!(taken);
        let taken = players.take_for_player(1, PlayingCard::from(PlayingCard::KING_CLUBS));
        assert!(taken);
        assert!(players.is_active(1));
        assert!(players.is_dealt());

        assert_eq!("[Seat 0: A♦ A♣, Seat 1: K♠ K♣]", players.to_string());
    }

    #[test]
    fn take() {
        let players = Seats::seat(3);
        let mut cards = PlayingCards::try_from("4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦").unwrap();

        assert!(players.take(cards.draw_one()));
        assert!(players.take(cards.draw_one()));
        assert_eq!(
            "[Seat 0: 4♥ __, Seat 1: K♣ __, Seat 2: __ __]",
            players.to_string()
        );

        assert!(players.take(cards.draw_one()));
        assert!(players.take(cards.draw_one()));
        assert_eq!(
            "[Seat 0: 4♥ 3♠, Seat 1: K♣ __, Seat 2: 5♥ __]",
            players.to_string()
        );

        assert!(players.take(cards.draw_one()));
        assert!(players.take(cards.draw_one()));
        assert_eq!(
            "[Seat 0: 4♥ 3♠, Seat 1: K♣ 8♦, Seat 2: 5♥ 9♠]",
            players.to_string()
        );

        assert!(!players.take(cards.draw_one()));
        assert_eq!(
            "[Seat 0: 4♥ 3♠, Seat 1: K♣ 8♦, Seat 2: 5♥ 9♠]",
            players.to_string()
        );
    }

    #[test]
    fn fold() {
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦";
        let players = Seats::from_index(index).unwrap();

        let folded = players.fold();

        assert_eq!("4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦", folded.to_string());
        assert_eq!(
            "[Seat 0: __ __, Seat 1: __ __, Seat 2: __ __, Seat 3: __ __]",
            players.to_string()
        );
        assert!(!players.is_dealt());
    }

    #[test]
    fn to_poker_cards() {
        let index = "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦";
        let players = Seats::from_index(index).unwrap();

        assert_eq!(
            "4♥ K♣ 5♥ 3♠ 8♦ 9♠ 8♠ J♦",
            players.to_playing_cards().to_string()
        );
        assert!(players.is_dealt());
    }

    #[test]
    fn display() {
        let players = Seats::from_index("AS KS AD KD").unwrap();

        assert_eq!("[Seat 0: A♠ K♠, Seat 1: A♦ K♦]", players.to_string());
    }

    #[test]
    fn display__blank() {
        assert_eq!(
            "[Seat 0: __ __, Seat 1: __ __, Seat 2: __ __]",
            format!("{}", Seats::seat(3))
        )
    }
}
