use std::io::BufRead;
use std::ops::RangeInclusive;

pub fn count_fully_contains(r: impl BufRead) -> u32 {
    do_count(r, fully_contains)
}

pub fn count_overlap_contains(r: impl BufRead) -> u32 {
    do_count(r, overlap_contains)
}

fn parse_range(line: &str) -> anyhow::Result<RangeInclusive<u32>> {
    let mut iter = line.split('-');
    let (Some(start), Some(end)) = (iter.next(), iter.next()) else {
        return Err(anyhow::anyhow!("parse range error"));
    };

    let start = start.parse::<u32>()?;
    let end = end.parse::<u32>()?;
    Ok(start..=end)
}

fn parse_line(line: &str) -> anyhow::Result<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut iter = line.split(',');
    let (Some(first), Some(second)) = (iter.next(), iter.next()) else {
        return Err(anyhow::anyhow!("parse line error"));
    };

    let first = parse_range(first)?;
    let second = parse_range(second)?;

    Ok((first, second))
}

fn do_count(r: impl BufRead, f: fn(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool) -> u32 {
    let mut count = 0;
    for line in r.lines() {
        let line = line.unwrap();
        let (first, second) = parse_line(&line).expect("parse line error");

        if f(first, second) {
            println!("{} is fully contains", line);
            count += 1;
        }
    }
    count
}

fn fully_contains(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    if a.contains(b.start()) && a.contains(b.end()) {
        return true;
    }
    if b.contains(a.start()) && b.contains(a.end()) {
        return true;
    }
    false
}

fn overlap_contains(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    if a.contains(b.start()) || a.contains(b.end()) {
        return true;
    }
    if b.contains(a.start()) || b.contains(a.end()) {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fully_contains() {
        assert!(!fully_contains(2..=4, 6..=8));
        assert!(!fully_contains(2..=3, 4..=5));
        assert!(!fully_contains(5..=7, 7..=9));

        assert!(fully_contains(2..=8, 3..=7));
        assert!(fully_contains(6..=6, 4..=6));

        assert!(!fully_contains(2..=6, 4..=8));
    }

    #[test]
    fn test_overlap_contains() {
        assert!(!overlap_contains(2..=4, 6..=8));
        assert!(!overlap_contains(2..=3, 4..=5));

        assert!(overlap_contains(5..=7, 7..=9));
        assert!(overlap_contains(2..=8, 3..=7));
        assert!(overlap_contains(6..=6, 4..=6));
        assert!(overlap_contains(2..=6, 4..=8));
    }

    #[test]
    fn test_parse_line() {
        let (first, second) = parse_line("2-4,6-8").unwrap();
        assert_eq!(first, 2..=4);
        assert_eq!(second, 6..=8);
    }

    #[test]
    fn test_count() {
        let text = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
        assert_eq!(count_fully_contains(text.trim().as_bytes()), 2);
        assert_eq!(count_overlap_contains(text.trim().as_bytes()), 4);
    }
}
