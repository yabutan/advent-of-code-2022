use day04::count_overlap_contains;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day04/data/input.txt")?);
    let count = count_overlap_contains(r);
    println!("answer: {}", count);
    Ok(())
}
