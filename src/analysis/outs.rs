use crate::types::playing_card::PlayingCard;
use crate::types::playing_cards::PlayingCards;
use indexmap::IndexMap;

/// TODO: Want to display outs based on best drawing hand.
#[derive(Clone, Debug)]
pub struct Outs(IndexMap<usize, PlayingCards>);

impl Outs {
    #[allow(clippy::missing_panics_doc, clippy::toplevel_ref_arg)]
    pub fn add(&mut self, player: usize, card: PlayingCard) {
        self.touch(player);

        // This code generates a clippy [toplevel_ref_arg](https://rust-lang.github.io/rust-clippy/master/index.html#toplevel_ref_arg)
        // warning, and recommends this code: `let set = &mut self.0.get_mut(&player).unwrap();`
        //
        // This code in turn generates a clippy [mut_mut lint warning](https://rust-lang.github.io/rust-clippy/master/index.html#mut_mut).
        // Damned if I do, damned if I don't.
        let ref mut set = self.0.get_mut(&player).unwrap();

        set.insert(card);
    }

    #[allow(clippy::missing_panics_doc, clippy::toplevel_ref_arg)]
    pub fn extend(&mut self, other: &Outs) {
        for (player, cards) in other.iter() {
            self.touch(*player);
            let ref mut set = self.0.get_mut(player).unwrap();
            set.append(cards);
        }
    }

    #[must_use]
    pub fn get(&self, player: usize) -> Option<&PlayingCards> {
        self.0.get(&player)
    }

    #[must_use]
    pub fn get_unless_most(&self, player: usize) -> Option<&PlayingCards> {
        self.0
            .get(&player)
            .filter(|&set| set.len() < self.len_most())
    }

    #[must_use]
    pub fn get_as_poker_cards(&self, player: usize) -> PlayingCards {
        match self.0.get(&player) {
            Some(set) => set.clone().sort(),
            None => PlayingCards::default(),
        }
    }

    /// This is used if you want to filter out the hand with the most outs.
    #[must_use]
    pub fn len_most(&self) -> usize {
        let mut longest = 0_usize;
        for (_, cards) in self.iter() {
            if cards.len() > longest {
                longest = cards.len();
            }
        }
        longest
    }

    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<'_, usize, PlayingCards> {
        self.0.iter()
    }

    pub fn touch(&mut self, player: usize) -> bool {
        if self.0.get(&player).is_none() {
            self.0.insert(player, PlayingCards::default());
            true
        } else {
            false
        }
    }
}

impl Default for Outs {
    fn default() -> Outs {
        Outs(IndexMap::new())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_outs_tests {
    use super::*;

    #[test]
    fn touch() {
        let mut outs = Outs::default();

        let touched = outs.touch(1);

        assert!(touched);
        assert_eq!(PlayingCards::default(), *outs.get(1).unwrap());
        assert!(outs.get(2).is_none());
    }

    #[test]
    fn add() {
        let mut outs = Outs::default();

        outs.add(1, PlayingCard::from(PlayingCard::SIX_SPADES));
        outs.add(1, PlayingCard::from(PlayingCard::SEVEN_CLUBS));

        assert_eq!("6♠ 7♣", outs.get(1).unwrap().to_string());
    }

    #[test]
    fn fold() {
        let mut outs1 = Outs::default();
        let mut outs2 = Outs::default();
        outs1.add(1, PlayingCard::from(PlayingCard::SIX_SPADES));
        outs2.add(1, PlayingCard::from(PlayingCard::SEVEN_CLUBS));

        outs1.extend(&outs2);

        assert_eq!("7♣ 6♠", outs1.get_as_poker_cards(1).to_string());
    }

    #[test]
    fn get_as_poker_cards() {
        let mut outs = Outs::default();

        outs.add(1, PlayingCard::from(PlayingCard::ACE_CLUBS));
        outs.add(1, PlayingCard::from(PlayingCard::ACE_DIAMONDS));
        outs.add(2, PlayingCard::from(PlayingCard::KING_CLUBS));
        outs.add(2, PlayingCard::from(PlayingCard::KING_DIAMONDS));

        assert_eq!("A♦ A♣", outs.get_as_poker_cards(1).to_string());
        assert_eq!("K♦ K♣", outs.get_as_poker_cards(2).to_string());
        assert_eq!("", outs.get_as_poker_cards(3).to_string());
    }
}
