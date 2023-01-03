use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use day03::{Priority, split_half};

/// 左と右で重複したアイテムを返す。
/// ひとつだけ重複してることを期待する。
fn find_duplicate_item_of_compartments(left: &str, right: &str) -> Result<char, String> {
    // ハッシュセットにしてユニーク化
    let left: HashSet<char> = HashSet::from_iter(left.chars());
    let right: HashSet<char> = HashSet::from_iter(right.chars());

    // 重複したものだけを返す。
    let duplicate_items: Vec<char> = left.intersection(&right).cloned().collect();

    if duplicate_items.len() != 1 {
        return Err(format!(
            "expected only one item duplicated but them: {:?}",
            duplicate_items
        ));
    }

    Ok(duplicate_items[0])
}

fn calc_part1(r: impl BufRead) -> u32 {
    r.lines()
        .flatten()
        .map(|line| {
            // 左と右に分割
            let (left, right) = split_half(&line).expect("expected even length");

            // 重複アイテムを取得
            let duplicate_item = find_duplicate_item_of_compartments(left, right)
                .expect("expected only one item duplicated");

            // プライオリティ値を取得
            duplicate_item
                .to_priority()
                .expect("expected priority value")
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day03/data/input.txt")?);
    let total = calc_part1(r);
    println!("answer: {}", total);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicate_item_of_compartments() {
        let (left, right) = split_half("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(find_duplicate_item_of_compartments(left, right), Ok('p'));

        let (left, right) = split_half("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        assert_eq!(find_duplicate_item_of_compartments(left, right), Ok('L'));

        let (left, right) = split_half("PmmdzqPrVvPwwTWBwg").unwrap();
        assert_eq!(find_duplicate_item_of_compartments(left, right), Ok('P'));

        // 重複がない
        assert!(find_duplicate_item_of_compartments("abc", "def").is_err());
        // 重複が一つじゃない
        assert!(find_duplicate_item_of_compartments("abc", "abd").is_err());
    }

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let total = calc_part1(r);
        assert_eq!(total, 157);
    }
}
