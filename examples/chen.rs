use fudd::types::ranges::chen_weighted::ChenWeighted;

fn main() {
    let all = ChenWeighted::all().sort();
    for pair in all.iter() {
        println!("{}", pair);
    }
}
