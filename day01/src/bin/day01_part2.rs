use std::fs;
use std::io::BufReader;

use anyhow::Result;
use day01::read_sum_list;
use itertools::Itertools;

fn main() -> Result<()> {
    let r = BufReader::new(fs::File::open("./day01/data/input.txt")?);
    let list = read_sum_list(r)?;

    let sum_of_top_three: u32 = list.iter().sorted().rev().take(3).sum();
    println!("answer: {}", sum_of_top_three);

    Ok(())
}
