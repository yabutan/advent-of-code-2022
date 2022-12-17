use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::mem;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

// fn main() -> anyhow::Result<()> {
//     let r = BufReader::new(File::open("./day16/data/input.txt")?);
//
//     Ok(())
// }
//
fn main() {
    let r = include_str!("../../data/sample.txt").as_bytes();
    let valves: HashMap<String, Valve> = read_valves(r)
        .into_iter()
        .map(|v| (v.name.clone(), v))
        .collect();

    let openable_count = valves.values().filter(|v| v.rate > 0).count();

    let mut routes = vec![Route {
        path_valves: vec!["AA".to_string()],
        opened_valves: HashSet::new(),
        pressure_released: 0,
    }];

    for minutes in 1..=30 {
        println!("Minute {}", minutes);

        let current_routes = mem::take(&mut routes);

        for mut route in current_routes {
            let valve = &valves[route.path_valves.last().unwrap()];

            // アイテムバルブを合算
            let pressure: u32 = route.opened_valves.iter().map(|v| valves[v].rate).sum();
            route.pressure_released += pressure;

            // すでに全バルブ開けてるなら、何もしない。
            if route.opened_valves.len() == openable_count {
                routes.push(route);
                continue;
            }

            // バルブがあいてないなら、開ける。
            if valve.rate > 0 && !route.opened_valves.contains(&valve.name) {
                route.opened_valves.insert(valve.name.clone());
                routes.push(route);
                continue;
            }

            // 次のフロアへ移動(Routeを増やす)
            valve.leads_to.iter().for_each(|v| {
                let mut new_route = Route {
                    path_valves: route.path_valves.clone(),
                    opened_valves: route.opened_valves.clone(),
                    pressure_released: route.pressure_released,
                };
                new_route.path_valves.push(v.clone());
                routes.push(new_route);
            });
        }
    }

    routes.sort_by_key(|r| r.pressure_released);
    routes.reverse();
    println!("{:?}", routes[0]);
}

#[derive(Debug, Clone)]
struct Route {
    path_valves: Vec<String>,
    opened_valves: HashSet<String>,
    pressure_released: u32,
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    leads_to: Vec<String>,
}

// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve JJ has flow rate=21; tunnel leads to valve II
fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), alpha1)(input)?;
    let (input, rate) = preceded(
        tag(" has flow rate="),
        map(digit1, |s: &str| s.parse().unwrap()),
    )(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, leads_to) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        Valve {
            name: name.to_string(),
            rate,
            leads_to: leads_to.iter().map(|s| s.to_string()).collect(),
        },
    ))
}

fn read_valves(r: impl BufRead) -> Vec<Valve> {
    r.lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| parse_valve(&l).unwrap().1)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem;

    use super::*;

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let valves: HashMap<String, Valve> = read_valves(r)
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect();

        let mut routes = vec![Route {
            path_valves: vec!["AA".to_string()],
            opened_valves: HashSet::new(),
            pressure_released: 0,
        }];

        for minutes in 1..=30 {
            println!("Minute {}", minutes);

            let current_routes = mem::take(&mut routes);

            for mut route in current_routes {
                let valve = &valves[route.path_valves.last().unwrap()];

                // アイテムバルブを合算
                let pressure: u32 = route.opened_valves.iter().map(|v| valves[v].rate).sum();
                route.pressure_released += pressure;

                // バルブがあいてないなら、開ける。
                if valve.rate > 0 && !route.opened_valves.contains(&valve.name) {
                    route.opened_valves.insert(valve.name.clone());
                    routes.push(route);
                    continue;
                }

                // 次のフロアへ移動(Routeを増やす)
                valve.leads_to.iter().for_each(|v| {
                    let mut new_route = Route {
                        path_valves: route.path_valves.clone(),
                        opened_valves: route.opened_valves.clone(),
                        pressure_released: route.pressure_released,
                    };
                    new_route.path_valves.push(v.clone());
                    routes.push(new_route);
                });
            }
        }

        routes.sort_by_key(|r| r.pressure_released);
        routes.reverse();
        println!("{:?}", routes[0]);
    }

    #[test]
    fn test_read_valves() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let valves = read_valves(r);

        assert_eq!(valves.len(), 10);
        assert_eq!(valves[0].name, "AA");
        assert_eq!(valves[0].rate, 0);
        assert_eq!(valves[0].leads_to, vec!["DD", "II", "BB"]);

        assert_eq!(valves[9].name, "JJ");
        assert_eq!(valves[9].rate, 21);
        assert_eq!(valves[9].leads_to, vec!["II"]);
    }

    #[test]
    fn test_parse_valve() {
        let (_, data) =
            parse_valve("Valve BB has flow rate=13; tunnels lead to valves CC, AA").unwrap();
        assert_eq!(data.name, "BB");
        assert_eq!(data.rate, 13);
        assert_eq!(data.leads_to, vec!["CC", "AA"]);

        let (_, data) = parse_valve("Valve JJ has flow rate=21; tunnel leads to valve II").unwrap();
        assert_eq!(data.name, "JJ");
        assert_eq!(data.rate, 21);
        assert_eq!(data.leads_to, vec!["II"]);
    }
}
