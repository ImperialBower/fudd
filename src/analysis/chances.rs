use crate::analysis::outs::Outs;
use std::collections::BTreeMap;

/// Entity to hold percentages mapped to unsigned integers. Used to store calculations
/// of win percentages for specific players at specific times, and provide an easy
/// way to display those results.
#[derive(Clone, Debug, Default)]
pub struct Chances(BTreeMap<usize, f32>);

impl Chances {
    #[must_use]
    pub fn get(&self, seat: usize) -> f32 {
        match self.0.get(&seat) {
            Some(chance) => *chance,
            None => 0_f32,
        }
    }

    /// Returns true if the `Chances'` percentages add up to 100%.
    #[must_use]
    pub fn keeping_it_100(&self) -> bool {
        (((self.total_percentage() * 100.0).round() / 100.0) - 100.0).abs() < f32::EPSILON
    }

    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, usize, f32> {
        self.0.keys()
    }

    pub fn seats(&self) -> std::collections::btree_map::Keys<'_, usize, f32> {
        self.0.keys()
    }

    pub fn set(&mut self, seat: usize, chances: f32) -> bool {
        if self.0.get(&seat).is_none() {
            self.0.insert(seat, chances);
            return true;
        }
        false
    }

    #[must_use]
    pub fn total_percentage(&self) -> f32 {
        let mut total = 0_f32;
        for seat in self.keys() {
            total += self.get(*seat);
        }
        total
    }

    pub fn playout(&self) {
        for k in self.keys() {
            println!("Seat {}: {:.1}%", k, self.get(*k));
        }
    }

    /// `Table` now does this as well as displaying the best possible hand for the `seat`
    /// in the lead.
    pub fn playout_with_outs(&self, outs: &Outs) {
        for k in self.keys() {
            let player_outs = outs.get_unless_most(*k);
            match player_outs {
                Some(o) => println!("Seat {}: {:.1}% - Outs: {}", k, self.get(*k), o),
                None => println!("Seat {}: {:.1}%", k, self.get(*k)),
            };
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_chances_tests {
    use super::*;

    #[test]
    fn get() {
        let mut chances = Chances::default();
        chances.set(1, 25_f32);
        chances.set(2, 0.00185);

        assert_eq!(25_f32, chances.get(1));
        assert_eq!(0.00185, chances.get(2));
        assert_eq!(0_f32, chances.get(3));
    }

    #[test]
    fn total_percentage() {
        let mut chances = Chances::default();
        chances.set(1, 25_f32);
        chances.set(2, 0.00185);

        assert_eq!(25.00185, chances.total_percentage());
    }

    #[test]
    fn total_percentage_100() {
        let mut chances = Chances::default();
        chances.set(1, 25_f32);
        chances.set(2, 25_f32);
        chances.set(4, 25_f32);
        chances.set(9, 25_f32);

        assert_eq!(100.0, chances.total_percentage());
        assert!(chances.keeping_it_100());
    }

    #[test]
    fn total_percentage_125() {
        let mut chances = Chances::default();
        chances.set(1, 50_f32);
        chances.set(2, 25_f32);
        chances.set(4, 25_f32);
        chances.set(9, 25_f32);

        assert_eq!(125.0, chances.total_percentage());
        assert!(!chances.keeping_it_100());
    }
}
