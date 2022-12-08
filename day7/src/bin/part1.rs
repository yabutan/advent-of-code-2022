use std::fs::File;
use std::io::{BufRead, BufReader};

use day7::calc_size_of_directories;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day7/data/input.txt")?);

    let total_size = calc_total_size(r)?;
    println!("answer: {}", total_size);

    Ok(())
}

fn calc_total_size(r: impl BufRead) -> anyhow::Result<u64> {
    const THRESHOLD: u64 = 100000;

    let directories = calc_size_of_directories(r)?;

    let total_size = directories
        .iter()
        .filter(|d| d.size <= THRESHOLD)
        .map(|d| d.size)
        .sum();

    Ok(total_size)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_total_size() {
        let total_size = calc_total_size(include_str!("../../data/sample.txt").as_bytes()).unwrap();
        assert_eq!(total_size, 95437);
    }
}
