use crate::analysis::eval::Eval;
use crate::types::arrays::five_cards::FiveCards;
use crate::types::arrays::seven_cards::SevenCards;
use crate::types::arrays::two_cards::TwoCards;
use crate::types::arrays::Evaluable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Eval7Card {
    pub eval: Eval,
    pub cards: SevenCards,
}

impl Eval7Card {
    #[must_use]
    pub fn new(cards: SevenCards, eval: Eval) -> Eval7Card {
        Eval7Card { eval, cards }
    }

    #[must_use]
    pub fn from_holdem(two_cards: TwoCards, five_cards: FiveCards) -> Eval7Card {
        Eval7Card::from(SevenCards::new(two_cards, five_cards))
    }
}

impl From<SevenCards> for Eval7Card {
    fn from(cards: SevenCards) -> Self {
        Eval7Card::new(cards, cards.eval())
    }
}
