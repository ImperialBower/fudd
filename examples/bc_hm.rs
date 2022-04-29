use ckc_rs::cards::binary_card::{BinaryCard, BC64};
use ckc_rs::cards::HandRanker;
use fudd::types::arrays::seven_card::SevenCard;
use fudd::types::playing_cards::PlayingCards;

fn main() {
    let now = std::time::Instant::now();

    let deck = PlayingCards::deck();
    for (_, b) in deck.combinations(7).enumerate() {
        let seven = SevenCard::try_from(b).unwrap().to_seven();
        let rank = seven.hand_rank_value();
        let bc = BinaryCard::from_seven(seven);
        println!("{} => {}", bc, rank);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
