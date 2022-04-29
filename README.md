# FUDD 

[![LICENSE](https://img.shields.io/badge/license-GPL3.0-blue.svg)](LICENSE)
![Build Status](https://github.com/ContractBridge/fudd/actions/workflows/basic.yaml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/fudd.svg)](https://crates.io/crates/fudd)

🚧 **Work In Progress** 🚧

Rust poker library. Code inspired by [Cactus Kev's](https://suffe.cool) 
[work in C](https://suffe.cool/poker/code/). See [ckc-rs](https://github.com/ContractBridge/ckc-rs)
for the core hand evaluation library which is isolated with no-std for future
use in embedded and wasm libraries.

Currently only supports [hold'em](https://en.wikipedia.org/wiki/Texas_hold_%27em), 
but working on [Omaha](https://en.wikipedia.org/wiki/Omaha_hold_%27em) and want 
to add more types of games. Supporting things like 
[Razz](https://en.wikipedia.org/wiki/Razz_(poker)) would be a total kick.

## Examples

There are several examples of the library in the examples directory, including
one that runs through [every possible hand combination](examples/all_possible.rs):

The [calc example](examples/calc.rs) allows you to do a full analysis of a hand 
of poker. Here it is running 
[the famous hand](https://www.youtube.com/watch?v=vjM60lqRhPg) quads vs full 
house between Gus Hansen and Daniel Negreanu on High Stakes Poker:

```txt
❯ cargo run --example calc -- -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠ 8♠"
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/calc -d '6♠ 6♥ 5♦ 5♣' -b '9♣ 6♦ 5♥ 5♠ 8♠'`
Cards Dealt: 6♠ 6♥ 5♦ 5♣ 9♣ 6♦ 5♥ 5♠ 8♠

[Seat 0: 6♠ 6♥, Seat 1: 5♦ 5♣]
[FLOP:  9♣ 6♦ 5♥, TURN:  5♠, RIVER: 8♠]

The Flop: 9♣ 6♦ 5♥
Chances of winning:
Seat #0 6♠ 6♥: 95.7% - CURRENT HAND: 6♠ 6♥ 6♦ 9♣ 5♥ HandRank { value: 2185, name: ThreeOfAKind, class: ThreeSixes }
Seat #1 5♦ 5♣: 6.0% - CURRENT HAND: 5♥ 5♦ 5♣ 9♣ 6♦ HandRank { value: 2251, name: ThreeOfAKind, class: ThreeFives }

The Nuts would be: 9♣ 8♠ 7♠ 6♦ 5♥ HandRank { value: 1605, name: Straight, class: NineHighStraight }

The Turn: 5♠
Chances of winning:
Seat 0: 2.3% - Outs: 6♣
Seat 1: 97.7% - Best Hand: 5♠ 5♥ 5♦ 5♣ 9♣ HandRank { value: 124, name: FourOfAKind, class: FourFives }

The River: 8♠
Seat 0: 0.0%
Seat 1: 100.0%

Winners:
   Seat 1: 5♠ 5♥ 5♦ 5♣ 9♣ HandRank { value: 124, name: FourOfAKind, class: FourFives }

Command:
❯ cargo run --example calc -- -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠ 8♠"
```

Add the `-n` flag and it will add all possible hands at the flop, sorted by
strength:

```txt
Possible hands at the flop, sorted by strength:
CKC #1605 9♣ 8♠ 7♠ 6♦ 5♥ HandRank { value: 1605, name: Straight, class: NineHighStraight }
CKC #1996 9♠ 9♥ 9♣ 6♦ 5♥ HandRank { value: 1996, name: ThreeOfAKind, class: ThreeNines }
CKC #2185 6♠ 6♥ 6♦ 9♣ 5♥ HandRank { value: 2185, name: ThreeOfAKind, class: ThreeSixes }
CKC #2251 5♠ 5♥ 5♦ 9♣ 6♦ HandRank { value: 2251, name: ThreeOfAKind, class: ThreeFives }
CKC #3047 9♠ 9♣ 6♠ 6♦ 5♥ HandRank { value: 3047, name: TwoPair, class: NinesAndSixes }
CKC #3058 9♠ 9♣ 5♠ 5♥ 6♦ HandRank { value: 3058, name: TwoPair, class: NinesAndFives }
CKC #3221 6♠ 6♦ 5♠ 5♥ 9♣ HandRank { value: 3221, name: TwoPair, class: SixesAndFives }
CKC #3501 A♠ A♥ 9♣ 6♦ 5♥ HandRank { value: 3501, name: Pair, class: PairOfAces }
CKC #3721 K♠ K♥ 9♣ 6♦ 5♥ HandRank { value: 3721, name: Pair, class: PairOfKings }
CKC #3941 Q♠ Q♥ 9♣ 6♦ 5♥ HandRank { value: 3941, name: Pair, class: PairOfQueens }
CKC #4161 J♠ J♥ 9♣ 6♦ 5♥ HandRank { value: 4161, name: Pair, class: PairOfJacks }
CKC #4381 T♠ T♥ 9♣ 6♦ 5♥ HandRank { value: 4381, name: Pair, class: PairOfTens }
CKC #4471 9♠ 9♣ A♠ 6♦ 5♥ HandRank { value: 4471, name: Pair, class: PairOfNines }
CKC #4836 8♠ 8♥ 9♣ 6♦ 5♥ HandRank { value: 4836, name: Pair, class: PairOfEights }
CKC #5056 7♠ 7♥ 9♣ 6♦ 5♥ HandRank { value: 5056, name: Pair, class: PairOfSevens }
CKC #5122 6♠ 6♦ A♠ 9♣ 5♥ HandRank { value: 5122, name: Pair, class: PairOfSixes }
CKC #5342 5♠ 5♥ A♠ 9♣ 6♦ HandRank { value: 5342, name: Pair, class: PairOfFives }
CKC #5720 4♠ 4♥ 9♣ 6♦ 5♥ HandRank { value: 5720, name: Pair, class: PairOfFours }
CKC #5940 3♠ 3♥ 9♣ 6♦ 5♥ HandRank { value: 5940, name: Pair, class: PairOfTreys }
CKC #6160 2♠ 2♥ 9♣ 6♦ 5♥ HandRank { value: 6160, name: Pair, class: PairOfDeuces }
CKC #6305 A♠ K♠ 9♣ 6♦ 5♥ HandRank { value: 6305, name: HighCard, class: AceHigh }
CKC #6753 K♠ Q♠ 9♣ 6♦ 5♥ HandRank { value: 6753, name: HighCard, class: KingHigh }
CKC #7046 Q♠ J♠ 9♣ 6♦ 5♥ HandRank { value: 7046, name: HighCard, class: QueenHigh }
CKC #7227 J♠ T♠ 9♣ 6♦ 5♥ HandRank { value: 7227, name: HighCard, class: JackHigh }
CKC #7346 T♠ 9♣ 8♠ 6♦ 5♥ HandRank { value: 7346, name: HighCard, class: TenHigh }
CKC #7420 9♣ 8♠ 6♦ 5♥ 4♠ HandRank { value: 7420, name: HighCard, class: NineHigh }
```

Add the `-e` flag and it will give you the odds of winning at the draw. 
**NOTE** this takes a very, very long time (_improve me_) :

```txt
Seat #0 6♠ 6♥: 81.7%
Seat #1 5♦ 5♣: 20.1%
```

The library is very forgiving with the format of the strings passed in, thanks
to my [cardpack.rs](https://github.com/ContractBridge/cardpack.rs) library:

```txt
❯ cargo run --example calc -- -d "AC 4D Kh 6H Kd TH" -b "7C 3D AS 4C 9d"
```

## Other Resources

* [Cactus Kev's Poker Hand Evaluator](https://suffe.cool/poker/evaluator.html)
* Repositories
    * [vsupalov](https://github.com/vsupalov/)
        * [pokereval-rs](https://github.com/vsupalov/pokereval-rs)
        * [cards-rs](https://github.com/vsupalov/cards-rs)
        * [holdem-rs](https://github.com/vsupalov/holdem-rs)
        * [pokerlookup-rs](https://github.com/vsupalov/pokerlookup-rs)
        * [pokerhandrange-rs](https://github.com/vsupalov/pokerhandrange-rs) - [crate](https://crates.io/crates/pokerhandrange)
    * [HenryRLee/PokerHandEvaluator](https://github.com/HenryRLee/PokerHandEvaluator) - An efficient poker hand evaluation algorithm and its implementation, supporting 7-card poker and Omaha poker evaluation
    * [adchari/better-hand](https://github.com/adchari/better-hand) - [crate](https://crates.io/crates/better-hand)
    * [deus-x-mackina/poker](https://github.com/deus-x-mackina/poker) - [crate](https://crates.io/crates/poker)
        * Port of the [treys](https://github.com/ihendley/treys) Python library
    * [elliottneilclark/rs-poker](https://github.com/elliottneilclark/rs-poker) - [crate](https://crates.io/crates/rs_poker)
    * [kmurf1999/rust_poker](https://github.com/kmurf1999/rust_poker) - [crate](https://crates.io/crates/rust_poker) - [crate](https://crates.io/crates/rust_poker)
        * In part based on: [OMPEval](https://github.com/zekyll/OMPEval) (C++)
          * [SKPokerEval](https://github.com/kennethshackleton/SKPokerEval) (C++)
          * [TwoPlusTwo Hand Evaluator](https://github.com/tangentforks/TwoPlusTwoHandEvaluator)
            * [XPokerEval](https://github.com/tangentforks/XPokerEval) - Compilation from codingthewheel.com
          * [ACE_eval](https://github.com/ashelly/ACE_eval) (C)
    * [manuelbucher/distributed-cards](https://gitlab.com/manuelbucher/distributed-cards) - [crate](https://crates.io/crates/distributed-cards) Implements the mental poker shuffling algorithm
    * [lucasholder/fair](https://github.com/lucasholder/fair) - [crate](https://crates.io/crates/fair) CLI tool and library for verifying provably fair games (baccarat, etc.).
    * [davefol/Poker-Range-Trainer](https://github.com/davefol/Poker-Range-Trainer)
    * Heads Up
      * [Poker odds pre-flop heads-up](https://tools.timodenk.com/poker-odds-pre-flop)
        * [JS Source](https://github.com/Simsso/Online-Tools/blob/master/src/page/logic/poker-odds-pre-flop.js)
* Articles
  * [How to Compute the Probability of Equal-Rank Cards in Stud Poker](https://stattrek.com/poker/probability-of-equal-rank-cards.aspx)
  * [Interactive Mathematics Miscellany and Puzzles](https://www.cut-the-knot.org/) > [Probabilities](https://www.cut-the-knot.org/probability.shtml) > [Example: A Poker Hand](https://www.cut-the-knot.org/Probability/PokerSampleSpaces.shtml)
  * [Probabilities of Poker Hands with Variations](https://meteor.geol.iastate.edu/~jdduda/portfolio/492.pdf)
  * [Heads Up Poker Rules for Texas Hold’em](https://automaticpoker.com/poker-basics/heads-up-game-play-rules-for-texas-holdem/)
  * [7 Card Hand Evaluators](https://web.archive.org/web/20111101152023/http://archives1.twoplustwo.com/showflat.php?Cat=0&Number=8513906&page=0&fpart=1&vc=1) Epic thread
  * [Counting Outs](https://www.countingouts.com/)
  * [Roguelike Tutorial - In Rust](http://bfnightly.bracketproductions.com/rustbook/)
    * [Patreon](https://www.patreon.com/m/505827/posts)
  * Chen Formula
    * [Chen Formula](https://www.thepokerbank.com/strategy/basic/starting-hand-selection/chen-formula/)
    * [The Strategy Behind The Chen Formula in Poker](https://www.888poker.com/magazine/strategy/chen-formula#The%20Chen%20Formula%20%E2%80%93%20Usage)
* Commercial Tools
  * [ProPokerTools](http://www.propokertools.com/)
    * [PQL](http://www.propokertools.com/pql)
* [kaggel - Poker Hold'Em Games](https://www.kaggle.com/smeilz/poker-holdem-games?select=File198.txt)
* [Poker Hand Data Set](https://archive.ics.uci.edu/ml/datasets/Poker+Hand)
* [Download all 10,000 hands that Pluribus (poker AI) played against pros](https://www.reddit.com/r/poker/comments/cdhasb/download_all_10000_hands_that_pluribus_poker_ai/)
  * [Superhuman AI for multiplayer poker](https://www.science.org/doi/10.1126/science.aay2400) 

## Dependencies

* [cardpack.rs](https://github.com/ContractBridge/cardpack.rs)
* [ckc-rs](https://github.com/ContractBridge/ckc-rs)
* [crossbeam](https://github.com/crossbeam-rs/crossbeam)
* [rust-csv](https://github.com/BurntSushi/rust-csv)
  * [Tutorial](https://docs.rs/csv/latest/csv/tutorial/index.html)
* [indexmap](https://github.com/bluss/indexmap)
* [rayon](https://github.com/rayon-rs/rayon)
  * [Rayon: data parallelism in Rust](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)
  * [Rust Cookbook > Concurrency > Parallel Tasks](https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html)
  * [How Rust makes Rayon's data parallelism magical](https://developers.redhat.com/blog/2021/04/30/how-rust-makes-rayons-data-parallelism-magical)
  * [Parallel stream processing with Rayon](https://morestina.net/blog/1432/parallel-stream-processing-with-rayon)
* [Serde](https://serde.rs/)
  * [serde_test](https://crates.io/crates/serde_test)
  * [Understanding Rust's serde using macro expansion](https://owengage.com/writing/2021-07-23-serde-expand/)
  * [Exploring serde's data model with a toy deserializer](https://owengage.com/writing/2021-08-14-serde-toy/)

## Example Dependencies

* [Clap](https://github.com/clap-rs/clap) - Used for the `calc` sample application.
* [env_logger](https://github.com/env-logger-rs/env_logger/)

## Resources

* [Command line apps in Rust Book](https://rust-cli.github.io/book/)
* [Rust and TUI: Building a command-line interface in Rust](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/)
