use std::fs::File;
use std::io::BufReader;

use day05::{simulate, CraneMover9000};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day05/data/input.txt")?);
    let ret = simulate(r, CraneMover9000);
    println!("answer: {}", ret);
    Ok(())
}
