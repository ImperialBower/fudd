use crate::types::arrays::Vectorable;
use crate::types::slots::flop::Flop;
use crate::types::U32Card;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreeCard(pub [U32Card; 3]);

impl ThreeCard {
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

impl Vectorable for ThreeCard {
    #[must_use]
    fn to_vec(&self) -> Vec<U32Card> {
        self.0.to_vec()
    }
}

impl From<&Flop> for ThreeCard {
    fn from(flop: &Flop) -> Self {
        ThreeCard(flop.to_array())
    }
}
