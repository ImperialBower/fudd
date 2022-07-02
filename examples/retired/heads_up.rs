use fudd::games::holdem::heads_up::HeadsUp;
use fudd::types::arrays::Vectorable;
use fudd::types::playing_cards::PlayingCards;

fn main() {
    // "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠ 8♠" HSP THE hand Negreanu/Hansen

    let headsup = HeadsUp::from("6♠ 6♥ 5♦ 5♣");
    let board = PlayingCards::try_from("9♣ 6♦ 5♥ 5♠ 8♠").unwrap();

    let (eval1, eval2) = headsup.best_from_seven(&board);

    println!("Best Hand #1:{}", eval1);
    println!("Best Hand #2:{}", eval2);
    println!("========================");
    println!("========================");

    for v in headsup.remaining().combinations(5) {
        let board = PlayingCards::from(v);
        // let (first_seven, second_seven) = hands.best_from_seven(PlayingCards::from(v));
        let (eval1, eval2) = headsup.best_from_seven(&board);
        println!("Board:       {}", board);
        println!("Best Hand #1:{}", eval1);
        println!("Best Hand #2:{}", eval2);
        println!("========================");
    }
}
