use ckc_rs::{CKCNumber, CardNumber, PokerCard};

fn main() {
    let card: CKCNumber = <CKCNumber as PokerCard>::filter(CardNumber::ACE_SPADES);

    assert!(!card.is_blank());
    println!("{}", card.as_u32());
}
