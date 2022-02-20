use crate::types::arrays::Vectorable;
use crate::types::slots::flop::Flop;
use crate::types::U32Card;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreeCards(pub [U32Card; 3]);

impl ThreeCards {
    //region getters
    #[must_use]
    pub fn first(&self) -> U32Card {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> U32Card {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> U32Card {
        self.0[2]
    }
    //endregion

    #[must_use]
    pub fn to_arr(&self) -> [U32Card; 3] {
        self.0
    }
}

impl Vectorable for ThreeCards {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.to_vec()
    }
}

impl From<&Flop> for ThreeCards {
    fn from(flop: &Flop) -> Self {
        ThreeCards(flop.to_array())
    }
}
