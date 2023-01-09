use std::fs::File;
use std::io::{BufRead, BufReader};

use day06::find_marker;

pub fn find_message_marker(text: &str) -> usize {
    // メッセージマーカーは14文字
    find_marker::<14>(text)
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day06/data/input.txt")?);
    let text = r.lines().next().unwrap()?;

    let ret = find_message_marker(&text);
    println!("answer: {}", ret);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_message_marker() {
        assert_eq!(find_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
