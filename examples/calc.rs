use clap::Parser;
use fudd::games::holdem::board::Board;
use fudd::games::holdem::seats::Seats;
use fudd::games::holdem::table::Table;
use std::time::Instant;

extern crate log;

/// Demo Holdem poker hand evaluator app.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long)]
    dealt: String,

    #[clap(short = 'b', long)]
    board: String,

    #[clap(short = 'n', long)]
    nuts: bool,

    #[clap(short = 'e', long)]
    eval_deal: bool,
}

/// `cargo run --example calc -- -d "4♥ 3♥ A♦ J♣ 8♦ 8♣" -b "T♥ J♠ J♥ 3♦"`
///
/// Interesting hands:
/// cargo run --example calc -- -d "3♥ A♠ 5♥ A♦ 8♦ 7♦ K♥ K♠ 2♥ Q♠" -b "6♦ 6♣ 7♣ 9♦ 5♦" - Straight Flush at the river
/// cargo run --example calc -- -d "3♠ 9♦ J♠ 8♦ 2♠ Q♠ 6♣ 4♠" -b "Q♥ 5♥ 5♣ 7♥ 4♥" -- Two Pair vs Straight Draw
///
/// cargo run --example calc -- -d "K♠ Q♠ 5♦ K♥ 5♥ J♥" -b "J♦ T♣ A♥ K♣ 2♣" -n -- Flopping the nuts
/// cargo run --example calc -- -d "A♣ Q♠ T♦ T♣ 6♦ 4♦ 2♥ 2♦" -b "J♦ J♠ J♥ A♥ 3♦" HSP S04E08 Harman/Safai
/// cargo run --example calc -- -d "T♦ 2♦ 9♠ 6♥" -b "3♠ 8♦ A♦" HSP S04E08 Elezra/Negreanu
/// cargo run --example calc -- -d "A♣ 4♠ K♥ 6♥ K♦ T♥" -b "7♠ 3♦ A♠ 4♦" HSP S04E08 Farha/Harman/Safai
/// cargo run --example calc -- -d "6♠ 6♦ A♣ Q♠ A♥ 9♥ Q♦ 5♠" -b "9♦ T♦ 6♥ T♥ K♠" HSP S04E08 Harman/Elezra
/// cargo run --example calc -- -d "T♠ 9♣ J♦ J♣ Q♥ T♣" -b "T♥ 7♣ A♥ J♠ 8♦" HSP S04E08 Harman/Elezra/Farha
/// cargo run --example calc -- -d "A♦ 7♦ T♠ T♥ K♦ K♥" -b "7♠ 6♥ 4♣" HSP S01E01 Negreanu/Buss/Nasseri
/// cargo run --example calc -- -d "A♠ J♦ 6♥ 6♣" -b "A♥ 3♠ 6♠ J♠ 5♠" HSP S01E01 Negreanu/Greenstein
/// cargo run --example calc -- -d "7♣ 6♥ K♣ 2♣ J♦ 9♦" -b "Q♣ 7♥ K♥ 6♣ Q♠" HSP S01E01 Alaei/Negreanu/Harman
/// cargo run --example calc -- -d "A♠ K♠ A♣ K♥" -b "4♠ 7♠ K♣" HSP S04E09 Hellmuth/Gold
/// cargo run --example calc -- -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠ 8♠" HSP THE hand Negreanu/Hansen
/// cargo run --example calc -- -d "6♠ 4♠ 8♣ 6♣ A♦ 2♦ K♥ J♣" -b "2♣ 3♦ 3♣ 4♦ 4♣" HSP S06E10 Grospellier/Benyamine
/// cargo run --example calc -- -d "A♠ K♥ 9♦ 8♥" -b "6♦ 7♥ T♣ 3♥ 5♥" HSP S06E11 Galfond/Negreanu
/// cargo run --example calc -- -d "7♠ 6♠ Q♠ Q♦" -b "2♠ 7♥ 9♠ T♦ 4♣" HSP S08E07 Bellande Schwimer FIRST RUN
/// cargo run --example calc -- -d "7♠ 6♠ Q♠ Q♦" -b "2♠ 7♥ 9♠ A♠ K♠" HSP S08E07 Bellande Schwimer SECOND RUN
/// cargo run --example calc -- -d "T♦ 9♦ 2♠ 2♥" -b "2♦ T♥ 7♦ 8♦ 6♥" DNEGS https://youtu.be/yyPU25EGLkA?t=123
/// cargo run --example calc -- -d "A♦ Q♠ K♣ Q♦" -b "J♥ 9♠ A♣ 4♦ T♣" HSP S09E03 DNEGS/Bellands
/// cargo run --example calc -- -d "J♥ 8♠ K♠ J♠ 3♠ 3♥" -b "7♥ 8♦ 2♣ 5♣ Q♠" HSP S09E04 Adelstein/Liu/Antonius
/// cargo run --example calc -- -d "A♥ 8♦ K♣ 7♣ T♥ T♦" -b "4♠ K♦ 2♦ J♥ 3♠" HSP S09E05 Brunson/Tilly/Antonius
/// cargo run --example calc -- -d "J♥ J♣ A♥ 4♥" -b "3♣ 4♠ 4♣ 7♣ A♣" HSP S09E05 Adelstein/Brunson 1st
/// cargo run --example calc -- -d "J♥ J♣ A♥ 4♥" -b "3♣ 4♠ 4♣ 7♣ 9♠" HSP S09E05 Adelstein/Brunson 2nd
/// cargo run --example calc -- -d "8♦ 5♦ K♦ J♥ 2♠ 2♥" -b "9♥ 2♦ K♥ 4♥ J♠" HSP S09E05 Tilly/Hultman
/// cargo run --example calc -- -d "J♥ J♦ A♠ K♦ T♣ 9♣" -b "7♦ K♠ 2♥ 7♣ A♦" HSP S09E05 Liu/Tilly/Menon
/// cargo run --example calc -- -d "5♠ 5♦ 9♠ 9♥ K♣ T♦" -b "5♣ 9♦ T♥ T♣ Q♦" HSP S09E13 Antonius, Negreanu, Ivey
///     https://www.pokernews.com/news/2022/05/phil-ivey-negreanu-high-stakes-poker-41207.htm
///
///
/// cargo run --example calc -- -d "K♣ T♦ 5♠ 5♦ 9♠ 9♥" -b "5♣ 9♦ T♥ T♣ Q♦" HSP S09E13
fn main() {
    let start = Instant::now();
    env_logger::init();
    let args = Args::parse();

    let mut table = Table::default();
    let seats = Seats::from_index(Box::leak(args.dealt.into_boxed_str()));
    let board = Board::from_index(Box::leak(args.board.into_boxed_str()));

    match seats {
        Ok(s) => table.players = s,
        Err(e) => println!("{:?}", e),
    }

    match board {
        Ok(b) => table.board = b,
        Err(e) => println!("{:?}", e),
    }

    if args.eval_deal {
        table.play_out_deal();
    }

    if args.nuts {
        table.play_out_detailed();
    } else {
        table.play_out();
    }

    let duration = start.elapsed();

    println!("Time taken performing calc: {:?}", duration);

    // Print a prettified version of the command back out.
    println!("\nCommand:");
    println!("{}", table.format_calc());
}
