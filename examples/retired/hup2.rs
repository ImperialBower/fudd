use fudd::games::holdem::heads_up::HeadsUp;

fn main() {
    alt();
}

#[allow(dead_code)]
fn the_hand() {
    // First: 79.73% (1365284)
    // Second: 18.39% (314904)
    // Ties: 1.88% (32116)
    // Elapsed: 1126.93s - linux
    do_it("6♠ 6♥ 5♦ 5♣");
    println!();

    // Covered
    //
    // First: 80.39% (1376436) diff: 11152
    // Second: 17.67% (302502)
    // Ties: 1.95% (33366)
    // Elapsed: 1059.40s
    do_it("6♠ 6♥ 5♠ 5♣");

    // Smothered
    // First: 81.04% (1387588)
    // Second: 16.94% (290100)
    // Ties: 2.02% (34616)
    // Elapsed: 1062.14s
    println!();
    do_it("6♠ 6♥ 5♠ 5♥");
}

fn alt() {
    do_it("A♠ A♥ 7♦ 7♣"); // Naked
    do_it("A♠ A♥ 7♠ 7♣"); // Covered
    do_it("A♠ A♥ 7♠ 7♥"); // Smothered
}

fn do_it(cards: &'static str) {
    let now = std::time::Instant::now();

    let hup = HeadsUp::from(cards);

    let odds = hup.odds_preflop();

    println!("{}", hup.odds_to_string(odds).to_string());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
