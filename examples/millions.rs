use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    for x in 0..3_000_000 {
        hm.insert(x, x + 1 as u64);
    }

    println!("{:?}", hm);
}
