use crate::analysis::eval::Eval;
use crate::types::arrays::five_card::FiveCard;
use crate::types::arrays::seven_card::SevenCard;
use crate::types::arrays::two_card::TwoCard;
use crate::types::arrays::Evaluable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Eval7Card {
    pub eval: Eval,
    pub cards: SevenCard,
}

impl Eval7Card {
    #[must_use]
    pub fn new(cards: SevenCard, eval: Eval) -> Eval7Card {
        Eval7Card { eval, cards }
    }

    #[must_use]
    pub fn from_holdem(two_cards: TwoCard, five_cards: FiveCard) -> Eval7Card {
        Eval7Card::from(SevenCard::new(two_cards, five_cards))
    }
}

impl From<SevenCard> for Eval7Card {
    fn from(cards: SevenCard) -> Self {
        Eval7Card::new(cards, cards.eval())
    }
}
