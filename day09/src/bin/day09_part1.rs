use std::fs::File;
use std::io::BufReader;

use day09::{simulate_tail, Rope};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day09/data/input.txt")?);

    let mut rope = Rope::new(2);
    let tails = simulate_tail(r, &mut rope);
    println!("answer: {}", tails.len());

    Ok(())
}
