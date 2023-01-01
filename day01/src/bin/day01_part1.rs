use std::fs;
use std::io::BufReader;

use anyhow::Result;
use day01::read_sum_list;

fn main() -> Result<()> {
    let r = BufReader::new(fs::File::open("./day01/data/input.txt")?);
    let list = read_sum_list(r)?;
    let max = *list.iter().max().expect("No max found");

    println!("answer: {}", max);
    Ok(())
}
