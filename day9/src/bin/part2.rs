use std::fs::File;
use std::io::BufReader;

use day9::{simulate_tail, Rope};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day9/data/input.txt")?);

    let mut rope = Rope::new(10);
    let ret = simulate_tail(r, &mut rope);
    println!("answer: {}", ret.len());

    Ok(())
}
