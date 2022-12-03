use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn get_elf(r: impl BufRead) -> Result<Vec<u32>> {
    let mut list = vec![];
    let mut sum = 0;
    for line in r.lines() {
        let line = line?;

        if line.is_empty() {
            list.push(sum);
            sum = 0;
            continue;
        }

        let value: u32 = line.trim().parse()?;
        sum += value;
    }

    if sum > 0 {
        list.push(sum);
    }

    Ok(list)
}

fn get_sum_of_top_tree(r: impl BufRead) -> Result<u32> {
    let mut list = get_elf(r)?;
    list.sort();
    list.reverse();

    println!("list: {:?}", list);

    let sum: u32 = list.iter().take(3).sum();

    Ok(sum)
}

fn main() -> Result<()> {
    let r = BufReader::new(fs::File::open("./day1/data/part1/input.txt").unwrap());
    let sum = get_sum_of_top_tree(r)?;
    println!("answer: {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_sum_of_top_tree() {
        let lines = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

        let sum = get_sum_of_top_tree(lines.as_bytes()).expect("failed to get max sum");
        assert_eq!(sum, 45000);
    }
}
