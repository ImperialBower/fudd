use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Msb0};
use cardpack::Standard52;
use fudd::types::bitvec::bit_card::{AnnotatedBitCard, BitCard};
use fudd::types::playing_card::PlayingCard;

/// Demonstration of an experiment with using the [bitvec](https://github.com/bitvecto-rs/bitvec)
/// library to define poker cards. The code for the module is now primarily used for come type
/// conversions and to validate other methods in tests.
fn main() {
    let king_spades: BitCard = BitCard::try_from("KS").unwrap();

    let r = king_spades.as_bitarray().load::<u64>();

    let bv: BitVec<Msb0, u64> = king_spades.as_bitarray().iter().collect();

    println!(">> {}", bv);
    let ks = Standard52::card_from_index("KS");

    let bvv = bv.load::<u64>();
    println!(">> {}", bvv);

    println!("{}", king_spades);
    println!("{}", r);
    println!("{}", PlayingCard::from(&ks));

    let asd = king_spades.get_rank_bitslice().load::<u64>();
    println!(">>> {}", asd);

    println!("{:#}", AnnotatedBitCard::new(king_spades));

    let standard52 = Standard52::default();
    for card in standard52.deck {
        let bs: BitCard = BitCard::from(&card);
        let bss = bs.as_bitarray().load::<u64>();
        let rbs = bs.get_rank_bitslice().load::<u64>();
        println!("{} {} {}", card, rbs, bss);
    }

    // let standard52 = Standard52::default();
    // for card in standard52.deck {
    //     card.debug();
    // }
}
