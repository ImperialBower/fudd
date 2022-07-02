use csv::Reader;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct BCM {
    pub bc: u64,
    pub rank: u16,
}

lazy_static! {
    static ref BC_RANK: HashMap<u64, u16> = {
        let mut m = HashMap::new();
        let file_path = "logs/bc.csv";
        let file = File::open(file_path).unwrap();
        let mut rdr = Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BCM = result.unwrap();
            m.insert(record.bc, record.rank);
        }
        m
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    println!(
        "The entry for `39586715926528` is \"{}\".",
        BC_RANK.get(&39586715926528u64).unwrap()
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
