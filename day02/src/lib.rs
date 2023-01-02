use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::value;
use nom::sequence::separated_pair;
use nom::IResult;

/// グー、パー、チョキ
#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn get_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

/// 勝敗
#[derive(Debug, PartialEq, Clone)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn get_score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

pub fn parse_shapes(line: &str) -> anyhow::Result<(Shape, Shape)> {
    match separated_pair(parse_opponent, space1, parse_shape)(line) {
        Ok((_, (opponent, my_shape))) => Ok((opponent, my_shape)),
        Err(e) => Err(anyhow::anyhow!("parse error: {}", e.to_string())),
    }
}

pub fn parse_shape_and_outcome(line: &str) -> anyhow::Result<(Shape, Outcome)> {
    match separated_pair(parse_opponent, space1, parse_outcome)(line) {
        Ok((_, (opponent, outcome))) => Ok((opponent, outcome)),
        Err(e) => Err(anyhow::anyhow!("parse error: {}", e.to_string())),
    }
}

fn parse_opponent(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, tag("A")),
        value(Shape::Paper, tag("B")),
        value(Shape::Scissors, tag("C")),
    ))(input)
}

/// part1用
fn parse_shape(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, tag("X")),
        value(Shape::Paper, tag("Y")),
        value(Shape::Scissors, tag("Z")),
    ))(input)
}

/// part2用
fn parse_outcome(input: &str) -> IResult<&str, Outcome> {
    alt((
        value(Outcome::Lose, tag("X")),
        value(Outcome::Draw, tag("Y")),
        value(Outcome::Win, tag("Z")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use Outcome::*;
    use Shape::*;

    use super::*;

    #[test]
    fn test_parse_shapes() {
        let r = include_str!("../data/sample.txt").as_bytes();

        let rounds: Vec<(Shape, Shape)> = r
            .lines()
            .flatten()
            .flat_map(|line| parse_shapes(&line))
            .collect();

        assert_eq!(
            rounds,
            vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors),]
        );
    }

    #[test]
    fn test_parse_shape_and_outcome() {
        let r = include_str!("../data/sample.txt").as_bytes();

        let rounds: Vec<(Shape, Outcome)> = r
            .lines()
            .flatten()
            .flat_map(|line| parse_shape_and_outcome(&line))
            .collect();

        assert_eq!(rounds, vec![(Rock, Draw), (Paper, Lose), (Scissors, Win),]);
    }
}
