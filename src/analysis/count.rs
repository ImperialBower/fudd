use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;

/// `Count` compromises a single `Cell` holding an unsigned integer that can be
/// easily incremented and decremented. Used to count specific wins by a player
/// so that odds can be calculated.
#[derive(Clone, Debug)]
pub struct Count(Cell<usize>);

impl Count {
    #[must_use]
    pub fn new() -> Count {
        Count(Cell::new(0))
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }

    pub fn increment(&self) -> usize {
        self.plus(1)
    }

    pub fn decrement(&self) -> usize {
        self.minus(1)
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn minus(&self, number: usize) -> usize {
        if (self.get() as isize - number as isize) < 1 {
            self.0.set(0);
        } else {
            self.0.set(self.0.get() - number);
        }
        self.0.get()
    }

    pub fn plus(&self, add: usize) -> usize {
        self.0.set(self.0.get() + add);
        self.0.get()
    }
}

impl Default for Count {
    fn default() -> Self {
        Count::new()
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

impl Eq for Count {}

impl PartialEq<Self> for Count {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl PartialOrd<Self> for Count {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Count {
    fn cmp(&self, other: &Count) -> Ordering {
        self.get().cmp(&other.get())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_count_tests {
    use super::*;

    #[test]
    fn increment() {
        let count = Count::default();
        count.increment();
        count.increment();

        assert_eq!("2", count.to_string());
    }

    #[test]
    fn decrement() {
        let count = Count::default();
        count.increment();
        count.increment();
        count.increment();
        count.decrement();

        assert_eq!(2, count.get());
    }

    #[test]
    fn decrement__overflow() {
        let count = Count::default();
        count.decrement();

        assert_eq!(0, count.get());
    }

    #[test]
    fn minus() {
        let count = Count::default();
        count.plus(8);
        count.minus(7);

        assert_eq!(1, count.get());
    }

    #[test]
    fn minus__overflow() {
        let count = Count::default();
        count.plus(8);
        count.minus(99);

        assert_eq!(0, count.get());
    }

    #[test]
    fn plus() {
        let count = Count::default();
        count.plus(4);
        count.increment();

        assert_eq!(5, count.get());
    }

    #[test]
    fn default() {
        assert_eq!("0", Count::default().to_string());
    }

    #[test]
    fn equal() {
        let count1 = Count::default();
        let count2 = Count::default();

        count1.increment();
        count2.increment();

        assert_eq!(count1, count2);
    }

    #[test]
    fn greater() {
        let count1 = Count::default();
        let count2 = Count::default();

        count1.increment();

        assert!(count1 > count2);
    }

    #[test]
    fn lesser() {
        let count1 = Count::default();
        let count2 = Count::default();

        count2.increment();

        assert!(count1 < count2);
    }
}
