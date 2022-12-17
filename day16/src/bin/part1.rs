extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day16/data/input.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {}
