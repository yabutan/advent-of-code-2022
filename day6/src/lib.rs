use itertools::Itertools;

pub fn find_packet_marker(text: &str) -> usize {
    find_index(text, 4)
}

pub fn find_message_marker(text: &str) -> usize {
    find_index(text, 14)
}

fn is_unique(chars: &[u8]) -> bool {
    let count = chars.iter().unique().count();
    count == chars.len()
}

fn find_index(text: &str, size: usize) -> usize {
    text.as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, s)| is_unique(s))
        .map(|(i, _)| i + size)
        .expect("not found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique("abcd".as_bytes()));
        assert!(!is_unique("abca".as_bytes()));
    }

    #[test]
    fn test_find_packet_marker() {
        assert_eq!(find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_find_message_marker() {
        assert_eq!(find_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
