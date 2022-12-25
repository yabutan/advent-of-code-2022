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

pub struct Calculator {
    value_map: HashMap<String, Expr>,
}

impl Calculator {
    pub fn new(expr_list: Vec<(String, Expr)>) -> Self {
        Self {
            value_map: HashMap::from_iter(expr_list),
        }
    }

    pub fn new_with_unknown_humn(expr_list: Vec<(String, Expr)>) -> Self {
        let mut value_map: HashMap<String, Expr> = HashMap::from_iter(expr_list);

        // humnはわからないものとして扱う。
        value_map.insert("humn".to_string(), Expr::Unknown);

        Self { value_map }
    }

    pub fn resolve_humn(&self) -> i64 {
        match &self.value_map["root"] {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                // rootはEqualとして扱うので、わかった方は確定値として、わからなかった方の値を求めていく。
                match (self.calc(a), self.calc(b)) {
                    (Some(a_value), None) => self.resolve(b, a_value),
                    (None, Some(b_value)) => self.resolve(a, b_value),
                    _ => unreachable!("illegal state"),
                }
            }
            _ => unreachable!("root should be an operation"),
        }
    }

    fn resolve(&self, name: &str, value: i64) -> i64 {
        let expr = &self.value_map[name];
        match expr {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                match (self.calc(a), self.calc(b)) {
                    (Some(a_value), None) => {
                        // 確定値から逆算して、わからない方の値を計算。
                        let value = match expr {
                            Expr::Add(_, _) => value - a_value,
                            Expr::Sub(_, _) => a_value - value,
                            Expr::Mul(_, _) => value / a_value,
                            Expr::Div(_, _) => a_value / value,
                            _ => unreachable!("unreachable"),
                        };

                        self.resolve(b, value)
                    }
                    (None, Some(b_value)) => {
                        // 確定値から逆算して、わからない方の値を計算。
                        let value = match expr {
                            Expr::Add(_, _) => value - b_value,
                            Expr::Sub(_, _) => value + b_value,
                            Expr::Mul(_, _) => value / b_value,
                            Expr::Div(_, _) => value * b_value,
                            _ => unreachable!("unreachable"),
                        };

                        self.resolve(a, value)
                    }
                    _ => unreachable!("illegal state"),
                }
            }
            Expr::Num(v) => *v,
            Expr::Unknown => value, // this is the value we are looking for
        }
    }

    pub fn calc(&self, name: &str) -> Option<i64> {
        let expr = &self
            .value_map
            .get(name)
            .unwrap_or_else(|| panic!("name not found: {}", name));

        let (a, b) = match expr {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                let (Some(a), Some(b)) = ( self.calc(a ) , self.calc(b)) else {
                    return None;
                };
                (a, b)
            }
            Expr::Num(v) => return Some(*v),
            Expr::Unknown => return None,
        };

        match expr {
            Expr::Add(_, _) => Some(a + b),
            Expr::Sub(_, _) => Some(a - b),
            Expr::Mul(_, _) => Some(a * b),
            Expr::Div(_, _) => Some(a / b),
            _ => unreachable!("unreachable"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),
    Unknown,
}

pub fn read_expr(r: impl BufRead) -> Vec<(String, Expr)> {
    r.lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let (_, (name, expr)) = parse_operation(&line).unwrap();
            (name.to_string(), expr)
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_expr() {
        let list = read_expr(include_str!("../data/sample.txt").as_bytes());

        assert_eq!(list.len(), 15);
        assert_eq!(
            list[0],
            (
                "root".to_string(),
                Expr::Add("pppw".to_string(), "sjmn".to_string())
            )
        );

        assert_eq!(list[1], ("dbpl".to_string(), Expr::Num(5)));

        assert_eq!(
            list[4],
            (
                "ptdq".to_string(),
                Expr::Sub("humn".to_string(), "dvpt".to_string())
            )
        );
    }
}
