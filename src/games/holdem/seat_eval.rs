use crate::analysis::eval::Eval;
use crate::games::holdem::seat::Seat;
use crate::types::arrays::five_card::FiveCard;
use crate::types::slots::flop::Flop;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SeatEval {
    pub seat: Seat,
    pub eval: Eval,
}

impl SeatEval {
    pub fn new(seat: Seat, hand: FiveCard) -> SeatEval {
        SeatEval {
            seat,
            eval: Eval::from(hand),
        }
    }

    pub fn new_from_eval(seat: Seat, eval: Eval) -> SeatEval {
        SeatEval { seat, eval }
    }

    pub fn new_from_flop(seat: Seat, flop: &Flop) -> SeatEval {
        let hand = flop.to_poker_hand_add_hole_cards(&seat.hole_cards);
        SeatEval {
            seat,
            eval: Eval::from(hand.to_arr()),
        }
    }
}

impl fmt::Display for SeatEval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Seat #{}: {} - {}",
            self.seat.number, self.eval.hand, self.eval.rank
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_seat_eval_tests {
    use super::*;
    use crate::types::card_slot::CardSlot;
    use crate::types::playing_card::PlayingCard;

    #[test]
    fn display() {
        let seat = Seat::new(1);
        seat.take(PlayingCard::from(PlayingCard::ACE_SPADES));
        seat.take(PlayingCard::from(PlayingCard::KING_SPADES));
        let hand = Eval::try_from("AS KS QS JS TS").unwrap();

        let seat_eval = SeatEval::new_from_eval(seat, hand);

        assert_eq!("Seat #1: A♠ K♠ Q♠ J♠ T♠ - HandRank { value: 1, name: StraightFlush, class: RoyalFlush }", seat_eval.to_string());
    }

    #[test]
    fn display_sort() {
        let seat = Seat::new(1);
        let hand = Eval::try_from("K♥ K♣ 6♥ 6♦ A♦").unwrap();
        let seat_eval = SeatEval::new_from_eval(seat, hand);

        assert_eq!("Seat #1: K♥ K♣ 6♥ 6♦ A♦ - HandRank { value: 2666, name: TwoPair, class: KingsAndSixes }", seat_eval.to_string());
    }
}
