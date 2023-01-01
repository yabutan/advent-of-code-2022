use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::map;
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;
use std::cmp::{min, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day13/data/input.txt")?);
    let ret = simulate(r);
    println!("answer: {}", ret);
    Ok(())
}

fn simulate(mut r: impl BufRead) -> usize {
    let mut text = String::new();
    r.read_to_string(&mut text).unwrap();

    let (_, mut packets) = parse_text(&text).unwrap();

    // divider packets
    let divider2 = Item::List(vec![Item::List(vec![Item::Value(2)])]);
    let divider6 = Item::List(vec![Item::List(vec![Item::Value(6)])]);
    packets.push(divider2.clone());
    packets.push(divider6.clone());

    packets.sort_by(compare);

    let decoder_key = packets
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if *item == divider2 || *item == divider6 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();

    decoder_key
}

fn parse_item_value(input: &str) -> IResult<&str, Item> {
    let (input, v) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    Ok((input, Item::Value(v)))
}

fn parse_item_list(input: &str) -> IResult<&str, Item> {
    let (input, _) = tag("[")(input)?;
    let (input, list) = separated_list0(tag(","), alt((parse_item_value, parse_item_list)))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Item::List(list)))
}

fn parse_text(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(multispace1, parse_item_list)(input)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

fn compare(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::Value(v1), Item::Value(v2)) => {
            if v1 < v2 {
                return Ordering::Less;
            }
            if v1 > v2 {
                return Ordering::Greater;
            }
            Ordering::Equal
        }
        (Item::List(l1), Item::List(l2)) => {
            let len = min(l1.len(), l2.len());
            for i in 0..len {
                let v1 = &l1[i];
                let v2 = &l2[i];
                match compare(v1, v2) {
                    Ordering::Equal => continue,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                }
            }
            l1.len().cmp(&l2.len())
        }
        (Item::Value(_), Item::List(_)) => {
            let l1 = Item::List(vec![left.clone()]);
            compare(&l1, right)
        }
        (Item::List(_), Item::Value(_)) => {
            let l2 = Item::List(vec![right.clone()]);
            compare(left, &l2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Item::{List, Value};

    #[test]
    fn test_sample() {
        let ret = simulate(include_str!("../../data/sample.txt").as_bytes());
        println!("answer: {}", ret);
    }

    #[test]
    fn test_parse_item_list() {
        let (_, list) = parse_item_list("[1,1,3,1,1]").unwrap();
        assert_eq!(
            list,
            List(vec![Value(1), Value(1), Value(3), Value(1), Value(1)])
        );

        let (_, list) = parse_item_list("[[1],[2,3,4]]").unwrap();
        assert_eq!(
            list,
            List(vec![
                List(vec![Value(1)]),
                List(vec![Value(2), Value(3), Value(4)])
            ])
        );

        let (_, list) = parse_item_list("[[1],4]").unwrap();
        assert_eq!(list, List(vec![List(vec![Value(1)]), Value(4)]));

        let (_, list) = parse_item_list("[]").unwrap();
        assert_eq!(list, List(vec![]));

        let (_, list) = parse_item_list("[3]").unwrap();
        assert_eq!(list, List(vec![Value(3)]));

        let (_, list) = parse_item_list("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        assert_eq!(
            list,
            List(vec![
                Value(1),
                List(vec![
                    Value(2),
                    List(vec![
                        Value(3),
                        List(vec![Value(4), List(vec![Value(5), Value(6), Value(7)])])
                    ])
                ]),
                Value(8),
                Value(9)
            ])
        );
    }

    #[test]
    fn test_make_list() {
        let left = List(vec![Value(1), List(vec![Value(2), Value(3), Value(4)])]);
        println!("{:?}", left);

        let right = List(vec![Value(1), Value(4)]);
        println!("{:?}", right);
    }

    #[test]
    fn test_compare_1() {
        // [1,1,3,1,1] vs [1,1,5,1,1]
        let left = List(vec![Value(1), Value(1), Value(3), Value(1), Value(1)]);
        let right = List(vec![Value(1), Value(1), Value(5), Value(1), Value(1)]);

        assert_eq!(compare(&left, &right), Ordering::Less);
    }
    #[test]
    fn test_compare_2() {
        let left = List(vec![Value(1), List(vec![Value(2), Value(3), Value(4)])]);
        let right = List(vec![List(vec![Value(1)]), Value(4)]);

        assert_eq!(compare(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_compare_3() {
        let left = List(vec![Value(9)]);
        let right = List(vec![List(vec![Value(8), Value(7), Value(6)])]);

        assert_eq!(compare(&left, &right), Ordering::Greater);
    }
}
