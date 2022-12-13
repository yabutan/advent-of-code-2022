use std::fs::File;
use std::io::BufReader;

use day12::simulate;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day12/data/input.txt")?);
    let path = simulate(r, true).expect("Failed to simulate");
    println!("answer: {}", path.len() - 1);
    Ok(())
}
