use ckc_rs::cards::binary_card::{BinaryCard, BC64};
use ckc_rs::cards::HandRanker;
use fudd::types::arrays::seven_card::SevenCard;
use fudd::types::playing_cards::PlayingCards;
use num_format::{Locale, ToFormattedString};

fn main() {
    let now = std::time::Instant::now();

    let deck = PlayingCards::deck();
    for (i, b) in deck.combinations(7).enumerate() {
        let seven_card = SevenCard::try_from(b).unwrap();
        let seven = seven_card.to_seven();
        let rank = seven.hand_rank();
        let bc = BinaryCard::from_seven(seven);
        println!(
            "#{}: {} {}, {}",
            i.to_formatted_string(&Locale::en),
            bc,
            seven_card,
            rank
        );
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
