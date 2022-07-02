use crate::analysis::eval::Eval;
use crate::analysis::evals::EvalsPerClass;
use ckc_rs::hand_rank::HandRankValue;
use indexmap::IndexMap;

/// Ordered `HashMap` of a collection of hand `Evals` using the
/// [indexmap](https://github.com/bluss/indexmap) crate.
///
/// **NOTE**: This library does not automatically sort the index. Use `Evals` for this.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Indexed(IndexMap<HandRankValue, Eval>);

impl Indexed {
    #[must_use]
    pub fn index_map(&self) -> &IndexMap<HandRankValue, Eval> {
        &self.0
    }
}

impl From<&EvalsPerClass> for Indexed {
    fn from(evals: &EvalsPerClass) -> Self {
        let mut im: IndexMap<HandRankValue, Eval> = IndexMap::new();
        for e in evals.to_vec() {
            im.insert(e.rank.value, *e);
        }
        Indexed(im)
    }
}
