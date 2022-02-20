use fudd::types::arrays::Evaluable;
use fudd::util::uci::UCI;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/UCI/poker-hand-testing.data") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let (five, name) = UCI::parse_line(ip).unwrap();
                println!("{} - {:?}", five.eval(), name);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
