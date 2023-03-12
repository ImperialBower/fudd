use crate::types::arrays::five_card::FiveCard;
use crate::types::U32Card;
use ckc_rs::hand_rank::HandRankName;
use ckc_rs::{CardRank, CardSuit, PokerCard};

pub struct UCI;

impl UCI {
    #[allow(clippy::missing_panics_doc, clippy::needless_pass_by_value)]
    #[must_use]
    pub fn parse_line(s: String) -> Option<(FiveCard, HandRankName)> {
        let i: Vec<&str> = s.split(',').collect();
        if i.len() < 11 {
            return None;
        }
        let first = U32Card::create(
            UCI::get_rank(i.get(1).unwrap().parse::<usize>().unwrap()),
            UCI::get_suit(i.first().unwrap().parse::<usize>().unwrap()),
        );
        let second = U32Card::create(
            UCI::get_rank(i.get(3).unwrap().parse::<usize>().unwrap()),
            UCI::get_suit(i.get(2).unwrap().parse::<usize>().unwrap()),
        );
        let third = U32Card::create(
            UCI::get_rank(i.get(5).unwrap().parse::<usize>().unwrap()),
            UCI::get_suit(i.get(4).unwrap().parse::<usize>().unwrap()),
        );
        let forth = U32Card::create(
            UCI::get_rank(i.get(7).unwrap().parse::<usize>().unwrap()),
            UCI::get_suit(i.get(6).unwrap().parse::<usize>().unwrap()),
        );
        let fifth = U32Card::create(
            UCI::get_rank(i.get(9).unwrap().parse::<usize>().unwrap()),
            UCI::get_suit(i.get(8).unwrap().parse::<usize>().unwrap()),
        );
        let five = FiveCard::from([first, second, third, forth, fifth]);
        Some((
            five,
            UCI::get_hand_rank_name(i.get(10).unwrap().parse::<usize>().unwrap()),
        ))
    }

    #[must_use]
    pub fn get_hand_rank_name(number: usize) -> HandRankName {
        match number {
            0 => HandRankName::HighCard,
            1 => HandRankName::Pair,
            2 => HandRankName::TwoPair,
            3 => HandRankName::ThreeOfAKind,
            4 => HandRankName::Straight,
            5 => HandRankName::Flush,
            6 => HandRankName::FullHouse,
            7 => HandRankName::FourOfAKind,
            _ => HandRankName::StraightFlush,
        }
    }

    // Ordinal (1-4) representing {Hearts, Spades, Diamonds, Clubs}
    #[must_use]
    pub fn get_suit(number: usize) -> CardSuit {
        match number {
            1 => CardSuit::HEARTS,
            2 => CardSuit::SPADES,
            3 => CardSuit::DIAMONDS,
            4 => CardSuit::CLUBS,
            _ => CardSuit::BLANK,
        }
    }

    // Numerical (1-13) representing (Ace, 2, 3, ... , Queen, King)
    #[must_use]
    pub fn get_rank(number: usize) -> CardRank {
        match number {
            1 => CardRank::ACE,
            2 => CardRank::TWO,
            3 => CardRank::THREE,
            4 => CardRank::FOUR,
            5 => CardRank::FIVE,
            6 => CardRank::SIX,
            7 => CardRank::SEVEN,
            8 => CardRank::EIGHT,
            9 => CardRank::NINE,
            10 => CardRank::TEN,
            11 => CardRank::JACK,
            12 => CardRank::QUEEN,
            13 => CardRank::KING,
            _ => CardRank::BLANK,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod util_uci_tests {
    use super::*;
    use ckc_rs::hand_rank::HandRankName::StraightFlush;
    use ckc_rs::CardNumber;

    #[test]
    fn create() {
        let card = U32Card::create(UCI::get_rank(10), UCI::get_suit(1));

        assert_eq!(card, CardNumber::TEN_HEARTS);
    }

    #[test]
    fn parse() {
        let s = "1,10,1,11,1,13,1,12,1,1,9".to_string();
        let (hand, class) = UCI::parse_line(s).unwrap();

        // T♥ J♥ K♥ Q♥ A♥ StraightFlush
        assert_eq!(hand.to_string(), "T♥ J♥ K♥ Q♥ A♥");
        assert_eq!(class, StraightFlush);
    }
}
