use fudd::games::holdem::deal::Deal;
use fudd::types::playing_cards::PlayingCards;
use serde_json::Result;
use std::fs;

/// Working to get a satisfying Serde serialization and deserialization.
fn main() -> Result<()> {
    let index_string = "2S 3D 3S QS QD KH 3C 9H 3H 6H 4H 2H 5S 6D 9S 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD TC AD";
    let mut deal = Deal::from_deck(PlayingCards::try_from(index_string).unwrap()).unwrap();
    deal.deal(2);
    deal.flop();
    // table.fold(1);
    deal.turn();
    deal.river();

    let (winning_seats, winning_hand) = deal.winner();

    println!("The Board: {}", deal.table.board);

    println!("Winner:");
    for i in winning_seats.iter() {
        println!("   Player {}", i + 1);
    }

    println!("The Winning Hand:{}", winning_hand);

    let j = serde_json::to_string(&deal.table)?;
    let y = serde_yaml::to_string(&deal.table).unwrap();

    fs::write("logs/example.json", j.clone()).expect("Unable to write file");
    fs::write("logs/example.yaml", y.clone()).expect("Unable to write file");

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
    println!("{}", y);

    Ok(())
}
