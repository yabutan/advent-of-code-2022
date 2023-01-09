use std::fs::File;
use std::io::{BufRead, BufReader};

use day06::find_marker;

fn find_packet_marker(text: &str) -> usize {
    // パケットマーカーは4文字
    find_marker::<4>(text)
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day06/data/input.txt")?);
    let text = r.lines().next().unwrap()?;

    let ret = find_packet_marker(&text);
    println!("answer: {}", ret);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_packet_marker() {
        assert_eq!(find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
