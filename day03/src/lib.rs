/// 文字列を半分に分割する。
/// 文字数が偶数でなければNoneを返す。
pub fn split_half(line: &str) -> Option<(&str, &str)> {
    if line.len() % 2 != 0 {
        return None;
    }

    let left = &line[0..line.len() / 2];
    let right = &line[line.len() / 2..];
    Some((left, right))
}

/// プライオリティ値の取得
pub trait Priority {
    fn to_priority(&self) -> Option<u32>;
}

impl Priority for char {
    fn to_priority(&self) -> Option<u32> {
        let p = match self {
            'a'..='z' => u32::from(*self) - u32::from('a') + 1,
            'A'..='Z' => u32::from(*self) - u32::from('A') + 27,
            _ => return None,
        };
        Some(p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_half() {
        // 前後半分にした文字列を返すこと。
        assert_eq!(split_half("abcdef"), Some(("abc", "def")));
        assert_eq!(split_half("af"), Some(("a", "f")));

        // 文字列が偶数でなければ、Noneを返すこと。
        assert_eq!(split_half("abc"), None);
    }

    #[test]
    fn test_get_priority() {
        // プライオリティ値に変換できること
        assert_eq!('a'.to_priority(), Some(1));
        assert_eq!('z'.to_priority(), Some(26));
        assert_eq!('A'.to_priority(), Some(27));
        assert_eq!('Z'.to_priority(), Some(52));

        //　a-zA-z 以外はNoneになること
        assert_eq!('0'.to_priority(), None);
        assert_eq!('*'.to_priority(), None);
    }
}
