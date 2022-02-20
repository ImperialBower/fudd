use fudd::analysis::eval::Eval;
use fudd::types::poker_deck::PokerDeck;

/// This example shows off some of the core entities in the library.
///
/// At the core of the library is the `PokerHand`, with its ability to evaluation
/// the strength for a specific collection of five cards.
fn main() {
    // Take a traditional 52 card `PokerDeck`, and return the cards
    // as a shuffled `PokerCards` collection.
    let mut deck = PokerDeck::poker_cards_shuffled();

    // Deal two five card `PokerHands` from the deck.
    let first_hand = Eval::try_from(&deck.deal(5)).unwrap();
    let second_hand = Eval::try_from(&deck.deal(5)).unwrap();

    // See which hand wins:
    if first_hand > second_hand {
        println!("{} beats \n{}", first_hand, second_hand);
    } else {
        println!("{} is beaten by \n{}", first_hand, second_hand);
    }
}
