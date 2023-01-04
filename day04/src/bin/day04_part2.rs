use std::fs::File;
use std::io::BufReader;
use std::ops::RangeInclusive;

use day04::read_list;

fn overlap_contains(pair: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    let (a, b) = pair;
    if a.contains(b.start()) || a.contains(b.end()) {
        return true;
    }
    if b.contains(a.start()) || b.contains(a.end()) {
        return true;
    }
    false
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day04/data/input.txt")?);

    let count = read_list(r)?.into_iter().filter(overlap_contains).count();
    println!("answer: {}", count);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlap_contains() {
        assert!(!overlap_contains(&(2..=4, 6..=8)));
        assert!(!overlap_contains(&(2..=3, 4..=5)));

        assert!(overlap_contains(&(5..=7, 7..=9)));
        assert!(overlap_contains(&(2..=8, 3..=7)));
        assert!(overlap_contains(&(6..=6, 4..=6)));
        assert!(overlap_contains(&(2..=6, 4..=8)));
    }

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        assert_eq!(
            read_list(r)
                .unwrap()
                .into_iter()
                .filter(overlap_contains)
                .count(),
            4
        );
    }
}
