use std::fs::File;
use std::io::{BufRead, BufReader};

use day06::find_packet_marker;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day06/data/input.txt")?);
    let text = r.lines().next().unwrap()?;

    let ret = find_packet_marker(&text);
    println!("answer: {}", ret);

    Ok(())
}
