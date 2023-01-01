extern crate core;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{opt, recognize};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day15/data/input.txt")?);

    let data_list = read_data(r);

    let mut find_pos = None;
    for y in 0..=4000000 {
        println!("y: {}", y);
        let mut range_list = Vec::new();
        for data in &data_list {
            if let Some(r) = data.get_range(y) {
                range_list.push(r);
            }
        }

        if let Some(k) = find_space::<4000000>(&range_list) {
            find_pos = Some((k, y));
            break;
        }
    }

    let (x, y) = find_pos.expect("not found");
    let x = x as u64;
    let y = y as u64;

    let tuning_frequency = x * 4000000 + y;
    println!("tuning_frequency: {}", tuning_frequency);

    Ok(())
}

#[derive(Debug)]
struct Data {
    sensor: (i32, i32),
    beacon: (i32, i32),
    distance: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum CheckResult {
    Sensor,
    Beacon,
    Cannot,
    Possible,
}

fn find_space<const W: i32>(range_list: &[RangeInclusive<i32>]) -> Option<i32> {
    for x in range_list {
        let rs = x.start() - 1;
        let re = x.end() + 1;

        if (0..=W).contains(&rs) && !range_list.iter().any(|r| r.contains(&rs)) {
            return Some(rs);
        }

        if (0..=W).contains(&re) && !range_list.iter().any(|r| r.contains(&re)) {
            return Some(re);
        }
    }
    None
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

    fn get_range(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let dy = max(self.sensor.1, y) - min(self.sensor.1, y);
        if self.distance < dy as u32 {
            return None;
        }

        let dx = self.distance as i32 - dy;

        Some((self.sensor.0 - dx)..=(self.sensor.0 + dx))
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
    fn test_find_space() {
        const W: i32 = 400000;

        let r = vec![(0..=400000)];
        assert_eq!(find_space::<W>(&r), None);

        let r = vec![(0..=10), (11..=400000)];
        assert_eq!(find_space::<W>(&r), None);

        let r = vec![(0..=10), (12..=100)];
        assert_eq!(find_space::<W>(&r), Some(11));

        let r = vec![(0..=100), (20..=399999)];
        assert_eq!(find_space::<W>(&r), Some(400000));
    }

    #[test]
    fn test_get_range() {
        let data = Data::new((8, 7), (2, 10));

        assert_eq!(data.get_range(-3), None);
        assert_eq!(data.get_range(-2), Some(8..=8));
        assert_eq!(data.get_range(-1), Some(7..=9));
        assert_eq!(data.get_range(0), Some(6..=10));
        assert_eq!(data.get_range(1), Some(5..=11));
        assert_eq!(data.get_range(2), Some(4..=12));

        assert_eq!(data.get_range(7), Some(-1..=17));

        assert_eq!(data.get_range(16), Some(8..=8));
        assert_eq!(data.get_range(17), None);
    }

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let data_list = read_data(r);

        let mut find_pos = None;

        for y in 0..=20 {
            let mut range_list = Vec::new();
            for data in &data_list {
                if let Some(r) = data.get_range(y) {
                    range_list.push(r);
                }
            }

            if let Some(k) = find_space::<20>(&range_list) {
                find_pos = Some((k, y));
                break;
            }
        }

        let (x, y) = find_pos.unwrap();
        let tuning_frequency = x * 4000000 + y;
        println!("tuning_frequency: {}", tuning_frequency);
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
