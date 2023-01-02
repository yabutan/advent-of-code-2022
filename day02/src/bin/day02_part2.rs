use std::fs::File;
use std::io::{BufRead, BufReader};

use day02::{parse_shape_and_outcome, Outcome, Shape};

/// 結果に合わせた手を返す
fn get_shape(opponent: &Shape, outcome: &Outcome) -> Shape {
    use Outcome::{Draw, Lose, Win};
    use Shape::{Paper, Rock, Scissors};

    match (opponent, outcome) {
        (Rock, Win) => Paper,
        (Rock, Draw) => Rock,
        (Rock, Lose) => Scissors,

        (Paper, Win) => Scissors,
        (Paper, Draw) => Paper,
        (Paper, Lose) => Rock,

        (Scissors, Win) => Rock,
        (Scissors, Draw) => Scissors,
        (Scissors, Lose) => Paper,
    }
}

fn simulate_part2(r: impl BufRead) -> u32 {
    let rounds: Vec<(Shape, Outcome)> = r
        .lines()
        .flatten()
        .flat_map(|line| parse_shape_and_outcome(&line))
        .collect();

    rounds
        .iter()
        .map(|(opponent, outcome)| {
            let my_shape = get_shape(opponent, outcome);
            my_shape.get_score() + outcome.get_score()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day02/data/input.txt")?);
    let total_score = simulate_part2(r);
    println!("total_score: {}", total_score);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let sum = simulate_part2(r);
        assert_eq!(sum, 12);
    }
}
