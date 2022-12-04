use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
        _ => panic!("invalid input {}", c),
    }
}

fn split_half(line: &str) -> (&str, &str) {
    if line.len() % 2 != 0 {
        panic!("invalid input '{}'", line);
    }

    let left = &line[0..line.len() / 2];
    let right = &line[line.len() / 2..];
    (left, right)
}

fn find_duplicated_char(left: &str, right: &str) -> char {
    let left: HashSet<char> = HashSet::from_iter(left.chars());
    let right: HashSet<char> = HashSet::from_iter(right.chars());

    let duplicated: Vec<char> = left.intersection(&right).cloned().collect();
    assert_eq!(duplicated.len(), 1);

    duplicated[0]
}

fn calc_priority(r: impl BufRead) -> u32 {
    let mut total = 0;
    for line in r.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (left, right) = split_half(&line);
        let duplicated = find_duplicated_char(left, right);
        let priority = get_priority(duplicated);

        println!("{} {}", duplicated, priority);
        total += priority;
    }
    total
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day3/data/input.txt")?);
    let total = calc_priority(r);
    println!("answer: {}", total);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_split_half() {
        assert_eq!(split_half("abcdef"), ("abc", "def"));
        assert_eq!(split_half("af"), ("a", "f"));
    }

    #[test]
    fn test_find_duplicated_char() {
        let (left, right) = split_half("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(find_duplicated_char(left, right), 'p');

        let (left, right) = split_half("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(find_duplicated_char(left, right), 'L');

        let (left, right) = split_half("PmmdzqPrVvPwwTWBwg");
        assert_eq!(find_duplicated_char(left, right), 'P');
    }

    #[test]
    fn test_calc_priority() {
        let text = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        let total = calc_priority(text.as_bytes());
        assert_eq!(total, 157);
    }
}
