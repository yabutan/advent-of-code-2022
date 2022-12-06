use std::fs::File;
use std::io::BufReader;

use day5::{simulate, CraneMover9000};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day5/data/input.txt")?);
    let ret = simulate(r, CraneMover9000);
    println!("answer: {}", ret);
    Ok(())
}
