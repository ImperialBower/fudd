use fudd::types::ranges::two_cards_set::TwoCardsSet;

fn main() {
    let every = TwoCardsSet::every().two_cards_vec();

    for (i, hand) in every.iter().enumerate() {
        println!("{} {}", i, hand);
    }

    println!("{}", every.len());
}
