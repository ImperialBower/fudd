use clap::Parser;
use std::fs;

use fudd::games::holdem::table::Table;
use fudd::types::card_slot::CardSlot;
use fudd::types::playing_card::PlayingCard;
use fudd::types::slots::hole_cards::HoleCards;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'h', long)]
    hero: String,

    #[clap(short = 'v', long)]
    villain: String,
    //
    // #[clap(short = 'b', long)]
    // board: String,
}

///
fn main() {
    env_logger::init();
    let args = Args::parse();

    let hero = HoleCards::from_index(Box::leak(args.hero.into_boxed_str()));
    let villain = HoleCards::from_index(Box::leak(args.villain.into_boxed_str()));
    // let board = Board::from_index(Box::leak(args.board.into_boxed_str())).unwrap();

    let table = Table::seat(2);
    table.take(PlayingCard::from(hero.get_first_card()));
    table.take(PlayingCard::from(villain.get_first_card()));
    table.take(PlayingCard::from(hero.get_second_card()));
    table.take(PlayingCard::from(villain.get_second_card()));

    // let s = table.play_out_deal_fmt();
    let filename = format!(
        "logs/{}-{}.txt",
        hero.simple_index_short(),
        villain.simple_index_short(),
    );
    // println!("{}", s.clone());
    fs::write(filename.clone(), filename).expect("Unable to write file");
}

// let other_hands = rockets.every_other().two_cards_vec();
//
// for hand in other_hands.iter() {
//     let table = Table::seat(2);
//     table.take(PlayingCard::from(rockets.first()));
//     table.take(PlayingCard::from(rockets.second()));
//     table.take(PlayingCard::from(hand.first()));
//     table.take(PlayingCard::from(hand.second()));
//     table.play_out_deal();
// }
