use crate::types::bitvec::bit_card::BitCard;
use bitvec::field::BitField;
use ckc_rs::{CKCNumber, CardNumber, PokerCard, Shifty};
use serde::de::Deserializer;
use serde::ser::{Serialize, Serializer};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlayingCard(#[serde(deserialize_with = "deserialize_card_index")] CKCNumber);

impl PlayingCard {
    //region cards
    pub const ACE_SPADES: PlayingCard = PlayingCard(CardNumber::ACE_SPADES);
    pub const KING_SPADES: PlayingCard = PlayingCard(CardNumber::KING_SPADES);
    pub const QUEEN_SPADES: PlayingCard = PlayingCard(CardNumber::QUEEN_SPADES);
    pub const JACK_SPADES: PlayingCard = PlayingCard(CardNumber::JACK_SPADES);
    pub const TEN_SPADES: PlayingCard = PlayingCard(CardNumber::TEN_SPADES);
    pub const NINE_SPADES: PlayingCard = PlayingCard(CardNumber::NINE_SPADES);
    pub const EIGHT_SPADES: PlayingCard = PlayingCard(CardNumber::EIGHT_SPADES);
    pub const SEVEN_SPADES: PlayingCard = PlayingCard(CardNumber::SEVEN_SPADES);
    pub const SIX_SPADES: PlayingCard = PlayingCard(CardNumber::SIX_SPADES);
    pub const FIVE_SPADES: PlayingCard = PlayingCard(CardNumber::FIVE_SPADES);
    pub const FOUR_SPADES: PlayingCard = PlayingCard(CardNumber::FOUR_SPADES);
    pub const TREY_SPADES: PlayingCard = PlayingCard(CardNumber::TREY_SPADES);
    pub const DEUCE_SPADES: PlayingCard = PlayingCard(CardNumber::DEUCE_SPADES);
    pub const ACE_HEARTS: PlayingCard = PlayingCard(CardNumber::ACE_HEARTS);
    pub const KING_HEARTS: PlayingCard = PlayingCard(CardNumber::KING_HEARTS);
    pub const QUEEN_HEARTS: PlayingCard = PlayingCard(CardNumber::QUEEN_HEARTS);
    pub const JACK_HEARTS: PlayingCard = PlayingCard(CardNumber::JACK_HEARTS);
    pub const TEN_HEARTS: PlayingCard = PlayingCard(CardNumber::TEN_HEARTS);
    pub const NINE_HEARTS: PlayingCard = PlayingCard(CardNumber::NINE_HEARTS);
    pub const EIGHT_HEARTS: PlayingCard = PlayingCard(CardNumber::EIGHT_HEARTS);
    pub const SEVEN_HEARTS: PlayingCard = PlayingCard(CardNumber::SEVEN_HEARTS);
    pub const SIX_HEARTS: PlayingCard = PlayingCard(CardNumber::SIX_HEARTS);
    pub const FIVE_HEARTS: PlayingCard = PlayingCard(CardNumber::FIVE_HEARTS);
    pub const FOUR_HEARTS: PlayingCard = PlayingCard(CardNumber::FOUR_HEARTS);
    pub const TREY_HEARTS: PlayingCard = PlayingCard(CardNumber::TREY_HEARTS);
    pub const DEUCE_HEARTS: PlayingCard = PlayingCard(CardNumber::DEUCE_HEARTS);
    pub const ACE_DIAMONDS: PlayingCard = PlayingCard(CardNumber::ACE_DIAMONDS);
    pub const KING_DIAMONDS: PlayingCard = PlayingCard(CardNumber::KING_DIAMONDS);
    pub const QUEEN_DIAMONDS: PlayingCard = PlayingCard(CardNumber::QUEEN_DIAMONDS);
    pub const JACK_DIAMONDS: PlayingCard = PlayingCard(CardNumber::JACK_DIAMONDS);
    pub const TEN_DIAMONDS: PlayingCard = PlayingCard(CardNumber::TEN_DIAMONDS);
    pub const NINE_DIAMONDS: PlayingCard = PlayingCard(CardNumber::NINE_DIAMONDS);
    pub const EIGHT_DIAMONDS: PlayingCard = PlayingCard(CardNumber::EIGHT_DIAMONDS);
    pub const SEVEN_DIAMONDS: PlayingCard = PlayingCard(CardNumber::SEVEN_DIAMONDS);
    pub const SIX_DIAMONDS: PlayingCard = PlayingCard(CardNumber::SIX_DIAMONDS);
    pub const FIVE_DIAMONDS: PlayingCard = PlayingCard(CardNumber::FIVE_DIAMONDS);
    pub const FOUR_DIAMONDS: PlayingCard = PlayingCard(CardNumber::FOUR_DIAMONDS);
    pub const TREY_DIAMONDS: PlayingCard = PlayingCard(CardNumber::TREY_DIAMONDS);
    pub const DEUCE_DIAMONDS: PlayingCard = PlayingCard(CardNumber::DEUCE_DIAMONDS);
    pub const ACE_CLUBS: PlayingCard = PlayingCard(CardNumber::ACE_CLUBS);
    pub const KING_CLUBS: PlayingCard = PlayingCard(CardNumber::KING_CLUBS);
    pub const QUEEN_CLUBS: PlayingCard = PlayingCard(CardNumber::QUEEN_CLUBS);
    pub const JACK_CLUBS: PlayingCard = PlayingCard(CardNumber::JACK_CLUBS);
    pub const TEN_CLUBS: PlayingCard = PlayingCard(CardNumber::TEN_CLUBS);
    pub const NINE_CLUBS: PlayingCard = PlayingCard(CardNumber::NINE_CLUBS);
    pub const EIGHT_CLUBS: PlayingCard = PlayingCard(CardNumber::EIGHT_CLUBS);
    pub const SEVEN_CLUBS: PlayingCard = PlayingCard(CardNumber::SEVEN_CLUBS);
    pub const SIX_CLUBS: PlayingCard = PlayingCard(CardNumber::SIX_CLUBS);
    pub const FIVE_CLUBS: PlayingCard = PlayingCard(CardNumber::FIVE_CLUBS);
    pub const FOUR_CLUBS: PlayingCard = PlayingCard(CardNumber::FOUR_CLUBS);
    pub const TREY_CLUBS: PlayingCard = PlayingCard(CardNumber::TREY_CLUBS);
    pub const DEUCE_CLUBS: PlayingCard = PlayingCard(CardNumber::DEUCE_CLUBS);
    pub const BLANK: PlayingCard = PlayingCard(CardNumber::BLANK);

    //endregion

    #[must_use]
    pub fn as_card(&self) -> cardpack::Card {
        BitCard::from(self.0).to_card()
    }

    #[must_use]
    pub fn simple_index(&self) -> String {
        format!("{}{}", self.get_rank_char(), self.get_suit_letter())
    }
}

impl PokerCard for PlayingCard {
    fn as_u32(&self) -> u32 {
        self.0
    }

    fn is_blank(&self) -> bool {
        self.0 == CardNumber::BLANK
    }
}

impl fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_rank_char(), self.get_suit_char())
    }
}

impl From<&BitCard> for PlayingCard {
    fn from(bitcard: &BitCard) -> Self {
        PlayingCard(bitcard.as_bitslice().load_be::<u32>())
    }
}

/// Returns a `PlayingCard` mapped from the Rank and Suit values of a
/// standard 52 `cardpack::Card`. If the Card passed in isn't in the
/// standard 52 deck, it will return a `Blank` `PlayingCard`.
impl From<&cardpack::Card> for PlayingCard {
    fn from(value: &cardpack::Card) -> PlayingCard {
        let suit: u32 = value.suit.binary_signature_revised();
        let bits = 1 << (16 + value.rank.weight);
        let rank_eight = value.rank.weight << 8;

        PlayingCard::from(bits | value.rank.prime | rank_eight | suit)
    }
}

impl From<&'static u32> for PlayingCard {
    fn from(value: &'static u32) -> Self {
        PlayingCard::from(*value)
    }
}

impl From<&'static str> for PlayingCard {
    fn from(value: &'static str) -> PlayingCard {
        PlayingCard(CKCNumber::from_index(value))
    }
}

/// Sieve to ensure that only valid binary card representations are passed in.
///
/// Invalid `CardNumbers` will return a `Blank` `PlayingCard`.
impl From<CKCNumber> for PlayingCard {
    fn from(value: CKCNumber) -> Self {
        PlayingCard(ckc_rs::CardNumber::filter(value))
    }
}

impl From<String> for PlayingCard {
    fn from(string: String) -> Self {
        PlayingCard::from(&cardpack::Standard52::card_from_string(string))
    }
}

impl Serialize for PlayingCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("PlayingCard", &self.to_string())
    }
}

fn deserialize_card_index<'de, D>(deserializer: D) -> Result<CKCNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(PlayingCard::from(buf).as_u32())
}

impl Shifty for PlayingCard {
    fn shift_suit(&self) -> Self {
        PlayingCard(self.as_u32().shift_suit())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod holdem_playing_card_tests {
    use super::*;
    use crate::types::poker_deck::PokerDeck;
    use cardpack::Standard52;
    use ckc_rs::PokerCard;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn compare() {
        assert!(PlayingCard::KING_SPADES > PlayingCard::QUEEN_CLUBS);
        assert!(PlayingCard::ACE_CLUBS > PlayingCard::KING_SPADES);
        assert!(PlayingCard::TEN_CLUBS < PlayingCard::KING_SPADES);
    }

    #[test]
    fn from__card() {
        let card = cardpack::Standard52::card_from_index("QH");
        let playing_card = PlayingCard::from(&card);

        assert!(!playing_card.is_blank());
        assert_eq!("Q♥", playing_card.to_string());
    }

    #[test]
    fn from__card__invalid() {
        let card = cardpack::Standard52::card_from_index("JK");
        let playing_card = PlayingCard::from(&card);

        assert!(playing_card.is_blank());
        assert_eq!("__", playing_card.to_string());
    }

    #[test]
    fn from__card_number() {
        let playing_card = PlayingCard::from(PlayingCard::SIX_SPADES);

        assert!(!playing_card.is_blank());
        assert_eq!("6♠", playing_card.to_string());
    }

    #[test]
    fn from__card_number__invalid() {
        let playing_card = PlayingCard::from(12);

        assert!(playing_card.is_blank());
        assert_eq!("__", playing_card.to_string());
    }

    #[test]
    fn from__index() {
        let playing_card = PlayingCard::from("QH");

        assert!(!playing_card.is_blank());
        assert_eq!("Q♥", playing_card.to_string());
    }

    #[test]
    fn from__index__invalid() {
        let playing_card = PlayingCard::from("JK");

        assert!(playing_card.is_blank());
        assert_eq!("__", playing_card.to_string());
    }

    #[test]
    fn display() {
        let card = cardpack::Standard52::card_from_index("QH");
        let playing_card = PlayingCard::from("QH");

        assert_eq!("QH", format!("{}", card));
        assert_eq!("Q♥", format!("{}", playing_card));
    }

    #[test]
    fn display__blank() {
        assert_eq!("__", PlayingCard::default().to_string())
    }

    #[test]
    fn simple_index() {
        let playing_card = PlayingCard::from("QH");

        assert_eq!("QH", playing_card.simple_index());
    }

    #[test]
    fn deck() {
        let standard52 = Standard52::default();

        for (i, card) in standard52.deck.into_iter().enumerate() {
            let playing_card = PlayingCard::from(&card);
            assert_eq!(PokerDeck::get(i), playing_card.as_u32());
            assert_eq!(playing_card.as_card(), card);
        }
    }

    #[test]
    fn shifty() {
        assert_eq!(
            PlayingCard::from("QH"),
            PlayingCard::from("QS").shift_suit()
        )
    }

    // https://serde.rs/unit-testing.html
    #[test]
    fn serialize() {
        let playing_card = PlayingCard::from("QH");

        assert_tokens(
            &playing_card,
            &[
                Token::NewtypeStruct {
                    name: "PlayingCard",
                },
                Token::Str("Q♥"),
            ],
        );
    }
}
