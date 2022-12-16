extern crate core;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day15/data/input.txt")?);
    //println!("answer: {}", ret);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
