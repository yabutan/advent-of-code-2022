use std::io::BufRead;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Action {
    time: u8,
    from: u8,
    to: u8,
}

pub trait CraneMove {
    fn do_move(&self, stacks: &mut [Vec<char>], action: &Action);
}

// for Part1
pub struct Crane9000;

impl CraneMove for Crane9000 {
    fn do_move(&self, stacks: &mut [Vec<char>], action: &Action) {
        for _ in 0..action.time {
            let a = stacks[(action.from - 1) as usize].pop().unwrap();
            stacks[(action.to - 1) as usize].push(a);
        }
    }
}

// for Part2
pub struct Crane9001;

impl CraneMove for Crane9001 {
    fn do_move(&self, stacks: &mut [Vec<char>], action: &Action) {
        let list_from = &mut stacks[(action.from - 1) as usize];

        let index_of_begin = list_from.len() - action.time as usize;
        let moved: Vec<char> = list_from.drain(index_of_begin..).collect();

        stacks[(action.to - 1) as usize].extend(moved);
    }
}

pub fn simulate(mut r: impl BufRead, crane: impl CraneMove) -> String {
    let mut stacks = {
        let mut header = String::new();

        for line in r.by_ref().lines() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            header.push_str(&line);
            header.push('\n');
        }

        parse_crate_stacks(&header)
    };
    println!("stacks: {:?}", stacks);

    // process actions
    for line in r.lines() {
        let line = line.unwrap();
        let (_, action) = parse_action(&line).unwrap();

        crane.do_move(&mut stacks, &action);
        println!("action:{:?} -> stacks:{:?}", action, stacks);
    }

    // read top of stacks
    let mut ret = String::new();
    for x in stacks {
        if let Some(c) = x.last() {
            ret.push(*c);
        }
    }
    ret
}

fn parse_crate_stacks(text: &str) -> Vec<Vec<char>> {
    let lines: Vec<_> = text.trim_end().lines().collect();
    let line_num = lines.len() - 1;

    let crate_number_line = lines.last().unwrap();
    println!("crate_number_line: {}", crate_number_line);

    let (_, crate_num) = parse_crate_num(crate_number_line).unwrap();
    println!("crate_num:{}", crate_num);

    let mut stacks = Vec::new();
    for x in 0..crate_num {
        let mut stack = Vec::new();

        // 1,5,9,13,...
        let x = 1 + (x * 4);

        for y in (0..line_num).rev() {
            match lines[y].chars().nth(x as usize) {
                None => break,
                Some(' ') => break,
                Some(c) => stack.push(c),
            };
        }

        stacks.push(stack);
    }

    stacks
}

fn parse_crate_num(input: &str) -> IResult<&str, u32> {
    let (input, num) = preceded(
        multispace1,
        separated_list1(multispace1, map_res(digit1, str::parse::<u32>)),
    )(input)?;
    Ok((input, *num.last().unwrap()))
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, time) = preceded(tag("move "), map_res(digit1, str::parse::<u8>))(input)?;
    let (input, from) = preceded(tag(" from "), map_res(digit1, str::parse::<u8>))(input)?;
    let (input, to) = preceded(tag(" to "), map_res(digit1, str::parse::<u8>))(input)?;
    Ok((input, Action { time, from, to }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_action() {
        let (_, action) = parse_action("move 1 from 2 to 1").unwrap();
        assert_eq!(
            action,
            Action {
                time: 1,
                from: 2,
                to: 1
            }
        );

        let (_, action) = parse_action("move 12 from 9 to 3").unwrap();
        assert_eq!(
            action,
            Action {
                time: 12,
                from: 9,
                to: 3
            }
        );
    }

    #[test]
    fn test_parse_crate_num() {
        let (_, num) = parse_crate_num(" 1   2   3 ").unwrap();
        assert_eq!(num, 3);

        let (_, num) = parse_crate_num(" 1   2   3   4   5   6   7   8   9 ").unwrap();
        assert_eq!(num, 9);
    }

    #[test]
    fn test_simulate() {
        let text = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

        let ret = simulate(text.as_bytes(), Crane9000);
        println!("answer:{:?}", ret);
        assert_eq!(ret, "CMZ");

        let ret = simulate(text.as_bytes(), Crane9001);
        println!("answer:{:?}", ret);
        assert_eq!(ret, "MCD");
    }

    #[test]
    fn test_parse_crate_stacks() {
        let text = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 
 "#;

        let stacks = parse_crate_stacks(text);
        println!("stacks:{:?}", stacks);

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);
    }

    #[test]
    fn test_do_move() {
        let action = Action {
            time: 2,
            from: 3,
            to: 1,
        };

        let mut stacks = vec![vec!['A'], vec!['B'], vec!['C', 'D']];
        Crane9000.do_move(&mut stacks, &action);

        println!("stacks:{:?}", stacks);
        assert_eq!(stacks[0], vec!['A', 'D', 'C']);
        assert_eq!(stacks[1], vec!['B']);
        assert_eq!(stacks[2], vec![]);

        let mut stacks = vec![vec!['A'], vec!['B'], vec!['C', 'D']];
        Crane9001.do_move(&mut stacks, &action);

        println!("stacks:{:?}", stacks);
        assert_eq!(stacks[0], vec!['A', 'C', 'D']);
        assert_eq!(stacks[1], vec!['B']);
        assert_eq!(stacks[2], vec![]);
    }
}
