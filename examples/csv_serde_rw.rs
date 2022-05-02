extern crate csv;

use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinaryCardMap {
    pub bc: u64,
    pub rank: u16,
}

fn get_reader() -> Result<Reader<File>, Box<dyn Error>> {
    let file_path = "logs/bc.csv";
    let file = File::open(file_path)?;
    Ok(csv::Reader::from_reader(file))
}

fn get_writer() -> Result<LineWriter<File>, Box<dyn Error>> {
    let file = File::create("logs/bcrank.rs")?;
    Ok(LineWriter::new(file))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader()?;
    let mut file = get_writer()?;

    // create code prelude
    file.write_all(b"#[macro_use]\n")?;
    file.write_all(b"extern crate lazy_static;\n")?;
    file.write_all(b"use std::collections::HashMap;\n\n")?;
    file.write_all(b"lazy_static! {\n")?;
    file.write_all(b"\tstatic ref BC_RANK: HashMap<u64, u16 = {\n")?;
    file.write_all(b"\t\tlet mut m = HashMap::new();\n")?;

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: BinaryCardMap = result?;
        let r = format!("\t\tm.insert({}, {});\n", record.bc, record.rank);
        file.write_all(r.as_bytes())?;
        // println!("{:?}", record);
    }
    file.flush()?;
    Ok(())
}
