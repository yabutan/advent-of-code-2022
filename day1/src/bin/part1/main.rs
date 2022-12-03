use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn get_max_sum(r: impl BufRead) -> Result<u32> {
    let mut max = 0;
    let mut sum = 0;
    for line in r.lines() {
        let line = line?;

        println!("line: {}", line);

        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }

        let value: u32 = line.trim().parse()?;
        sum += value;
    }

    if sum > max {
        max = sum;
    }

    Ok(max)
}

fn main() -> Result<()> {
    let r = BufReader::new(fs::File::open("./day1/data/part1/input.txt").unwrap());
    let max = get_max_sum(r)?;
    println!("answer: {}", max);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_sum_max() {
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

        let max = get_max_sum(lines.as_bytes()).expect("failed to get max sum");
        assert_eq!(max, 24000);

        let lines = r#"1000

10000
10000
"#;

        let max = get_max_sum(lines.as_bytes()).expect("failed to get max sum");
        assert_eq!(max, 20000);

        let lines = r#"1000

10000
10000"#;

        let max = get_max_sum(lines.as_bytes()).expect("failed to get max sum");
        assert_eq!(max, 20000);
    }
}
