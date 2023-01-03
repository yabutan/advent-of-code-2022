use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use day03::Priority;

fn find_duplicate_item_of_group(first: &str, second: &str, third: &str) -> Result<char, String> {
    // ハッシュセットにしてユニーク化
    let first: HashSet<char> = HashSet::from_iter(first.chars());
    let second: HashSet<char> = HashSet::from_iter(second.chars());
    let third: HashSet<char> = HashSet::from_iter(third.chars());

    // first, second, thirdで、重複したものだけを抽出。
    let duplicate_items: HashSet<char> = first.intersection(&second).cloned().collect();
    let duplicate_items: Vec<char> = duplicate_items.intersection(&third).cloned().collect();

    if duplicate_items.len() != 1 {
        return Err(format!(
            "expected only one item duplicated but them: {:?}",
            duplicate_items
        ));
    }

    Ok(duplicate_items[0])
}

fn calc_part2(r: impl BufRead) -> u32 {
    r.lines()
        .flatten()
        .chunks(3) // 3行で一つのグループにする
        .into_iter()
        .map(|mut chunk| {
            let first = chunk.next().expect("expected first line");
            let second = chunk.next().expect("expected second line");
            let third = chunk.next().expect("expected third line");

            let duplicate_item = find_duplicate_item_of_group(&first, &second, &third)
                .expect("expected only one item duplicated");

            duplicate_item
                .to_priority()
                .expect("expected priority value")
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day03/data/input.txt")?);
    let total = calc_part2(r);
    println!("answer: {}", total);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicate_item_of_group() {
        assert_eq!(
            find_duplicate_item_of_group(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            Ok('r')
        );

        assert_eq!(
            find_duplicate_item_of_group(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Ok('Z')
        );
    }

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let total = calc_part2(r);
        assert_eq!(total, 70);
    }
}
