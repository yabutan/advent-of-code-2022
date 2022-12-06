use std::fs::File;
use std::io::{BufRead, BufReader};

use day6::find_packet_marker;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day6/data/input.txt")?);
    let text = r.lines().next().unwrap()?;

    let ret = find_packet_marker(&text);
    println!("answer: {}", ret);

    Ok(())
}
