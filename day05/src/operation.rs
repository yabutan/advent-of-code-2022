use std::io::BufRead;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::space1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Operation {
    pub times: u32,
    pub from: u32,
    pub to: u32,
}

pub fn read_operations(r: impl BufRead) -> Vec<Operation> {
    r.lines()
        .flatten()
        .map(|line| {
            let (_, action) = parse_operation(&line).unwrap();
            action
        })
        .collect()
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, times) = preceded(tag("move "), complete::u32)(input)?;
    let (input, _) = space1(input)?;
    let (input, from) = preceded(tag("from "), complete::u32)(input)?;
    let (input, _) = space1(input)?;
    let (input, to) = preceded(tag("to "), complete::u32)(input)?;

    Ok((input, Operation { times, from, to }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let (_, op) = parse_operation("move 1 from 2 to 1").unwrap();
        assert_eq!(
            op,
            Operation {
                times: 1,
                from: 2,
                to: 1
            }
        );

        let (_, op) = parse_operation("move 12 from 9 to 3").unwrap();
        assert_eq!(
            op,
            Operation {
                times: 12,
                from: 9,
                to: 3
            }
        );
    }
}
