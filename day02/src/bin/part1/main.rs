use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

const LOST_POINT: u32 = 0;
const DRAW_POINT: u32 = 3;
const WIN_POINT: u32 = 6;

const SHAPE_POINT_ROCK: u32 = 1;
const SHAPE_POINT_PAPER: u32 = 2;
const SHAPE_POINT_SCISSORS: u32 = 3;

fn calc_score(opponent: &Shape, you: &Shape) -> u32 {
    let shape_point = match you {
        Shape::Rock => SHAPE_POINT_ROCK,
        Shape::Paper => SHAPE_POINT_PAPER,
        Shape::Scissors => SHAPE_POINT_SCISSORS,
    };

    let outcome_point = match (opponent, you) {
        (Shape::Rock, Shape::Rock) => DRAW_POINT,
        (Shape::Rock, Shape::Paper) => WIN_POINT,
        (Shape::Rock, Shape::Scissors) => LOST_POINT,
        (Shape::Paper, Shape::Rock) => LOST_POINT,
        (Shape::Paper, Shape::Paper) => DRAW_POINT,
        (Shape::Paper, Shape::Scissors) => WIN_POINT,
        (Shape::Scissors, Shape::Rock) => WIN_POINT,
        (Shape::Scissors, Shape::Paper) => LOST_POINT,
        (Shape::Scissors, Shape::Scissors) => DRAW_POINT,
    };

    shape_point + outcome_point
}

fn parse(line: &str) -> (Shape, Shape) {
    let mut iter = line.split_whitespace();

    let opponent = match iter.next() {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => panic!("invalid input"),
    };

    let you = match iter.next() {
        Some("X") => Shape::Rock,
        Some("Y") => Shape::Paper,
        Some("Z") => Shape::Scissors,
        _ => panic!("invalid input"),
    };

    (opponent, you)
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day02/data/part1/input.txt").unwrap());

    let mut sum = 0;
    for line in r.lines() {
        let line = line?;

        let (opponent, you) = parse(&line);
        let score = calc_score(&opponent, &you);
        println!("{:?} x {:?} = {}", opponent, you, score);
        sum += score;
    }
    println!("sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{calc_score, parse};

    #[test]
    fn test() {
        let text = r#"A Y
B X
C Z
"#;

        let mut sum = 0;
        for line in text.trim().lines() {
            let (opponent, you) = parse(line);
            let score = calc_score(&opponent, &you);
            println!("{:?} x {:?} = {}", opponent, you, score);
            sum += score;
        }
        println!("sum: {}", sum);
    }
}
