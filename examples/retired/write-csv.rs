use fudd::analysis::store::holdem::heads_up_row::HeadsUpRow;
use fudd::games::holdem::hand::Hand;
use fudd::types::playing_card::PlayingCard;
use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(HeadsUpRow::new(
        Hand::new(PlayingCard::ACE_CLUBS, PlayingCard::KING_DIAMONDS),
        Hand::new(PlayingCard::ACE_SPADES, PlayingCard::KING_SPADES),
        0,
        0,
        0,
    ))?;

    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!(">>> {}", err);
        process::exit(1);
    }
}
