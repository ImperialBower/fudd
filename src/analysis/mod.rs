pub mod chances;
pub mod count;
pub mod eval;
pub mod eval_7card;
pub mod evals;
pub mod evals_7card;
pub mod holdem_playout;
pub mod indexed;
pub mod outs;
pub mod preflop;
pub mod seat_calc;

use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::Evaluable;
use crate::types::poker_deck::POKER_DECK;
use ckc_rs::hand_rank::{HandRank, HandRankClass, HandRankName};
use log::debug;
use std::collections::HashMap;

pub struct Evaluate;

impl Evaluate {
    pub const POSSIBLE_COMBINATIONS: usize = 7937;

    /// Brute force iteration over every possible combination of 5 cards in the standard 52 card
    /// French Deck used in most forms of poker.
    ///
    /// Verifies the breakdown of [Cactus Kev's Hand Rank](https://suffe.cool/poker/evaluator.html)
    /// system of evaluating poker hands.
    ///
    /// Returns a tuple compromising a `HashMap` of every `HandRankClass` and its count,
    /// a `HashMap` of every `HandRankName` and the count of every `HandRank`, and finally
    /// a `HashMap` of every `HandRank` and a boolean flag representing if it was counted.
    ///
    /// You can see an example of how this method can be used to demonstrate the values
    /// of the hand ranking system in the examples folder.
    ///
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn all_possible_combos() -> (
        HashMap<HandRankClass, usize>,
        HashMap<HandRankName, usize>,
        HashMap<HandRank, bool>,
    ) {
        let mut rank_class_count: HashMap<HandRankClass, usize> = HashMap::new();
        let mut rank_name_count: HashMap<HandRankName, usize> = HashMap::new();
        let mut rank_count: HashMap<HandRank, bool> = HashMap::new();
        for v in POKER_DECK.combinations(5) {
            let (hand, hand_rank) = FiveCard::try_from(v).unwrap().evaluate();
            debug!("{} {}", hand, hand_rank);
            rank_count.entry(hand_rank).or_insert(true);
            let class_count = rank_class_count.entry(hand_rank.class).or_insert(0);
            *class_count += 1;
        }

        for key in rank_count.keys() {
            let rank_name = key.name;

            let name_count = rank_name_count.entry(rank_name).or_insert(0);
            *name_count += 1;
        }

        (rank_class_count, rank_name_count, rank_count)
    }

    /// Takes in a `HashMap` counting unique `HandRankClass` instances and returns
    /// the total number of unique hands.
    #[must_use]
    pub fn count_possible_hands(hands: &HashMap<HandRankClass, usize>) -> usize {
        hands.values().copied().collect::<Vec<usize>>().iter().sum()
    }

    /// Call to core hand analysis library. This is the heart of the system.
    #[must_use]
    pub fn five_cards(five_cards: FiveCard) -> HandRank {
        HandRank::from(ckc_rs::evaluate::five_cards(five_cards.to_arr()))
    }

    /// Forgiving percentage calculator. It will return zero if you try
    /// to divide by zero.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn percent(number: usize, total: usize) -> f32 {
        match total {
            0 => 0_f32,
            _ => ((number as f32 * 100.0) / total as f32) as f32,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod evaluate_tests {
    use super::*;
    use crate::analysis::eval::Eval;
    use crate::types::arrays::five_card::FiveCard;
    use ckc_rs::hand_rank::HandRankValue;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[ignore]
    fn eval_on_all_possible_combinations() {
        init();

        let (classes, names, ranks) = Evaluate::all_possible_combos();

        // There should be 7462 unique HandRanks.
        assert_eq!(ranks.len(), 7462);
        // There should be 308 different Hand Rank Classes.
        assert_eq!(classes.len(), 309);

        // Verifies the numbers from
        // [Cactus Kev's Hand Rank breakdown](https://suffe.cool/poker/evaluator.html).
        assert_eq!(*names.get(&HandRankName::HighCard).unwrap(), 1277);
        assert_eq!(*names.get(&HandRankName::Pair).unwrap(), 2860);
        assert_eq!(*names.get(&HandRankName::TwoPair).unwrap(), 858);
        assert_eq!(*names.get(&HandRankName::ThreeOfAKind).unwrap(), 858);
        assert_eq!(*names.get(&HandRankName::Straight).unwrap(), 10);
        assert_eq!(*names.get(&HandRankName::Flush).unwrap(), 1277);
        assert_eq!(*names.get(&HandRankName::FullHouse).unwrap(), 156);
        assert_eq!(*names.get(&HandRankName::FourOfAKind).unwrap(), 156);
        assert_eq!(*names.get(&HandRankName::StraightFlush).unwrap(), 10);

        let possible_hands = Evaluate::count_possible_hands(&classes);
        assert_eq!(possible_hands, 2_598_960);

        // 4 of 2,598,960 possible hands dealt (0.00015%) will be a Royal Flush.
        assert_eq!(*classes.get(&HandRankClass::RoyalFlush).unwrap(), 4);
        // 502,860 of 2,598,960 possible hands dealt (19.34851%) will be an Ace High.
        assert_eq!(*classes.get(&HandRankClass::AceHigh).unwrap(), 502860);

        // Verify 100% of counted
        let mut total_percentage = 0.0;
        for v in HandRankClass::iter() {
            let c = classes.get(&v);
            match c {
                Some(class) => {
                    let pec = ((*class as f32 * 100.0) / possible_hands as f32) as f32;
                    total_percentage += pec;
                }
                None => (),
            }
        }
        assert_eq!(format!("{:.2}", total_percentage), "100.00");
    }

    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRankName::StraightFlush)]
    #[case("A♣ 2♣ 3♣ 4♣ 5♣", 10, HandRankName::StraightFlush)]
    #[case("A♠ A♥ A♦ A♣ K♠", 11, HandRankName::FourOfAKind)]
    #[case("2♠ 2♥ 2♦ 2♣ 3♠", 166, HandRankName::FourOfAKind)]
    #[case("A♠ A♥ A♦ K♠ K♦", 167, HandRankName::FullHouse)]
    #[case("2♠ 2♥ 2♦ 3♠ 3♦", 322, HandRankName::FullHouse)]
    #[case("A♠ K♠ Q♠ J♠ 9♠", 323, HandRankName::Flush)]
    #[case("2♣ 3♣ 4♣ 5♣ 7♣", 1599, HandRankName::Flush)]
    #[case("A♣ K♠ Q♠ J♠ T♠", 1600, HandRankName::Straight)]
    #[case("A♥ 2♣ 3♣ 4♣ 5♣", 1609, HandRankName::Straight)]
    #[case("A♠ A♥ A♦ K♠ Q♣", 1610, HandRankName::ThreeOfAKind)]
    #[case("2♠ 2♥ 2♦ 3♠ 4♣", 2467, HandRankName::ThreeOfAKind)]
    #[case("A♠ A♥ K♦ K♠ Q♣", 2468, HandRankName::TwoPair)]
    #[case("3♠ 3♥ 2♦ 2♠ 4♣", 3325, HandRankName::TwoPair)]
    #[case("A♠ A♥ K♠ Q♠ J♠", 3326, HandRankName::Pair)]
    #[case("2♠ 2♥ 3♠ 4♠ 5♠", 6185, HandRankName::Pair)]
    #[case("A♠ K♠ Q♠ J♠ 9♣", 6186, HandRankName::HighCard)]
    #[case("2♣ 3♣ 4♣ 5♥ 7♣", 7462, HandRankName::HighCard)]
    #[case("2♣ 3♦ 4♣ 5♥ 7♣", 7462, HandRankName::HighCard)]
    fn evaluate(
        #[case] index: &'static str,
        #[case] hand_rank_value: HandRankValue,
        #[case] hand_rank_name: HandRankName,
    ) {
        let hand = FiveCard::try_from(index).unwrap();

        let actual_hand_rank = Evaluate::five_cards(hand);

        assert_eq!(hand_rank_value, actual_hand_rank.value);
        assert_eq!(hand_rank_name, actual_hand_rank.name);
        assert_eq!(HandRank::from(hand_rank_value), actual_hand_rank);
    }

    #[test]
    fn percent() {
        let percentage = Evaluate::percent(48, 2_598_960);

        assert_eq!("0.00185%", format!("{:.5}%", percentage));
    }

    #[test]
    fn percent__zero_numerator() {
        let percentage = Evaluate::percent(0, 2_598_960);

        assert_eq!("0.00000%", format!("{:.5}%", percentage));
    }

    #[test]
    fn percent__zero_denominator() {
        let percentage = Evaluate::percent(48, 0);

        assert_eq!("0.00000%", format!("{:.5}%", percentage));
    }

    #[test]
    fn display() {
        let eval = Eval::try_from("AS KS QS JS TS").unwrap();
        assert_eq!(
            "A♠ K♠ Q♠ J♠ T♠ HandRank { value: 1, name: StraightFlush, class: RoyalFlush }",
            eval.to_string()
        );
    }

    #[test]
    fn display__invalid() {
        let eval = Eval::try_from("AS AS QS JS TS").unwrap();
        assert_eq!(
            "HandRank { value: 0, name: Invalid, class: Invalid }",
            eval.rank.to_string()
        );
    }
}
