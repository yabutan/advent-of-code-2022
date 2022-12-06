use std::fs::File;
use std::io::BufReader;

use day5::{simulate, CraneMover9001};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day5/data/input.txt")?);
    let ret = simulate(r, CraneMover9001);
    println!("answer: {}", ret);
    Ok(())
}
