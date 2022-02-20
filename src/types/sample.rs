use rand::{thread_rng, Rng};

/// `Vector` sampler. Idea from [here](https://stackoverflow.com/questions/53755017/can-i-randomly-sample-from-a-hashset-efficiently).
pub trait Sample {
    type Item;

    fn sample(&mut self) -> Option<Self::Item>;
}

impl<T> Sample for Vec<T> {
    type Item = T;

    fn sample(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();
        if self.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.len());
            Some(self.swap_remove(index))
        }
    }
}
