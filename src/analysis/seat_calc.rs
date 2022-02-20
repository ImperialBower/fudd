use crate::analysis::chances::Chances;
use crate::analysis::count::Count;
use crate::analysis::Evaluate;
use std::collections::HashMap;

/// Utility class to facilitate displaying the winning percentages for each player.
#[derive(Clone, Debug, Default)]
pub struct SeatCalc(HashMap<usize, Count>);

impl SeatCalc {
    #[must_use]
    pub fn get(&self, seat: usize) -> usize {
        match self.0.get(&seat) {
            Some(count) => count.get(),
            None => 0,
        }
    }

    /// Creates a zero value entry for a specific seat.
    pub fn touch(&mut self, seat: usize) -> usize {
        self.plus(seat, 0)
    }

    /// Increments the `Count` for the seat number passed in by 1.
    pub fn increment(&mut self, seat: usize) -> usize {
        self.plus(seat, 1)
    }

    /// # Panics
    ///
    /// Shouldn't be possible.
    pub fn plus(&mut self, seat: usize, add: usize) -> usize {
        if self.0.get(&seat).is_none() {
            self.0.insert(seat, Count::new());
        }
        self.0.get(&seat).unwrap().plus(add)
    }

    #[must_use]
    pub fn percentage(&self, seat: usize, of: usize) -> f32 {
        Evaluate::percent(self.get(seat), of)
    }

    #[must_use]
    pub fn chances(&self, against: usize) -> Chances {
        let mut chances = Chances::default();
        for seat in self.0.keys() {
            chances.set(*seat, self.percentage(*seat, against));
        }
        chances
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_seat_calc_tests {
    use super::*;

    #[test]
    fn increment() {
        let mut calc = SeatCalc::default();
        calc.increment(4);
        calc.increment(4);
        calc.increment(2);

        assert_eq!(0, calc.get(0));
        assert_eq!(2, calc.get(4));
        assert_eq!(1, calc.get(2));
    }

    #[test]
    fn percentage() {
        let mut calc = SeatCalc::default();

        calc.plus(1, 48);
        calc.plus(2, 4);
        calc.plus(3, 1316);

        assert_eq!("0.00185%", format!("{:.5}%", calc.percentage(1, 2_598_960)));
        assert_eq!("0.00015%", format!("{:.5}%", calc.percentage(2, 2_598_960)));
        assert_eq!("0.05064%", format!("{:.5}%", calc.percentage(3, 2_598_960)));
    }

    #[test]
    fn chances__50_50() {
        let mut calc = SeatCalc::default();
        calc.plus(1, 50);
        calc.plus(2, 50);

        let chances = calc.chances(100);
        assert_eq!("50%", format!("{:.0}%", chances.get(1)));
        assert_eq!("50%", format!("{:.0}%", chances.get(2)));
        assert_eq!("0%", format!("{:.0}%", chances.get(3)));
    }
}
