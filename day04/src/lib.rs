use std::io::BufRead;
use std::ops::RangeInclusive;

use anyhow::anyhow;
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

fn parse_pair(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let range = |i| {
        map(
            separated_pair(complete::u32, complete::char('-'), complete::u32),
            |(start, end)| start..=end,
        )(i)
    };

    separated_pair(range, complete::char(','), range)(input)
}

pub fn read_list(
    r: impl BufRead,
) -> anyhow::Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    r.lines()
        .flatten()
        .map(|line| match parse_pair(&line) {
            Ok((_, pair)) => Ok(pair),
            Err(e) => Err(anyhow!("Error parsing line: {:?} {:?}", line, e)),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_list() {
        let list = read_list(include_str!("../data/sample.txt").as_bytes()).unwrap();
        assert_eq!(list.len(), 6);
        assert_eq!(list[0], (2..=4, 6..=8));
        assert_eq!(list[5], (2..=6, 4..=8));
    }
}
