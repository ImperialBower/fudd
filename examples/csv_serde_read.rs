extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinaryCardMap {
    pub bc: u64,
    pub rank: u16,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = "logs/bc.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: BinaryCardMap = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
