extern crate core;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day14/data/input.txt")?);
    let ret = count_sand::<WIDTH, HEIGHT>(r);
    println!("answer: {}", ret);
    Ok(())
}

fn count_sand<const W: usize, const H: usize>(r: impl BufRead) -> usize {
    let mut stage: Stage<W, H> = Stage::make_stage(r);

    while stage.turn() {
        //stage.display_stage();
    }
    stage.display_stage();

    stage
        .data
        .iter()
        .flatten()
        .filter(|&&m| m == Mark::Sand)
        .count()
}

// 498,4
fn parse_pos(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, x) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    Ok((input, (x, y)))
}

// 498,4 -> 498,6 -> 496,6
fn parse_path(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(tag(" -> "), parse_pos)(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mark {
    Source,
    Air,
    Sand,
    Rock,
}

struct Stage<const W: usize, const H: usize> {
    data: [[Mark; W]; H],
    bottom_y: usize,
}

impl<const W: usize, const H: usize> Stage<W, H> {
    fn make_stage(r: impl BufRead) -> Self {
        let mut data = [[Mark::Air; W]; H];
        data[0][500] = Mark::Source;

        let mut highest = 0;
        for line in r.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }

            let (_, path_list) = parse_path(&line).expect("parse error");

            let max_y = path_list.iter().map(|(_, y)| y).max().unwrap();
            if *max_y > highest {
                highest = *max_y;
            }

            for i in 1..path_list.len() {
                let a = path_list[i - 1];
                let b = path_list[i];

                if a.0 == b.0 {
                    // vertical
                    for y in min(a.1, b.1)..=max(a.1, b.1) {
                        data[y as usize][a.0 as usize] = Mark::Rock;
                    }
                } else {
                    // horizontal
                    for x in min(a.0, b.0)..=max(a.0, b.0) {
                        data[a.1 as usize][x as usize] = Mark::Rock;
                    }
                }
            }
        }

        Self {
            data,
            bottom_y: (highest + 2) as usize,
        }
    }

    fn display_stage(&self) {
        for y in 0..H {
            for x in 420..580 {
                if y == self.bottom_y {
                    print!("#");
                } else {
                    match self.data[y][x] {
                        Mark::Air => print!("."),
                        Mark::Source => print!("+"),
                        Mark::Sand => print!("o"),
                        Mark::Rock => print!("#"),
                    }
                }
            }
            println!()
        }
    }

    fn turn(&mut self) -> bool {
        let (mut x, mut y) = (500, 0);
        for new_y in 1..self.bottom_y {
            // drop a sand from source pos
            if self.data[new_y][x] == Mark::Air {
                y = new_y;
                continue;
            }

            // if down left is air then move to
            if x == 0 {
                panic!("x is out of range");
            }
            if self.data[new_y][x - 1] == Mark::Air {
                x -= 1;
                y = new_y;
                continue;
            }

            // else if down right is air then move to
            if x + 1 >= W {
                // end
                panic!("x is out of range");
            }
            if self.data[new_y][x + 1] == Mark::Air {
                x += 1;
                y = new_y;
                continue;
            }

            break;
        }

        // otherwise come to rest
        self.data[y][x] = Mark::Sand;
        if y == 0 && x == 500 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let mut stage: Stage<WIDTH, HEIGHT> = Stage::make_stage(r);

        let mut c = 0;
        while stage.turn() {
            c += 1;
            println!("turn {}", c);
            //stage.display_stage();
        }
        stage.display_stage();

        let sand_count = stage
            .data
            .iter()
            .flatten()
            .filter(|&&m| m == Mark::Sand)
            .count();
        println!("sand count: {}", sand_count);
    }

    #[test]
    fn test_path_list() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let stage: Stage<WIDTH, HEIGHT> = Stage::make_stage(r);
        stage.display_stage();
    }
}
