use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt, recognize, value};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day10/data/input.txt")?);
    let signals = simulate(r);

    let sum: i32 = signals
        .iter()
        .map(|(_, _, signal_strength)| signal_strength)
        .sum();

    println!("answer: {}", sum);

    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Cpu<F> {
    cycle_count: u32,
    x: i32,
    callback: F,
}

impl<F: FnMut(&u32, &i32)> Cpu<F> {
    fn with_callback(callback: F) -> Self {
        Self {
            cycle_count: 0,
            x: 1,
            callback,
        }
    }

    fn tick(&mut self) {
        self.cycle_count += 1;

        // callback
        (self.callback)(&self.cycle_count, &self.x);
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.tick(),
            Instruction::AddX(x) => {
                self.tick();
                self.tick();
                self.x += x;
            }
        }
    }
}

/// (cycle_count, x, signal_strength)
fn simulate(r: impl BufRead) -> Vec<(u32, i32, i32)> {
    let mut signals = Vec::new();

    let mut cpu = Cpu::with_callback(|cycle_count, x| {
        if matches!(cycle_count, 20 | 60 | 100 | 140 | 180 | 220) {
            let signal_strength = *cycle_count as i32 * x;

            signals.push((*cycle_count, *x, signal_strength));
        };
    });

    for line in r.lines() {
        let line = line.unwrap();
        let (_, instruction) = parse_instruction(&line).expect("parse error");
        cpu.run(&instruction);
    }

    signals
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<i32>().unwrap()
    })(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let noop = value(Instruction::Noop, tag("noop"));
    let addx = map(preceded(tag("addx "), parse_num), Instruction::AddX);

    alt((noop, addx))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simulate() {
        let signals = simulate(include_str!("../../data/sample.txt").as_bytes());

        assert_eq!(signals.len(), 6);
        assert_eq!(signals[0], (20, 21, 420));
        assert_eq!(signals[1], (60, 19, 1140));
        assert_eq!(signals[2], (100, 18, 1800));
        assert_eq!(signals[3], (140, 21, 2940));
        assert_eq!(signals[4], (180, 16, 2880));
        assert_eq!(signals[5], (220, 18, 3960));

        let sum: i32 = signals
            .iter()
            .map(|(_, _, signal_strength)| signal_strength)
            .sum();
        assert_eq!(sum, 13140);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("addx 15"), Ok(("", Instruction::AddX(15))));
        assert_eq!(parse_instruction("addx 6"), Ok(("", Instruction::AddX(6))));
        assert_eq!(parse_instruction("addx -11"), Ok(("", Instruction::AddX(-11))));
        assert_eq!(parse_instruction("addx -3"), Ok(("", Instruction::AddX(-3))));
        assert_eq!(parse_instruction("noop"), Ok(("", Instruction::Noop)));
    }
}
