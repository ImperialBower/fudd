use ckc_rs::CardNumber;
use fudd::types::arrays::two_cards::TwoCards;
use fudd::types::ranges::two_cards_set::TwoCardsSet;

fn main() {
    let hand = TwoCards::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES).unwrap();

    let every_other = hand.every_other().two_cards_vec();

    for (i, hand) in every_other.iter().enumerate() {
        println!("{} {}", i, hand);
    }

    println!("{}", TwoCardsSet::every().len());
}
