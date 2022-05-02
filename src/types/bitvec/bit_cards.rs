use crate::types::arrays::Vectorable;
use crate::types::bitvec::bit_card::BitCard;
use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use crate::types::poker_cards::PokerCards;
use crate::types::U32Card;
use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Msb0};
use ckc_rs::HandError;
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct BitCards(Vec<BitCard>);

impl BitCards {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn rank_count(&self) -> u8 {
        self.or_rank_bit_slice().count_ones() as u8
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn suit_count(&self) -> u8 {
        self.or_suit_bit_slice().count_ones() as u8
    }

    #[must_use]
    pub fn get(&self, i: usize) -> Option<&BitCard> {
        self.0.get(i)
    }

    #[must_use]
    pub fn is_complete_hand(&self) -> bool {
        self.len() == 5
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn is_flush(&self) -> bool {
        (self.suit_count() == 1) && self.is_complete_hand()
    }

    #[must_use]
    pub fn is_straight(&self) -> bool {
        let v = self.or_rank_bit_slice();
        ((v.leading_zeros() + v.trailing_zeros()) == 11) && self.is_complete_hand()
    }

    #[must_use]
    pub fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    pub fn iter(&self) -> impl Iterator<Item = &BitCard> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn and(&self) -> BitVec<Msb0, u8> {
        let mut v = BitVec::new();
        for bit_card in self.iter() {
            v = bit_card.and(&v);
        }
        v
    }

    #[must_use]
    pub fn and_to_usize(&self) -> usize {
        self.and().as_bitslice().load_be::<usize>()
    }

    #[must_use]
    pub fn or(&self) -> BitVec<Msb0, u8> {
        let mut v = BitVec::new();
        for bit_card in self.iter() {
            v = bit_card.or(&v);
        }
        v
    }

    #[must_use]
    pub fn or_to_usize(&self) -> usize {
        self.or().as_bitslice().load_be::<usize>()
    }

    #[must_use]
    pub fn or_rank_bit_slice(&self) -> BitVec<Msb0, u8> {
        let mut v = BitVec::new();
        for bit_card in self.iter() {
            v = bit_card.or_rank_bitslice(&v);
        }
        v
    }

    #[must_use]
    pub fn and_suit_bitslice(&self) -> BitVec<Msb0, u8> {
        let mut v = BitVec::new();
        for bit_card in self.iter() {
            v = bit_card.and_suit_bitslice(&v);
        }
        v
    }

    #[must_use]
    pub fn or_suit_bit_slice(&self) -> BitVec<Msb0, u8> {
        let mut v = BitVec::new();
        for bit_card in self.iter() {
            v = bit_card.or_suit_bitslice(&v);
        }
        v
    }

    pub fn push(&mut self, bit_card: BitCard) {
        self.0.push(bit_card);
    }

    #[must_use]
    pub fn to_poker_cards(&self) -> PokerCards {
        let v: Vec<U32Card> = self.0.iter().map(BitCard::to_poker_card).collect();
        PokerCards::from(v)
    }
}

impl Display for BitCards {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        for bit_card in self.0.clone() {
            let mut mark_string = String::with_capacity(35);
            mark_string.push_str("xxxAKQJT 98765432 SHDCrrrr xxpppppp");

            out.entry(&(bit_card.display(true)).fmt_display());
            out.entry(&(&mark_string).fmt_display());
        }

        out.finish()
    }
}

impl From<Vec<BitCard>> for BitCards {
    fn from(value: Vec<BitCard>) -> Self {
        BitCards(value)
    }
}

impl From<Vec<U32Card>> for BitCards {
    fn from(vec: Vec<U32Card>) -> Self {
        BitCards(vec.into_iter().map(BitCard::from).collect())
    }
}

impl From<PokerCards> for BitCards {
    fn from(poker_cards: PokerCards) -> Self {
        BitCards::from(poker_cards.to_vec())
    }
}

impl From<Vec<PlayingCard>> for BitCards {
    fn from(vec: Vec<PlayingCard>) -> Self {
        BitCards(vec.into_iter().map(BitCard::from).collect())
    }
}

impl From<PlayingCards> for BitCards {
    fn from(playing_cards: PlayingCards) -> Self {
        BitCards::from(playing_cards.to_vec())
    }
}

impl FromIterator<BitCard> for BitCards {
    fn from_iter<T: IntoIterator<Item = BitCard>>(iter: T) -> Self {
        let mut c = BitCards::default();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl IntoIterator for BitCards {
    type Item = BitCard;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<&'static str> for BitCards {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        let pile = cardpack::Standard52::pile_from_index(index);

        if pile.is_err() {
            return Err(HandError::InvalidIndex);
        }

        let mut cards = BitCards::default();
        for card in pile.unwrap() {
            cards.push(BitCard::from(&card));
        }
        Ok(cards)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_cards_tests {
    use super::*;
    use crate::types::U32Card;
    use ckc_rs::CardNumber;

    #[test]
    fn to_cactus_kev_cards() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();
        let ckc = cards.to_poker_cards();

        assert_eq!(*ckc.get(0).unwrap(), cards.get(0).unwrap().to_poker_card());
        assert_eq!(*ckc.get(4).unwrap(), cards.get(4).unwrap().to_poker_card());
        assert_eq!(ckc.get(5), None);
    }

    #[test]
    fn get() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        assert_eq!(cards.len(), 5);
        let c = cards.get(1).unwrap();
        assert_eq!(c, &BitCard::try_from("KS").unwrap());
    }

    #[test]
    fn is_empty() {
        assert!(BitCards::default().is_empty());
    }

    #[test]
    fn is_flush() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        assert!(cards.is_flush());
    }

    #[test]
    fn is_straight() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        assert!(cards.is_straight());
    }

    #[test]
    fn is_straight__false() {
        let cards = BitCards::try_from("AS KS QS JS 9S").unwrap();

        assert!(!cards.is_straight());
    }

    #[test]
    fn is_straight__incomplete() {
        let cards = BitCards::try_from("AS KS QS TS").unwrap();

        assert!(!cards.is_straight());
    }

    #[test]
    fn is_straight_flush() {
        let cards = BitCards::try_from("KS QS JS TS 9S").unwrap();

        assert!(cards.is_straight_flush());
    }

    #[test]
    fn is_straight_flush__false() {
        let cards = BitCards::try_from("AS KS QS JS TC").unwrap();

        assert!(!cards.is_straight_flush());
    }

    #[test]
    fn len() {
        let mut cards = BitCards::default();
        assert_eq!(0, cards.len());

        cards.push(BitCard::try_from("AS").unwrap());
        assert_eq!(1, cards.len());
    }

    #[test]
    fn or_rank_bit_slice() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        assert_eq!(
            "[00011111, 00000000]",
            format!("{}", cards.or_rank_bit_slice())
        );
    }

    #[test]
    fn or_suit_bit_slice() {
        let cards = BitCards::try_from("AS KC QH JD TS").unwrap();

        assert_eq!("[1111]", format!("{:04b}", cards.or_suit_bit_slice()));
    }

    #[test]
    fn push() {
        let mut cards = BitCards::default();
        cards.push(BitCard::try_from("AS").unwrap());
        cards.push(BitCard::try_from("KS").unwrap());
        let expected = "[00010000 00000000 10001100 00101001, xxxAKQJT 98765432 SHDCrrrr xxpppppp, 00001000 00000000 10001011 00100101, xxxAKQJT 98765432 SHDCrrrr xxpppppp]";

        // println!("{:#}", cards);
        assert_eq!(format!("{}", cards), expected);
    }

    fn shift_16(c1: &U32Card, c2: &U32Card, c3: &U32Card, c4: &U32Card, c5: &U32Card) -> usize {
        ((c1 | c2 | c3 | c4 | c5) as usize) >> 16
    }

    fn flush_hunt(c1: &U32Card, c2: &U32Card, c3: &U32Card, c4: &U32Card, c5: &U32Card) -> bool {
        (c1 & c2 & c3 & c4 & c5 & CardNumber::SUIT_FILTER) != 0
    }

    #[test]
    #[ignore]
    fn hand_rank() {
        let cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        let c1: &U32Card = &cards.get(0).unwrap().to_poker_card();
        let c2: &U32Card = &cards.get(1).unwrap().to_poker_card();
        let c3: &U32Card = &cards.get(2).unwrap().to_poker_card();
        let c4: &U32Card = &cards.get(3).unwrap().to_poker_card();
        let c5: &U32Card = &cards.get(4).unwrap().to_poker_card();

        let q = shift_16(c1, c2, c3, c4, c5);
        let q2 = cards.or_to_usize() >> 16;

        println!("q = {} {}", q, q2);
        // 00000000 00000000 11110000 00000000
        println!("SUITS_FILTER = {}", CardNumber::SUIT_FILTER);

        let f = flush_hunt(c1, c2, c3, c4, c5);
        println!("f = {}", f);
    }

    #[test]
    #[ignore]
    fn scratch() {
        let _cards = BitCards::try_from("AS KS QS JS TS").unwrap();

        // cards.into_iter().map()

        let pile = cardpack::Standard52::pile_from_index("AS KS QS JS TS")
            .unwrap()
            .sort();
        let ck_ace_spades: BitCard = BitCard::from(pile.get(0).unwrap());
        let ck_king_spades: BitCard = BitCard::from(pile.get(1).unwrap());
        let ck_queen_spades: BitCard = BitCard::from(pile.get(2).unwrap());
        let ck_jack_spades: BitCard = BitCard::from(pile.get(3).unwrap());
        let ck_ten_spades: BitCard = BitCard::from(pile.get(4).unwrap());
        // let s = ck_king_spades.bites.to_bitvec().sum()

        let sum = ck_ace_spades.get_rank_bitslice().to_bitvec()
            | ck_king_spades.get_rank_bitslice().to_bitvec()
            | ck_queen_spades.get_rank_bitslice().to_bitvec()
            | ck_jack_spades.get_rank_bitslice().to_bitvec()
            | ck_ten_spades.get_rank_bitslice().to_bitvec();

        println!("{}", sum);

        println!("{}", sum.leading_zeros());
        println!("{}", sum.trailing_zeros());
    }
}
