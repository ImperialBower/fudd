extern crate log;

use fudd::games::holdem::table::Table;
use fudd::types::card_slot::CardSlot;
use fudd::types::playing_cards::PlayingCards;

/// Spits out a quick three handed random hand of poker, doing the full percentages
/// and outs.
fn main() {
    let player_count: usize = 3;
    let table = Table::seat(player_count);
    let mut cards = PlayingCards::deck_shuffled();

    for _ in 0..(player_count * 2) + 5 {
        table.take(cards.draw_one());
    }

    // Print a prettified version of the command back out.
    println!("{}", table.format_indexer());
    println!("{}", table.format_calc());

    env_logger::init();

    table.play_out();
}
