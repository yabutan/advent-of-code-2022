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

    //println!("answer: {}", sum);

    Ok(())
}
