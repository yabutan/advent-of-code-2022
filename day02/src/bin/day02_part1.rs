use std::fs::File;
use std::io::{BufRead, BufReader};

use day02::{Outcome, parse_shapes, Shape};

/// 勝ち負け判定
pub fn judge(opponent: &Shape, my_shape: &Shape) -> Outcome {
    use Outcome::{Draw, Lose, Win};
    use Shape::{Paper, Rock, Scissors};

    match (opponent, my_shape) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Lose,

        (Paper, Rock) => Lose,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,

        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (Scissors, Scissors) => Draw,
    }
}

fn simulate_part1(r: impl BufRead) -> u32 {
    let rounds: Vec<(Shape, Shape)> = r
        .lines()
        .flatten()
        .flat_map(|line| parse_shapes(&line))
        .collect();

    rounds
        .iter()
        .map(|(opponent, my_shape)| {
            let outcome = judge(opponent, my_shape);
            my_shape.get_score() + outcome.get_score()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day02/data/input.txt")?);

    let total_score = simulate_part1(r);
    println!("total_score: {}", total_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let sum = simulate_part1(r);
        assert_eq!(sum, 15);
    }
}
