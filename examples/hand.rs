use clap::Parser;
use fudd::analysis::eval::Eval;
use fudd::types::arrays::five_cards::FiveCards;
use fudd::util::str_from_string;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'h', long)]
    hand: String,
}

/// Evaluates a single hand, printing out it's `HandRank`.
///
/// To evaluate the [Dead man's hand](https://en.wikipedia.org/wiki/Dead_man's_hand):
///
/// `❯ cargo run --example hand -- -h "8♠ A♠ 5♦ A♣ 8♣"`
fn main() {
    env_logger::init();
    let args = Args::parse();

    let s = str_from_string(args.hand);
    let hand = FiveCards::try_from(s);
    match hand {
        Ok(h) => println!("{}", Eval::from(h)),
        Err(e) => println!("{:?}", e),
    }
}
