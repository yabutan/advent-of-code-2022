use std::fs::File;
use std::io::BufReader;

use day05::{simulate, Operation};

/// CrateMover9001による移動
fn move_by_9001(stacks: &mut [Vec<char>], op: &Operation) {
    // 移動元のスタック
    let stack = &mut stacks[(op.from - 1) as usize];

    // 末尾から指定個数分だけ取り出し、順番はそのまま。
    let drained: Vec<char> = stack.drain((stack.len() - op.times as usize)..).collect();

    // 移動先のスタックに追加
    stacks[(op.to - 1) as usize].extend(drained);
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day05/data/input.txt")?);
    let top_crates = simulate(r, move_by_9001);
    println!("answer: {}", top_crates);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let top_crates = simulate(r, move_by_9001);
        assert_eq!(top_crates, "MCD");
    }

    #[test]
    fn test_move_by_9001() {
        let mut stacks = vec![vec!['A'], vec!['B'], vec!['C', 'D']];

        move_by_9001(
            &mut stacks,
            &Operation {
                times: 2,
                from: 3,
                to: 1,
            },
        );

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['A', 'C', 'D']);
        assert_eq!(stacks[1], vec!['B']);
        assert_eq!(stacks[2], vec![]);
    }
}
