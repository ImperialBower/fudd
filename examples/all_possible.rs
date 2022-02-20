extern crate log;

use ckc_rs::hand_rank::{HandRankClass, HandRankName};
use fudd::analysis::Evaluate;
use strum::IntoEnumIterator;
use thousands::Separable;

/// Prints out the number of possible distinct combinations of hands, hand ranks,
/// and hand class combinations in a standard 52 Card Poker deck.
///
/// This example supports logging out each distinct possible poker hand, although it will
/// take a while:
///
/// ```
/// ❯ RUST_LOG=debug cargo run --example all_possible
/// ```
fn main() {
    env_logger::init();

    let (classes, names, ranks) = Evaluate::all_possible_combos();

    let possible_hands = Evaluate::count_possible_hands(&classes);

    println!(
        "There are {} possible hand combinations.",
        possible_hands.separate_with_commas()
    );
    println!(
        "There are {} different hand ranks.\n",
        ranks.len().separate_with_commas()
    );

    for v in HandRankName::iter() {
        let n = names.get(&v);
        match n {
            Some(name) => println!("{} possible {:?} hand ranks.", name, v),
            None => (),
        }
    }

    println!();

    let mut total_percentage = 0.0;

    for v in HandRankClass::iter() {
        let c = classes.get(&v);

        match c {
            Some(class) => {
                let pec = Evaluate::percent(*class, possible_hands);
                total_percentage += pec;
                println!("{} possible {:?} combinations. ({:.5}%)", class, v, pec)
            }
            None => (),
        }
    }
    println!("\n{:.2}% of possible hands calculated.", total_percentage)
}
