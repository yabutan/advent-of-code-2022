use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let text = read_to_string("./day11/data/input.txt")?;
    let (_, mut monkeys) = parse_monkeys(&text).unwrap();

    // 最小公倍数
    let k = get_lcm(&monkeys);

    let mut inspect_counts = vec![0; monkeys.len()];
    for i in 0..10000 {
        do_round(&mut monkeys, &mut inspect_counts, k);
        if matches!(
            i + 1,
            1 | 20 | 1000 | 2000 | 3000 | 4000 | 5000 | 6000 | 7000 | 8000 | 9000 | 10000
        ) {
            println!("count:{} inspect_counts:{:?}", i + 1, inspect_counts);
        }
    }

    let monkey_business: u64 = inspect_counts.iter().sorted().rev().take(2).product();
    println!("answer: {}", monkey_business);

    Ok(())
}

fn do_round(monkeys: &mut [Monkey], inspect_counts: &mut [u64], k: u32) {
    let mut tmp_items: HashMap<usize, Vec<u64>> = HashMap::new();

    for monkey in monkeys.iter_mut() {
        if let Some(items) = tmp_items.get_mut(&monkey.id) {
            monkey.items.append(items);
        }

        loop {
            if monkey.items.is_empty() {
                break;
            }

            inspect_counts[monkey.id] += 1;
            let mut item = monkey.items.remove(0);

            //println!(" item:{}", item);

            match monkey.operation {
                (Operation::Add, Operand::Old) => item += item,
                (Operation::Add, Operand::Value(v)) => item += v as u64,
                (Operation::Multiply, Operand::Old) => {
                    item *= item;
                }
                (Operation::Multiply, Operand::Value(v)) => {
                    item *= v as u64;
                }
            };

            item %= k as u64;

            let to_monkey_id = if item % monkey.divisible as u64 == 0 {
                monkey.id_if_true
            } else {
                monkey.id_if_false
            };

            tmp_items.entry(to_monkey_id).or_default().push(item);
        }
    }

    for (monkey_id, items) in tmp_items {
        monkeys[monkey_id].items.extend(items);
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: (Operation, Operand),
    divisible: u32,
    id_if_true: usize,
    id_if_false: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
enum Operand {
    Old,
    Value(u32),
}

// old * 19
fn parse_operation(input: &str) -> IResult<&str, (Operation, Operand)> {
    let (input, _) = tag("old ")(input)?;

    let (input, op) = alt((
        value(Operation::Add, tag("+")),
        value(Operation::Multiply, tag("*")),
    ))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, operand) = alt((
        value(Operand::Old, tag("old")),
        map(digit1, |s: &str| Operand::Value(s.parse().unwrap())),
    ))(input)?;

    Ok((input, (op, operand)))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(multispace1, parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // Monkey 0:
    let (input, id) = map(delimited(tag("Monkey "), digit1, tag(":")), |s: &str| {
        s.parse::<usize>().unwrap()
    })(input)?;
    let (input, _) = multispace1(input)?;

    // Starting items: 79, 98
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), map(digit1, |s: &str| s.parse::<u64>().unwrap())),
    )(input)?;
    let (input, _) = multispace1(input)?;

    // Operation: new = old * 19
    let (input, operation) = preceded(tag("Operation: new = "), parse_operation)(input)?;
    let (input, _) = multispace1(input)?;

    // Test: divisible by 23
    let (input, divisible) = preceded(
        tag("Test: divisible by "),
        map(digit1, |s: &str| s.parse::<u32>().unwrap()),
    )(input)?;
    let (input, _) = multispace1(input)?;

    // If true: throw to monkey 2
    let (input, id_if_true) = preceded(
        tag("If true: throw to monkey "),
        map(digit1, |s: &str| s.parse::<usize>().unwrap()),
    )(input)?;
    let (input, _) = multispace1(input)?;

    // If false: throw to monkey 3
    let (input, id_if_false) = preceded(
        tag("If false: throw to monkey "),
        map(digit1, |s: &str| s.parse::<usize>().unwrap()),
    )(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            divisible,
            id_if_true,
            id_if_false,
        },
    ))
}

fn get_lcm(monkeys: &[Monkey]) -> u32 {
    let mut k = monkeys[0].divisible;
    for x in monkeys.iter().skip(1) {
        let b = x.divisible;
        k = num::integer::lcm(k, b);
    }
    k
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_do_round() {
        let text = include_str!("../../data/sample.txt");
        let (_, mut monkeys) = parse_monkeys(text).unwrap();

        // 最小公倍数
        let k = get_lcm(&monkeys);

        let mut inspect_counts = vec![0; monkeys.len()];
        for i in 0..10000 {
            do_round(&mut monkeys, &mut inspect_counts, k);
            if matches!(
                i + 1,
                1 | 20 | 1000 | 2000 | 3000 | 4000 | 5000 | 6000 | 7000 | 8000 | 9000 | 10000
            ) {
                println!("count:{} inspect_counts:{:?}", i + 1, inspect_counts);
            }
        }

        let monkey_business: u64 = inspect_counts.iter().sorted().rev().take(2).product();
        assert_eq!(monkey_business, 2713310158);
    }

    #[test]
    fn test_parse_monkeys() {
        let text = include_str!("../../data/sample.txt");
        let (_, monkeys) = parse_monkeys(text).unwrap();

        for x in &monkeys {
            println!("{:?}", x);
        }
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_parse_monkey() {
        let text = indoc! {r#"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
        "#};

        let (_, monkey) = parse_monkey(text).unwrap();
        assert_eq!(
            monkey,
            Monkey {
                id: 0,
                items: vec![79, 98,],
                operation: (Operation::Multiply, Operand::Value(19,),),
                divisible: 23,
                id_if_true: 2,
                id_if_false: 3,
            }
        );

        let text = indoc! {r#"
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3 
        "#};

        let (_, monkey) = parse_monkey(text).unwrap();

        assert_eq!(
            monkey,
            Monkey {
                id: 2,
                items: vec![79, 60, 97,],
                operation: (Operation::Multiply, Operand::Old,),
                divisible: 13,
                id_if_true: 1,
                id_if_false: 3,
            }
        );
    }
}
