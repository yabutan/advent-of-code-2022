extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        ((c as u8) - b'a' + 1) as u32
    } else if c.is_ascii_uppercase() {
        ((c as u8) - b'A' + 27) as u32
    } else {
        panic!("invalid input {}", c);
    }
}

fn find_duplicated_char(first: &str, second: &str, third: &str) -> char {
    // to_set
    let first: HashSet<char> = HashSet::from_iter(first.chars());
    let second: HashSet<char> = HashSet::from_iter(second.chars());
    let third: HashSet<char> = HashSet::from_iter(third.chars());

    // grouping and get duplicated char
    let duplicated: HashSet<char> = first
        .iter()
        .chain(second.iter())
        .chain(third.iter())
        .counts()
        .iter()
        .filter(|(_, &v)| v == 3)
        .map(|(&k, _)| *k)
        .collect();

    assert_eq!(duplicated.len(), 1);

    *duplicated.iter().next().unwrap()
}

fn calc_priority(r: impl BufRead) -> u32 {
    let mut total = 0;

    // 3 lines chunk for loop
    for mut chunk in &r.lines().chunks(3) {
        let first = chunk.next().expect("missing first line").unwrap();
        let second = chunk.next().expect("missing second line").unwrap();
        let third = chunk.next().expect("missing third line").unwrap();

        let duplicated = find_duplicated_char(&first, &second, &third);
        let priority = get_priority(duplicated);
        total += priority;

        println!(
            "first:{} second:{} third:{} ==> {}({})",
            first, second, third, duplicated, priority
        );
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
    fn test_find_duplicated_char() {
        assert_eq!(
            find_duplicated_char(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            'r'
        );

        assert_eq!(
            find_duplicated_char(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            'Z'
        );
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
        assert_eq!(total, 70);
    }
}
