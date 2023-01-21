use std::fs::File;
use std::io::{BufRead, BufReader};

use day07::command::CommandParse;
use day07::directory::IntoDirectories;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day07/data/input.txt")?);

    let total_size = calc_size(r)?;
    println!("answer: {}", total_size);

    Ok(())
}

fn calc_size(mut r: impl BufRead) -> anyhow::Result<u64> {
    const THRESHOLD: u64 = 100000;

    let total_size = r
        .parse_commands()?
        .into_directories()
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
    fn test_calc_size() {
        let total_size = calc_size(include_str!("../../data/sample.txt").as_bytes()).unwrap();
        assert_eq!(total_size, 95437);
    }
}
