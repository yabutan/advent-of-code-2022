extern crate core;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{opt, recognize};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

use day15::CheckResult;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day15/data/input.txt")?);

    let data_list = read_data(r);

    let mut count = 0;

    for x in -5000000..=10000000 {
        let pos = (x, 2000000);
        if check_pos(&data_list, &pos) == CheckResult::Cannot {
            count += 1;
        }
    }
    println!("count: {}", count);

    Ok(())
}

#[derive(Debug)]
struct Data {
    sensor: (i32, i32),
    beacon: (i32, i32),
    distance: u32,
}

fn check_pos(data_list: &[Data], pos: &(i32, i32)) -> CheckResult {
    let mut res = CheckResult::Possible;

    for data in data_list {
        match data.check(pos) {
            CheckResult::Sensor => {
                return CheckResult::Sensor;
            }
            CheckResult::Beacon => {
                return CheckResult::Beacon;
            }
            CheckResult::Cannot => res = CheckResult::Cannot,
            CheckResult::Possible => {}
        }
    }
    res
}

fn calc_distance(&a: &(i32, i32), b: &(i32, i32)) -> u32 {
    let x1 = min(a.0, b.0);
    let x2 = max(a.0, b.0);
    let dx = x2 - x1;

    let y1 = min(a.1, b.1);
    let y2 = max(a.1, b.1);
    let dy = y2 - y1;

    (dx + dy) as u32
}

impl Data {
    fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Self {
        let distance = calc_distance(&sensor, &beacon);
        Self {
            sensor,
            beacon,
            distance,
        }
    }
    fn check(&self, pos: &(i32, i32)) -> CheckResult {
        if self.sensor == *pos {
            return CheckResult::Sensor;
        }
        if self.beacon == *pos {
            return CheckResult::Beacon;
        }

        let d = calc_distance(&self.sensor, pos);
        if d <= self.distance {
            return CheckResult::Cannot;
        }

        CheckResult::Possible
    }
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    let (input, s) = recognize(tuple((opt(tag("-")), digit1)))(input)?;
    let i = s.parse::<i32>().unwrap();
    Ok((input, i))
}
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_line(input: &str) -> IResult<&str, Data> {
    let (input, _) = tag("Sensor at ")(input)?;

    let (input, sensor) = separated_pair(
        preceded(tag("x="), parse_num),
        tag(", "),
        preceded(tag("y="), parse_num),
    )(input)?;

    let (input, _) = tag(": closest beacon is at ")(input)?;

    let (input, beacon) = separated_pair(
        preceded(tag("x="), parse_num),
        tag(", "),
        preceded(tag("y="), parse_num),
    )(input)?;

    Ok((input, Data::new(sensor, beacon)))
}

fn read_data(r: impl BufRead) -> Vec<Data> {
    let mut list = Vec::new();
    for line in r.lines() {
        let line = line.unwrap();
        let (_, data) = parse_line(&line).unwrap();
        list.push(data);
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let data_list = read_data(r);

        for y in 9..=11 {
            print!("{:2} ", y);
            for x in -4..=26 {
                let pos = (x, y);
                match check_pos(&data_list, &pos) {
                    CheckResult::Sensor => {
                        print!("S");
                    }
                    CheckResult::Beacon => {
                        print!("B");
                    }
                    CheckResult::Cannot => {
                        print!("#");
                    }
                    CheckResult::Possible => {
                        print!(".");
                    }
                }
            }
            println!()
        }
    }

    #[test]
    fn test_check_pos() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let data_list = read_data(r);

        let mut count = 0;
        for x in -4..=26 {
            let pos = (x, 10);
            if check_pos(&data_list, &pos) == CheckResult::Cannot {
                count += 1;
            }
        }
        println!("count: {}", count);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_display_cannot() {
        let d = Data::new((8, 7), (2, 10));

        for y in -2..=22 {
            print!("{:2}", y);
            for x in -2..=25 {
                let pos = (x, y);
                let r = d.check(&pos);

                match r {
                    CheckResult::Sensor => {
                        print!("S");
                    }
                    CheckResult::Beacon => {
                        print!("B");
                    }
                    CheckResult::Cannot => {
                        print!("#");
                    }
                    CheckResult::Possible => {
                        print!(".");
                    }
                }
            }
            println!()
        }
    }

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(&(0, 0), &(0, 0)), 0);
        assert_eq!(calc_distance(&(0, 0), &(3, 0)), 3);
        assert_eq!(calc_distance(&(0, 0), &(0, 3)), 3);
        assert_eq!(calc_distance(&(0, 0), &(1, 2)), 3);
        assert_eq!(calc_distance(&(0, 0), &(3, 3)), 6);
        assert_eq!(calc_distance(&(8, 7), &(2, 10)), 9);
    }

    #[test]
    fn test_read_data() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let data_list = read_data(r);

        assert_eq!(data_list.len(), 14);
        assert_eq!(data_list[0].sensor, (2, 18));
        assert_eq!(data_list[13].beacon, (15, 3));
    }

    #[test]
    fn test_parse_line() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let (input, data) = parse_line(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(data.sensor, (2, 18));
        assert_eq!(data.beacon, (-2, 15));
    }
}
