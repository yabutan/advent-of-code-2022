use itertools::Itertools;

/// 指定の配列がユニークか判定
fn is_unique(chars: &[u8]) -> bool {
    let count = chars.iter().unique().count();
    count == chars.len()
}

pub fn find_marker<const MARKER_SIZE: usize>(text: &str) -> usize {
    // 指定文字数で窓移動しながら、ユニークな場所を探す。
    text.as_bytes()
        .windows(MARKER_SIZE)
        .enumerate()
        .find(|(_, s)| is_unique(s))
        .map(|(i, _)| i + MARKER_SIZE) // マーカー終わり位置を返す
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
}
