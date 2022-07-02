use crate::types::arrays::Vectorable;
use crate::types::slots::flop::Flop;
use ckc_rs::CKCNumber;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreeCard(pub [CKCNumber; 3]);

impl ThreeCard {
    //region getters
    #[must_use]
    pub fn first(&self) -> CKCNumber {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> CKCNumber {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> CKCNumber {
        self.0[2]
    }
    //endregion

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 3] {
        self.0
    }
}

impl Vectorable for ThreeCard {
    #[must_use]
    fn to_vec(&self) -> Vec<CKCNumber> {
        self.0.to_vec()
    }
}

impl From<&Flop> for ThreeCard {
    fn from(flop: &Flop) -> Self {
        ThreeCard(flop.to_array())
    }
}
