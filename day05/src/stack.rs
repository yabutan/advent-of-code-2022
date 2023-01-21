use std::io::BufRead;

use itertools::{FoldWhile, Itertools};
use FoldWhile::{Continue, Done};

pub fn read_stacks(r: &mut impl BufRead) -> Vec<Vec<char>> {
    // 空行まで読み込み
    let header = r
        .by_ref()
        .lines()
        .flatten()
        .fold_while(String::new(), |mut acc, line| {
            if line.is_empty() {
                return Done(acc);
            }

            acc.push_str(&line);
            acc.push('\n');
            Continue(acc)
        })
        .into_inner();

    // スタック情報としてパース
    parse_crate_stacks(&header)
}

/// 数字がある位置をVecで返す。
fn parse_stack_indices(line: &str) -> Vec<usize> {
    line.char_indices()
        .filter(|(_, c)| c.is_ascii_digit())
        .map(|(i, _)| i)
        .collect()
}

/// 番号毎に、クレートスタックを取得
fn parse_crate_stacks(text: &str) -> Vec<Vec<char>> {
    // 扱いやすいように上下反転
    let lines: Vec<&str> = text.trim_end().lines().rev().collect();
    // 番号が書かれている行から、各スタックのオフセットを取得
    let indices = parse_stack_indices(lines[0]);

    indices
        .into_iter()
        .map(|index| {
            lines
                .iter()
                .skip(1) // スタック番号の行はスキップ
                .map_while(|line| match line.chars().nth(index) {
                    Some(' ') | None => None,
                    Some('[') | Some(']') => unreachable!("illegal index"), // ここに来るということは、インデックスがずれている。
                    Some(c) => Some(c),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_stack_indices() {
        assert_eq!(parse_stack_indices(" 1   2   3 "), vec![1, 5, 9]);

        assert_eq!(
            parse_stack_indices(" 1   2   3   4   5   6   7   8   9 "),
            vec![1, 5, 9, 13, 17, 21, 25, 29, 33]
        );
    }

    #[test]
    fn test_parse_crate_stacks() {
        let stacks = parse_crate_stacks(indoc! {r#"
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3
        "#});

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);
    }
}
