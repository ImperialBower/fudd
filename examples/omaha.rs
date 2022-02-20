use fudd::types::arrays::four_cards::FourCards;
use fudd::types::card_slot::CardSlot;
use fudd::types::poker_deck::PokerDeck;
use fudd::types::slots::flop::Flop;

fn main() {
    let mut deck = PokerDeck::poker_cards_shuffled();

    let flop = Flop::default();
    flop.take_from_poker_cards(&deck.deal(3));

    // Deal two Omama hands from the deck.
    let first_hand = FourCards::try_from(&deck.deal(4)).unwrap();
    let second_hand = FourCards::try_from(&deck.deal(4)).unwrap();
    let third_hand = FourCards::try_from(&deck.deal(4)).unwrap();
    let fourth_hand = FourCards::try_from(&deck.deal(4)).unwrap();
    let fifth_hand = FourCards::try_from(&deck.deal(4)).unwrap();
    let sixth_hand = FourCards::try_from(&deck.deal(4)).unwrap();

    // What are the outs?
    let first_outs = first_hand.straight_outs_at_flop(&flop);
    let second_outs = second_hand.straight_outs_at_flop(&flop);
    let third_outs = third_hand.straight_outs_at_flop(&flop);
    let fourth_outs = fourth_hand.straight_outs_at_flop(&flop);
    let fifth_outs = fifth_hand.straight_outs_at_flop(&flop);
    let sixth_outs = sixth_hand.straight_outs_at_flop(&flop);

    println!("Flop: {}", flop);
    println!(
        "Player #1: {} - {} Wrap Outs: {}",
        first_hand,
        first_outs.len(),
        first_outs
    );
    println!(
        "Player #2: {} - {} Wrap Outs: {}",
        second_hand,
        second_outs.len(),
        second_outs
    );
    println!(
        "Player #3: {} - {} Wrap Outs: {}",
        third_hand,
        third_outs.len(),
        third_outs
    );
    println!(
        "Player #4: {} - {} Wrap Outs: {}",
        fourth_hand,
        fourth_outs.len(),
        fourth_outs
    );
    println!(
        "Player #5: {} - {} Wrap Outs: {}",
        fifth_hand,
        fifth_outs.len(),
        fifth_outs
    );
    println!(
        "Player #6: {} - {} Wrap Outs: {}",
        sixth_hand,
        sixth_outs.len(),
        sixth_outs
    );
}
