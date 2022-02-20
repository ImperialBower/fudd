use crate::analysis::eval_7card::Eval7Card;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Evals7Card(Vec<Eval7Card>);

impl Evals7Card {
    pub fn push(&mut self, eval: Eval7Card) {
        self.0.push(eval);
    }

    #[must_use]
    pub fn to_vec(&self) -> &Vec<Eval7Card> {
        &self.0
    }
}
