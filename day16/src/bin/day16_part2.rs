use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

fn main() {
    let r = BufReader::new(File::open("./day16/data/input.txt").unwrap());

    let (a, b) = simulate(r);

    println!("result: {:?}", a);
    println!("result: {:?}", b);
    println!("result: {:?}", a.pressure_released + b.pressure_released);
}

fn simulate(r: impl BufRead) -> (Route, Route) {
    let valves = Valve::from_reader(r);

    let initial_route = Route {
        remain_time: 26,
        path: vec!["AA".to_string()],
        pressure_released: 0,
        opened_valves: HashSet::new(),
    };

    let mut best_pattern = None;

    // 自分とゾウが担当する組み合わせを洗い出し。
    let patterns = make_patterns(&valves);

    for (i, (target1, target2)) in patterns.into_iter().enumerate() {
        // you
        let mut results1: Vec<Route> = Vec::new();
        let searcher = Searcher::new(&valves, target1);
        searcher.search(initial_route.clone(), &mut results1);

        // elephant
        let mut results2: Vec<Route> = Vec::new();
        let searcher = Searcher::new(&valves, target2);
        searcher.search(initial_route.clone(), &mut results2);

        // それぞれのベストを合算した結果を使う。
        let best_route1 = results1.iter().max_by_key(|r| r.pressure_released).unwrap();
        let best_route2 = results2.iter().max_by_key(|r| r.pressure_released).unwrap();

        if best_pattern.is_none() {
            best_pattern = Some((best_route1.clone(), best_route2.clone()));
        } else if let Some((prev_best_route1, prev_best_route2)) = &best_pattern {
            let prev_best_score =
                prev_best_route1.pressure_released + prev_best_route2.pressure_released;

            let new_score = best_route1.pressure_released + best_route2.pressure_released;
            if new_score > prev_best_score {
                println!("{} score:{} {:?}", i, new_score, (best_route1, best_route2));
                best_pattern = Some((best_route1.clone(), best_route2.clone()));
            }
        }
    }

    best_pattern.expect("no pattern")
}

fn make_patterns(valves: &[Valve]) -> Vec<(HashSet<String>, HashSet<String>)> {
    let list: Vec<&str> = valves
        .iter()
        .filter(|v| v.rate > 0)
        .map(|v| v.name.as_str())
        .collect();

    let mut patterns = Vec::new();
    for i in 1..=(list.len() / 2) {
        let ret = list
            .iter()
            .combinations(i)
            .map(|v| v.iter().map(|s| s.to_string()).collect::<HashSet<String>>())
            .collect::<Vec<HashSet<String>>>();

        for x in &ret {
            let targets: HashSet<String> = valves
                .iter()
                .filter(|v| v.rate > 0)
                .filter(|v| !x.contains(&v.name))
                .map(|v| v.name.to_string())
                .collect();

            patterns.push((x.clone(), targets));
        }
    }

    patterns
}

struct Searcher<'a> {
    // label -> valve
    valves: HashMap<String, &'a Valve>,
    // from(label) -> to(label) -> distance
    distance_map: HashMap<String, HashMap<String, u32>>,

    target_valves: HashSet<String>,
}

impl<'a> Searcher<'a> {
    fn new(valves: &'a [Valve], target_valves: HashSet<String>) -> Searcher<'a> {
        let valves: HashMap<String, &Valve> = valves.iter().map(|v| (v.name.clone(), v)).collect();

        // make graph
        let mut graph: Graph<String, ()> = Graph::new();
        // add nodes
        let mut label_to_index: HashMap<String, NodeIndex> = HashMap::new();
        for name in valves.keys() {
            let index = graph.add_node(name.clone());
            label_to_index.insert(name.clone(), index);
        }
        // extend edge
        for (name, valve) in &valves {
            let index = label_to_index[name];
            for lead_to_label in &valve.leads_to {
                graph.update_edge(index, label_to_index[lead_to_label], ());
            }
        }

        let mut distance_map: HashMap<String, HashMap<String, u32>> = HashMap::new();
        for x in &valves {
            let index = label_to_index[x.0];

            // 今いる部屋から各部屋への最短距離を取得
            let node_map = dijkstra(&graph, index, None, |_| 1);

            for (to_index, distance) in node_map {
                distance_map
                    .entry(x.0.clone())
                    .or_default()
                    .insert(graph[to_index].clone(), distance);
            }
        }

        Self {
            valves,
            distance_map,
            target_valves,
        }
    }

    fn find(&self, route: &Route) -> Vec<Route> {
        // 今いる部屋から各部屋への最短距離を取得
        let distance_map = &self.distance_map[route.path.last().unwrap()];
        distance_map
            .iter()
            .filter_map(|(lead_label, distance)| {
                let lead_valve = &self.valves[lead_label];

                if !self.target_valves.contains(lead_label)
                    || route.opened_valves.contains(lead_label)
                {
                    //　ターゲットじゃないか、すでにバルブを開けている部屋は無視。
                    return None;
                }

                // 移動時間と、バルブを開ける時間
                let elapse_time = distance + 1;
                if route.remain_time < elapse_time {
                    // 行っても時間が足りない部屋は無視。
                    return None;
                }

                // 残り時間から解放できる圧力を計算
                let remain_time = route.remain_time - elapse_time;
                let pressure = remain_time * lead_valve.rate;

                let mut route = route.clone();
                route.remain_time = remain_time;
                route.path.push(lead_label.to_string());
                route.opened_valves.insert(lead_label.to_string());
                route.pressure_released += pressure;

                Some(route)
            })
            .collect()
    }

    ///
    /// route: current route
    /// results: result for stock
    fn search(&self, route: Route, results: &mut Vec<Route>) {
        if route.remain_time == 0 {
            // 時間が尽きたら終了
            results.push(route);
            return;
        }
        if self.target_valves == route.opened_valves {
            // 全てのバルブを開けてたら終了
            results.push(route);
            return;
        }

        let new_routes = self.find(&route);
        if new_routes.is_empty() {
            // 進めるルートが一つもない場合は終了
            results.push(route);
            return;
        }

        for route in new_routes {
            // 進めるルートごとに、再帰。
            self.search(route, results);
        }
    }
}

#[derive(Debug, Clone)]
struct Route {
    remain_time: u32,
    path: Vec<String>,
    opened_valves: HashSet<String>,
    pressure_released: u32,
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    leads_to: Vec<String>,
}

impl Valve {
    fn parse(input: &str) -> IResult<&str, Self> {
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
            Self {
                name: name.to_string(),
                rate,
                leads_to: leads_to.iter().map(|s| s.to_string()).collect(),
            },
        ))
    }

    fn from_reader(r: impl BufRead) -> Vec<Self> {
        r.lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty())
            .map(|l| Self::parse(&l).unwrap().1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_patterns() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let valves = Valve::from_reader(r);

        let patterns = make_patterns(&valves);
        println!("pattern len: {}", patterns.len());
        assert_eq!(patterns.len(), 41);
    }

    #[test]
    fn test_sample() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let (a, b) = simulate(r);

        println!("{:?}", a);
        println!("{:?}", b);
        assert_eq!(a.pressure_released + b.pressure_released, 1707);
        assert_eq!(a.path, vec!["AA", "JJ", "BB", "CC"]);
        assert_eq!(b.path, vec!["AA", "DD", "HH", "EE"]);
    }

    #[test]
    fn test_read_valves() {
        let valves = Valve::from_reader(include_str!("../../data/sample.txt").as_bytes());

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
            Valve::parse("Valve BB has flow rate=13; tunnels lead to valves CC, AA").unwrap();
        assert_eq!(data.name, "BB");
        assert_eq!(data.rate, 13);
        assert_eq!(data.leads_to, vec!["CC", "AA"]);

        let (_, data) =
            Valve::parse("Valve JJ has flow rate=21; tunnel leads to valve II").unwrap();
        assert_eq!(data.name, "JJ");
        assert_eq!(data.rate, 21);
        assert_eq!(data.leads_to, vec!["II"]);
    }
}
