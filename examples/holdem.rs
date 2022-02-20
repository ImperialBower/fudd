use fudd::games::holdem::deal::Deal;
use rand::Rng;

/// Run through a complete random deal for a random number of players..
fn main() {
    println!("Shuffle up and deal!");

    let mut rng = rand::thread_rng();
    let number_of_players = rng.gen_range(2..8);

    let mut deal = Deal::default();
    deal.deal(number_of_players);

    println!("\nDealing to {} players:", number_of_players);

    for i in 0..number_of_players {
        let seat_number = i;
        println!(
            "Seat {}: {}",
            seat_number,
            deal.hand_by_seat(seat_number).unwrap()
        );
    }

    deal.flop();
    deal.table.play_out_flop();
    deal.table.play_out_possible_hands_at_flop();

    deal.turn();
    deal.table.play_out_turn();

    deal.river();
    deal.table.play_out_river();

    println!("\nTo run the same hand again, type:");
    println!("{}", deal.table.format_calc());
}
