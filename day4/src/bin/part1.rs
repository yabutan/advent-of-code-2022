use day4::count_fully_contains;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day4/data/input.txt")?);
    let count = count_fully_contains(r);
    println!("answer: {}", count);
    Ok(())
}
