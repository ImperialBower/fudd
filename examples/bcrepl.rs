use ckc_rs::cards::binary_card::{BinaryCard, BC64};
use ckc_rs::cards::five::Five;
use ckc_rs::cards::seven::Seven;
use ckc_rs::cards::two::Two;
use ckc_rs::cards::HandValidator;
// use ckc_rs::Shifty;
use fudd::analysis::store::holdem::bcm::BC_RANK;
use fudd::types::arrays::five_card::FiveCard;
use fudd::types::arrays::two_card::TwoCard;
use fudd::types::arrays::Vectorable;
use fudd::types::playing_cards::PlayingCards;
use fudd::types::poker_cards::PokerCards;
use std::io;
use std::io::Write;
use wincounter::{Win, Wins};

/// cargo run --example bcrepl
fn main() {
    loop {
        read_input();
    }
}

fn read_input() {
    print!("hole cards> ");
    let _ = io::stdout().flush();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to receive value");

    let cards = PokerCards::try_from(input_text);

    match cards {
        Ok(c) => {
            if c.len() != 4 {
                println!("Enter 4 cards");
            } else {
                work(c);
            }
        }
        Err(_) => println!("Invalid Cards"),
    }
}

fn work(cards: PokerCards) {
    let hands = cards.try_into_twos().unwrap();
    let hero = hands.get(0).unwrap();
    let villain = hands.get(1).unwrap();
    let hero = hero.sort();
    let villain = villain.sort();

    let now = std::time::Instant::now();

    push(hero, villain, cards.clone());

    // let hero = hero.shift_suit();
    // let villain = villain.shift_suit();
    // push(hero, villain, cards.clone());
    //
    // let hero = hero.shift_suit();
    // let villain = villain.shift_suit();
    // push(hero, villain, cards.clone());
    //
    // let hero = hero.shift_suit();
    // let villain = villain.shift_suit();
    // push(hero, villain, cards.clone());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn push(hero: Two, villain: Two, cards: PokerCards) {
    let wins = grind(hero, villain, cards.remaining());
    let results = wins.results_heads_up();
    println!(
        "{} {}, {}",
        TwoCard::from(hero),
        TwoCard::from(villain),
        results
    );
}

fn grind(hero: Two, villain: Two, remaining: PlayingCards) -> Wins {
    let mut wins = Wins::default();
    let combos = remaining.combinations(5);

    for combo in combos {
        let board = FiveCard::from(PlayingCards::from(combo).to_five_array().unwrap());
        let five = Five::from(board.to_arr());

        let hero7 = BinaryCard::from_seven(Seven::new(hero, five));
        let villain7 = BinaryCard::from_seven(Seven::new(villain, five));

        let hero_rank = BC_RANK.get(&hero7).unwrap();
        let villain_rank = BC_RANK.get(&villain7).unwrap();

        if hero_rank.rank < villain_rank.rank {
            wins.add_win(Win::FIRST);
        } else if villain_rank.rank < hero_rank.rank {
            wins.add_win(Win::SECOND);
        } else {
            wins.add_win(Win::FIRST | Win::SECOND);
        }
    }

    wins
}
