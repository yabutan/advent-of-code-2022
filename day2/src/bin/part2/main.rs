use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

const LOST_POINT: u32 = 0;
const DRAW_POINT: u32 = 3;
const WIN_POINT: u32 = 6;

const SHAPE_POINT_ROCK: u32 = 1;
const SHAPE_POINT_PAPER: u32 = 2;
const SHAPE_POINT_SCISSORS: u32 = 3;

fn calc_score(opponent: &Shape, outcome: &Outcome) -> u32 {
    let you = match (opponent, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
    };

    let shape_point = match you {
        Shape::Rock => SHAPE_POINT_ROCK,
        Shape::Paper => SHAPE_POINT_PAPER,
        Shape::Scissors => SHAPE_POINT_SCISSORS,
    };

    let outcome_point = match outcome {
        Outcome::Win => WIN_POINT,
        Outcome::Draw => DRAW_POINT,
        Outcome::Lose => LOST_POINT,
    };

    shape_point + outcome_point
}

fn parse(line: &str) -> (Shape, Outcome) {
    let mut iter = line.split_whitespace();

    let opponent = match iter.next() {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => panic!("invalid input"),
    };

    let outcome = match iter.next() {
        Some("X") => Outcome::Lose,
        Some("Y") => Outcome::Draw,
        Some("Z") => Outcome::Win,
        _ => panic!("invalid input"),
    };

    (opponent, outcome)
}

fn calc(r: impl BufRead) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in r.lines() {
        let line = line?;
        let (opponent, outcome) = parse(&line);
        let score = calc_score(&opponent, &outcome);
        println!("{:?} x {:?} = {}", opponent, outcome, score);
        sum += score;
    }
    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day2/data/part1/input.txt").unwrap());
    let sum = calc(r)?;
    println!("sum: {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        let text = r#"A Y
B X
C Z
"#;

        let sum = calc(text.trim().as_bytes()).unwrap();
        assert_eq!(sum, 12);
    }
}
