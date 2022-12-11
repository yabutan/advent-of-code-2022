use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt, recognize, value};
use nom::sequence::{preceded, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day10/data/input.txt")?);

    let mut w = BufWriter::new(std::io::stdout());
    draw(&mut w, r);

    Ok(())
}

fn draw(w: &mut impl Write, r: impl BufRead) {
    let mut crt = Crt::with_writer(w);
    for line in r.lines() {
        let line = line.unwrap();
        let (_, instruction) = parse_instruction(&line).expect("parse error");
        crt.run(&instruction);
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

const WIDTH: u32 = 40;

struct Crt<'a, W> {
    cycle_count: u32,
    x: i32,
    w: &'a mut W,
}

impl<'a, W: Write> Crt<'a, W> {
    fn with_writer(w: &'a mut W) -> Self {
        Self {
            cycle_count: 0,
            x: 1,
            w,
        }
    }

    fn tick(&mut self) {
        // draw
        let pos = (self.cycle_count % WIDTH) as i32;

        let c = if (self.x - 1) <= pos && pos <= (self.x + 1) {
            '#' // lit
        } else {
            '.' // dark
        };

        write!(self.w, "{}", c).unwrap();
        if pos == WIDTH as i32 - 1 {
            writeln!(self.w).unwrap();
        }

        self.cycle_count += 1;
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.tick(),
            Instruction::AddX(x) => {
                self.tick();
                self.tick();

                self.x += *x;
            }
        }
    }
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
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_draw() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let mut w = BufWriter::new(Vec::new());
        draw(&mut w, r);

        let text = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
        .trim_start();

        assert_eq!(text, String::from_utf8(w.into_inner().unwrap()).unwrap());
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
