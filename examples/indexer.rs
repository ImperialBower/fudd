use clap::Parser;
use fudd::games::holdem::table::Table;

extern crate log;

/// Demo Holdem poker hand evaluator app.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'i', long, default_value = "")]
    index: String,

    #[clap(short = 'n', long, default_value = "0")]
    number: usize,
}

/// Demo hand calculator app that works against a single hold'em poker card index string
/// that combines cards dealt to players and community cards.
///
/// This application has been improved with the `calc.rs` example that separates
/// player cards and community cards.
///
/// # Usage:
///
/// ```
/// cargo run --example indexer -- --index "K♠ Q♠ 5♦ K♥ 5♥ J♥ J♦ T♣ A♥ K♣ 2♣"
/// ```
///
///
/// **NOTE** There need to be at least 9 cards passed into the index and there
/// need to be a random number of cards. The last five are used for the board,
/// and each pair in the front is used for a player.
///
/// Call it without an index and it will print out a randomly generated example:
///
/// ```
/// cargo run --example calc
/// ```
/// Some interesting hand examples:
/// K♠ Q♠ 5♦ K♥ 5♥ J♥ J♦ T♣ A♥ K♣ 2♣ -- flop the nuts
///      K♠ Q♠ 5♦ K♥ 5♥ J♥ J♦ T♣ A♥ Q♣ K♣ -- 3 way tie
///      3♠ Q♠ 5♦ K♥ 5♥ J♥ J♦ T♣ A♥ Q♣ 2♣ -- Player one instead
/// 6♠ 8♣ 8♦ 6♥ 8♠ 7♠ T♣ K♣ 9♦ T♦ 5♣ -- three way tie
/// 9♦ 6♣ 9♣ 8♥ Q♥ 3♠ 2♠ 3♣ 5♦ 4♥ 6♦ -- three way tie
/// K♣ A♥ K♥ A♦ 4♣ 5♥ T♣ J♦ A♠ 9♠ 7♥ -- two way tie
/// A♥ Q♥ A♣ Q♦ 9♦ 4♥ 8♥ 5♥ 3♦ - High Stakes Poker S04E06 Helmuth/Farha
/// 4♥ 3♥ A♦ J♣ 8♦ 8♣ T♥ J♠ J♥ 3♦ 2♣ - High Stakes Poker S04E06 Farha/Gold/Negreanu
/// K♥ Q♥ T♠ 7♠ T♦ 7♣ K♦ J♠ 9♠ -- High Stakes Poker S04E06 Harmon/Negreanu FIRST RUN
/// K♥ Q♥ T♠ 7♠ T♦ 7♣ K♦ A♠ Q♣ -- High Stakes Poker S04E06 Harmon/Negreanu SECOND RUN
fn main() {
    env_logger::init();
    let args = Args::parse();

    let index = Box::leak(args.index.into_boxed_str());

    if index.is_empty() {
        if args.number < 2 {
            Table::sample().play_out();
        } else {
            Table::sample_number(args.number).play_out();
        }
    } else {
        let table_option = Table::from_index(index);

        match table_option {
            Ok(table) => table.play_out(),
            Err(e) => println!("{:?}", e),
        }
    }
}
