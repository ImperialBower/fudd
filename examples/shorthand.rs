use fudd::games::holdem::deal::Deal;

/// This example spits out a shorthand random poker deal with between 2 and 11 players.
/// Run the hand again through calc to see the odds.
fn main() {
    let (deal, deck) = Deal::sample();

    println!("SHUFFLE: {}\n", deck);

    println!("{}", deal.table.players);
    println!("{}", deal.table.board);

    let (winning_seat, winning_hand) = deal.winner();
    println!(
        "\nWinning seats: {:?} with the hand {}",
        winning_seat, winning_hand
    );
    println!("\nFor odds:");
    println!("{}", deal.table.format_calc());
}
