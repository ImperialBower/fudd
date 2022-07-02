use crate::types::playing_card::PlayingCard;
use bitvec::field::BitField;
use bitvec::prelude::{BitArray, BitSlice, BitVec, Msb0};
use cardpack::Card;
use ckc_rs::{CKCNumber, HandError, PokerCard};
use std::fmt;
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

/// `BitCard` is an experiment with using the
/// [Alexander Payne](https://myrrlyn.net/)'s wonderful
/// [bitvec](https://github.com/bitvecto-rs/bitvec) library to represent
/// [Cactus Kev's](https://suffe.cool/poker/evaluator.html) binary representation
/// of a Poker card.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitCard(BitArray<Msb0, [u8; 4]>);

impl BitCard {
    // Constructors
    #[must_use]
    pub fn new(b: BitArray<Msb0, [u8; 4]>) -> BitCard {
        BitCard(b)
    }

    // Struct methods

    /// Takes the `BitArray` representation of the Card and returns a `String`
    /// representation of the bits. If split is set to true, it will put a space
    /// between each bite. For instance, `00001000000000000100101100100101`
    /// becomes `00001000 00000000 01001011 00100101`.
    #[must_use]
    pub fn display(&self, split: bool) -> String {
        let mut word_string = String::with_capacity(35);
        let start_bit: usize = 0;
        let bits = start_bit..start_bit + 32;

        for (bit, idx) in self.0.as_bitslice().iter().by_val().zip(bits) {
            word_string.push_str(if bit { "1" } else { "0" });
            if split && idx % 8 == 7 && idx % 32 != 31 {
                word_string.push(' ');
            }
        }
        word_string
    }

    #[must_use]
    pub fn as_bitarray(&self) -> BitArray<Msb0, [u8; 4]> {
        self.0
    }

    #[must_use]
    pub fn as_bitslice(&self) -> &BitSlice<Msb0, u8> {
        self.0.as_bitslice()
    }

    #[must_use]
    pub fn get_rank(&self) -> cardpack::Rank {
        match self.get_rank_bitslice().trailing_zeros() {
            12 => cardpack::Rank::new(cardpack::ACE),
            11 => cardpack::Rank::new(cardpack::KING),
            10 => cardpack::Rank::new(cardpack::QUEEN),
            9 => cardpack::Rank::new(cardpack::JACK),
            8 => cardpack::Rank::new(cardpack::TEN),
            7 => cardpack::Rank::new(cardpack::NINE),
            6 => cardpack::Rank::new(cardpack::EIGHT),
            5 => cardpack::Rank::new(cardpack::SEVEN),
            4 => cardpack::Rank::new(cardpack::SIX),
            3 => cardpack::Rank::new(cardpack::FIVE),
            2 => cardpack::Rank::new(cardpack::FOUR),
            1 => cardpack::Rank::new(cardpack::THREE),
            0 => cardpack::Rank::new(cardpack::TWO),
            _ => cardpack::Rank::default(),
        }
    }

    #[must_use]
    pub fn get_rank_bitslice(&self) -> &BitSlice<Msb0, u8> {
        &self.0[..16]
    }

    #[must_use]
    pub fn get_suit(&self) -> cardpack::Suit {
        match self.get_suit_bitslice().load_le::<u8>() {
            8 => cardpack::Suit::new(cardpack::SPADES),
            4 => cardpack::Suit::new(cardpack::HEARTS),
            2 => cardpack::Suit::new(cardpack::DIAMONDS),
            1 => cardpack::Suit::new(cardpack::CLUBS),
            _ => cardpack::Suit::default(),
        }
    }

    /// Returns a `BitSlice` of the `Suit` section of the `CactusKev` `BitArray`.
    #[must_use]
    pub fn get_suit_bitslice(&self) -> &BitSlice<Msb0, u8> {
        &self.0[16..20]
    }

    #[must_use]
    pub fn get_suit_binary_signature(&self) -> u32 {
        let s = self.get_suit_bitslice().load_be::<u32>();
        s << 12
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.0.count_zeros() == 32
    }

    #[must_use]
    pub fn and(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.as_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn or(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.as_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn or_rank_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_rank_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn and_suit_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_suit_bitslice().to_bitvec() & bc.to_bitvec()
    }

    #[must_use]
    pub fn or_suit_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_suit_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn to_bitvec(&self) -> BitVec<Msb0, u8> {
        self.0.to_bitvec()
    }

    /// Returns a `cardpack::Card`.
    #[must_use]
    pub fn to_card(&self) -> cardpack::Card {
        if self.is_blank() {
            return cardpack::Card::default();
        }
        cardpack::Card::new(self.get_rank(), self.get_suit())
    }

    #[must_use]
    pub fn to_poker_card(&self) -> CKCNumber {
        self.as_bitslice().load_be::<u32>()
    }

    // Private methods

    fn set_rank(&mut self, card: &cardpack::Card) {
        self.0[20..24].store_be(card.rank.weight);
    }

    fn set_rank_flag(&mut self, card: &cardpack::Card) {
        match card.rank.weight {
            12 => self.0.set(3, true), // Ace
            11 => self.0.set(4, true), // King
            10 => self.0.set(5, true), // Queen
            9 => self.0.set(6, true),  // Jack
            8 => self.0.set(7, true),  // Ten
            7 => self.0.set(8, true),  // Nine
            6 => self.0.set(9, true),  // Eight
            5 => self.0.set(10, true), // Seven
            4 => self.0.set(11, true), // Six
            3 => self.0.set(12, true), // Five
            2 => self.0.set(13, true), // Four
            1 => self.0.set(14, true), // Three
            0 => self.0.set(15, true), // Two
            _ => (),
        }
    }

    fn set_rank_prime(&mut self, card: &cardpack::Card) {
        self.0[26..32].store_be(card.rank.prime);
    }

    fn set_suit(&mut self, card: &cardpack::Card) {
        match card.suit.weight {
            4 => self.0.set(16, true), // Spades
            3 => self.0.set(17, true), // Hearts
            2 => self.0.set(18, true), // Diamonds
            1 => self.0.set(19, true), // Clubs
            _ => (),
        }
    }
}

impl Default for BitCard {
    fn default() -> BitCard {
        BitCard::new(BitArray::zeroed())
    }
}

/// ```txt
/// +--------+--------+--------+--------+
/// |xxxbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// SHDC = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// ```
impl Display for BitCard {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.display(true))
    }
}

/// **Usage:**
/// ```
/// use std::convert::TryFrom;
/// use fudd::types::bitvec::bit_card::BitCard;
///
/// let actual = BitCard::try_from("K♦").unwrap();
/// println!("{:032b}", actual);
/// ```
/// Prints out: `00001000000000000100101100100101`.
///
impl fmt::Binary for BitCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_poker_card(), f)
    }
}

impl From<&cardpack::Card> for BitCard {
    fn from(value: &Card) -> Self {
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_rank(value);
        bit_card.set_rank_flag(value);
        bit_card.set_rank_prime(value);
        bit_card.set_suit(value);
        bit_card
    }
}

impl From<PlayingCard> for BitCard {
    fn from(playing_card: PlayingCard) -> Self {
        BitCard::from(playing_card.as_u32())
    }
}

impl From<CKCNumber> for BitCard {
    fn from(number: CKCNumber) -> Self {
        let mut bc: BitCard = BitCard::default();
        if number == 0_u32 {
            return bc;
        }
        bc.0[..32].store_be(number);
        bc
    }
}

impl TryFrom<&'static str> for BitCard {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `HandError::InvalidCard` for an invalid index.
    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        let c = cardpack::Standard52::card_from_index(index);

        if c.is_valid() {
            Ok(BitCard::from(&c))
        } else {
            Err(HandError::InvalidCard)
        }
    }
}

/// Usage:
///
/// ```
/// use std::convert::TryFrom;
/// use fudd::types::bitvec::bit_card::{AnnotatedBitCard, BitCard};
///
/// let king_spades: BitCard = BitCard::try_from("KS").unwrap();
/// println!("{:#}", AnnotatedBitCard::new(king_spades));
///
/// // prints out:
/// // [
/// //     00001000 00000000 10001011 00100101,
/// //     xxxAKQJT 98765432 SHDCrrrr xxpppppp,
/// // ]
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct AnnotatedBitCard(BitCard);

impl AnnotatedBitCard {
    #[must_use]
    pub fn new(bit_card: BitCard) -> AnnotatedBitCard {
        AnnotatedBitCard(bit_card)
    }
}

/// [Module ``std::fmt``](https://doc.rust-lang.org/std/fmt/)
/// ```txt
/// +--------+--------+--------+--------+
/// |xxxbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// SHDC = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// ```
impl Display for AnnotatedBitCard {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        let mut mark_string = String::with_capacity(35);
        mark_string.push_str("xxxAKQJT 98765432 SHDCrrrr xxpppppp");

        out.entry(&(self.0.display(true)).fmt_display());
        out.entry(&(&mark_string).fmt_display());
        out.finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_card_tests {
    use super::*;
    use crate::types::playing_card::PlayingCard;
    use rstest::rstest;

    #[test]
    fn len() {
        assert_eq!(BitCard::default().0.len(), 32);
    }

    #[test]
    fn from__card() {
        let card = cardpack::Standard52::card_from_index("K♦");
        let cactusKevCard: BitCard = BitCard::from(&card);

        assert_eq!(
            "00001000 00000000 00101011 00100101",
            cactusKevCard.display(true)
        );
    }

    /// This test goes through all 52 cards in a Standard52 deck and compares the
    /// `CactusKevCard` version of the bite signature with the `Card`'s version.
    #[test]
    fn from__card__complete() {
        let standard52 = cardpack::Standard52::default();
        for card in standard52.deck {
            let cactusKevCard: BitCard = BitCard::from(&card);
            let s = format!("{:032b}", cactusKevCard).to_string();
            assert_eq!(s, cactusKevCard.display(false));
        }
    }

    #[test]
    fn from_index() {
        let card = cardpack::Standard52::card_from_index("KS");
        let expected = BitCard::from(&card);

        let actual = BitCard::try_from("KS").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_index__invalid() {
        assert!(BitCard::try_from("xx").is_err());
    }

    #[test]
    fn from_poker_card() {
        let ace_spades: u32 = 268_471_337;
        let s = "00010000 00000000 10001100 00101001".to_string();
        let actual = BitCard::from(ace_spades);

        assert_eq!(actual.display(true), s);
        assert_eq!(actual, BitCard::try_from("A♤").unwrap());
    }

    #[rstest]
    #[case("A♠", 268_471_337)]
    #[case("K♠", 134_253_349)]
    #[case("Q♠", 67_144_223)]
    #[case("J♠", 33_589_533)]
    #[case("T♠", 16_812_055)]
    #[case("9♠", 8_423_187)]
    #[case("8♠", 4_228_625)]
    #[case("7♠", 2_131_213)]
    #[case("6♠", 1_082_379)]
    #[case("5♠", 557_831)]
    #[case("4♠", 295_429)]
    #[case("3♠", 164_099)]
    #[case("2♠", 98_306)]
    #[case("A♥", 268_454_953)]
    #[case("K♥", 134_236_965)]
    #[case("Q♥", 67_127_839)]
    #[case("J♥", 33_573_149)]
    #[case("T♥", 16_795_671)]
    #[case("9♥", 8_406_803)]
    #[case("8♥", 4_212_241)]
    #[case("7♥", 2_114_829)]
    #[case("6♥", 1_065_995)]
    #[case("5♥", 541_447)]
    #[case("4♥", 279_045)]
    #[case("3♥", 147_715)]
    #[case("2♥", 81_922)]
    #[case("A♦", 268_446_761)]
    #[case("K♦", 134_228_773)]
    #[case("Q♦", 67_119_647)]
    #[case("J♦", 33_564_957)]
    #[case("T♦", 16_787_479)]
    #[case("9♦", 8_398_611)]
    #[case("8♦", 4_204_049)]
    #[case("7♦", 2_106_637)]
    #[case("6♦", 1_057_803)]
    #[case("5♦", 533_255)]
    #[case("4♦", 270_853)]
    #[case("3♦", 139_523)]
    #[case("2♦", 73_730)]
    #[case("A♣", 268_442_665)]
    #[case("K♣", 134_224_677)]
    #[case("Q♣", 67_115_551)]
    #[case("J♣", 33_560_861)]
    #[case("T♣", 16_783_383)]
    #[case("9♣", 8_394_515)]
    #[case("8♣", 4_199_953)]
    #[case("7♣", 2_102_541)]
    #[case("6♣", 1_053_707)]
    #[case("5♣", 529_159)]
    #[case("4♣", 266_757)]
    #[case("3♣", 135_427)]
    #[case("2♣", 69_634)]
    fn from_poker_card__comprehensive(#[case] expected: &'static str, #[case] input: u32) {
        let actual = BitCard::from(input);
        assert_eq!(actual, BitCard::try_from(expected).unwrap());
    }

    #[test]
    fn from_u64__comprehensive_too() {
        let standard52 = cardpack::Standard52::default();
        for card in standard52.deck {
            let actual = BitCard::from(PlayingCard::from(&card).as_u32());
            assert_eq!(actual.to_card(), card);
        }
    }

    #[test]
    fn to_poker_card() {
        let standard52 = cardpack::Standard52::new_shuffled();
        for card in standard52.deck {
            let bit_card = BitCard::from(&card);
            assert_eq!(bit_card.to_poker_card(), PlayingCard::from(&card).as_u32());
        }
    }

    /// Round trip tests between `Card` and `BitCard`.
    #[test]
    fn to_card() {
        let standard52 = cardpack::Standard52::default();
        for card in standard52.deck {
            let bit_card = BitCard::from(&card);
            assert_eq!(bit_card.to_card(), card);

            let bit_card = BitCard::from(&card);
            assert_eq!(bit_card.to_card(), card);

            // Extremely over the top test
            let leaked: &'static str = Box::leak(card.clone().index.into_boxed_str());
            let bit_card = BitCard::try_from(leaked).unwrap();
            assert_eq!(bit_card.to_card(), card);
        }
    }

    #[test]
    fn get_rank() {
        assert_eq!(
            BitCard::try_from("AS").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::ACE)
        );
        assert_eq!(
            BitCard::try_from("KS").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::KING)
        );
        assert_eq!(
            BitCard::try_from("QS").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::QUEEN)
        );
        assert_eq!(
            BitCard::try_from("JS").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::JACK)
        );
        assert_eq!(
            BitCard::try_from("TS").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::TEN)
        );
        assert_eq!(
            BitCard::try_from("9S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::NINE)
        );
        assert_eq!(
            BitCard::try_from("8S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::EIGHT)
        );
        assert_eq!(
            BitCard::try_from("7S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::SEVEN)
        );
        assert_eq!(
            BitCard::try_from("6S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::SIX)
        );
        assert_eq!(
            BitCard::try_from("5S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::FIVE)
        );
        assert_eq!(
            BitCard::try_from("4S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::FOUR)
        );
        assert_eq!(
            BitCard::try_from("3S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::THREE)
        );
        assert_eq!(
            BitCard::try_from("2S").unwrap().get_rank(),
            cardpack::Rank::new(cardpack::TWO)
        );
    }

    #[test]
    fn get_rank_bitslice() {
        let card: BitCard = BitCard::try_from("KS").unwrap();
        assert_eq!(
            "[00001000, 00000000]",
            format!("{:b}", card.get_rank_bitslice())
        );
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            BitCard::try_from("AS").unwrap().get_suit(),
            cardpack::Suit::new(cardpack::SPADES)
        );

        assert_eq!(
            BitCard::try_from("2H").unwrap().get_suit(),
            cardpack::Suit::new(cardpack::HEARTS)
        );

        assert_eq!(
            BitCard::try_from("3♦").unwrap().get_suit(),
            cardpack::Suit::new(cardpack::DIAMONDS)
        );

        assert_eq!(
            BitCard::try_from("TC").unwrap().get_suit(),
            cardpack::Suit::new(cardpack::CLUBS)
        )
    }

    #[test]
    fn get_suit_bitslice() {
        let card: BitCard = BitCard::try_from("KS").unwrap();
        assert_eq!("[1000]", format!("{:04b}", card.get_suit_bitslice()));

        let card: BitCard = BitCard::try_from("KH").unwrap();
        assert_eq!("[0100]", format!("{:04b}", card.get_suit_bitslice()));

        let card: BitCard = BitCard::try_from("K♦").unwrap();
        assert_eq!("[0010]", format!("{:04b}", card.get_suit_bitslice()));

        let card: BitCard = BitCard::try_from("KC").unwrap();
        assert_eq!("[0001]", format!("{:04b}", card.get_suit_bitslice()));
    }

    #[rstest]
    #[case("2C", 4096)]
    #[case("2D", 8192)]
    #[case("2H", 16384)]
    #[case("2♠", 32768)]
    fn get_suit_binary_signature(#[case] index: &'static str, #[case] expected: u32) {
        let bit_card: BitCard = BitCard::try_from(index).unwrap();

        assert_eq!(bit_card.get_suit_binary_signature(), expected);
    }

    #[test]
    fn is_blank() {
        assert!(BitCard::default().is_blank());
    }

    #[test]
    fn is_blank__false() {
        assert!(!BitCard::try_from("KS").unwrap().is_blank());
    }

    #[test]
    fn or_rank_bitslice() {
        let ace_spades = BitCard::try_from("AS").unwrap();
        let king_spades = BitCard::try_from("KS").unwrap();
        let result = ace_spades.or_rank_bitslice(&king_spades.get_rank_bitslice());

        assert_eq!(format!("{}", result), "[00011000, 00000000]");
    }

    #[test]
    fn and_suit_bitslice() {
        let king_spades: BitCard = BitCard::try_from("KS").unwrap();
        let queen_spades: BitCard = BitCard::try_from("QS").unwrap();

        let actual = king_spades.or_suit_bitslice(&queen_spades.get_suit_bitslice());

        assert_eq!("[1000]", format!("{:04b}", actual));
    }

    #[test]
    fn or_suit_bitslice() {
        let king_spades: BitCard = BitCard::try_from("KS").unwrap();
        let king_hearts: BitCard = BitCard::try_from("KH").unwrap();
        let king_diamonds: BitCard = BitCard::try_from("KD").unwrap();
        let king_clubs: BitCard = BitCard::try_from("KC").unwrap();

        let actual = king_spades.or_suit_bitslice(&king_hearts.get_suit_bitslice());
        assert_eq!("[1100]", format!("{:04b}", actual));

        let actual = king_diamonds.or_suit_bitslice(&actual);
        assert_eq!("[1110]", format!("{:04b}", actual));

        let actual = king_clubs.or_suit_bitslice(&actual);
        assert_eq!("[1111]", format!("{:04b}", actual));
    }

    #[test]
    fn set_rank() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank(&card);
        assert_eq!(
            "00000000 00000000 00001011 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_flag() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank_flag(&card);
        assert_eq!(
            "00001000 00000000 00000000 00000000",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_rank_prime() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank_prime(&card);
        assert_eq!(
            "00000000 00000000 00000000 00100101",
            bit_card.display(true)
        );
    }

    #[test]
    fn set_suit() {
        let mut bit_card: BitCard = BitCard::default();

        let card = cardpack::Standard52::card_from_index("KS");
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 10000000 00000000",
            bit_card.display(true)
        );

        let card = cardpack::Standard52::card_from_index("KH");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 01000000 00000000",
            bit_card.display(true)
        );

        let card = cardpack::Standard52::card_from_index("K♦");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00100000 00000000",
            bit_card.display(true)
        );

        let card = cardpack::Standard52::card_from_index("KC");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(&card);
        assert_eq!(
            "00000000 00000000 00010000 00000000",
            bit_card.display(true)
        );
    }
}
