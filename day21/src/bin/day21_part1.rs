use std::collections::HashMap;
use std::fmt::Debug;
use std::io::BufRead;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, space1};
use nom::combinator::map;
use nom::sequence::terminated;
use nom::Err::Error;
use nom::IResult;

fn main() {
    //let r = include_str!("../../data/sample.txt").as_bytes();
    let r = include_str!("../../data/input.txt").as_bytes();

    let expr_list: Vec<(String, Expr)> = r
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let (_, (name, expr)) = parse_operation(&line).unwrap();
            (name.to_string(), expr)
        })
        .collect();

    let calculator = Calculator::new(expr_list);

    let ret = calculator.calc("root");
    println!("result: {:?}", ret);
}

struct Calculator {
    value_map: HashMap<String, Expr>,
}

impl Calculator {
    fn new(expr_list: Vec<(String, Expr)>) -> Self {
        Self {
            value_map: HashMap::from_iter(expr_list),
        }
    }

    fn calc(&self, name: &str) -> i64 {
        let expr = &self.value_map[name];

        match expr {
            Expr::Add(a, b) => self.calc(a) + self.calc(b),
            Expr::Sub(a, b) => self.calc(a) - self.calc(b),
            Expr::Mul(a, b) => self.calc(a) * self.calc(b),
            Expr::Div(a, b) => self.calc(a) / self.calc(b),
            Expr::Num(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Expr {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),
}

fn parse_op(input: &str) -> IResult<&str, Expr> {
    let (input, left) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, op) = alt((tag("+"), tag("-"), tag("*"), tag("/")))(input)?;
    let (input, _) = space1(input)?;
    let (input, right) = alpha1(input)?;

    let expr = match op {
        "+" => Expr::Add(left.to_string(), right.to_string()),
        "-" => Expr::Sub(left.to_string(), right.to_string()),
        "*" => Expr::Mul(left.to_string(), right.to_string()),
        "/" => Expr::Div(left.to_string(), right.to_string()),
        _ => {
            return Err(Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, expr))
}

// root: pppw + sjmn
// dbpl: 5
// ptdq: humn - dvpt
// sjmn: drzm * dbpl
// pppw: cczh / lfqf
fn parse_operation(input: &str) -> IResult<&str, (&str, Expr)> {
    let (input, name) = terminated(alpha1, tag(": "))(input)?;
    let (input, expr) = alt((map(complete::i64, Expr::Num), parse_op))(input)?;
    Ok((input, (name, expr)))
}
