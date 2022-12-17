use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let searcher = Searcher::from_reader(r);

    let mut results: Vec<Route> = Vec::new();
    searcher.search(
        30,
        Route {
            path: vec!["AA".to_string()],
            pressure_released: 0,
            opened_valves: HashSet::new(),
        },
        &mut results,
    );

    let best_route = results.iter().max_by_key(|r| r.pressure_released).unwrap();
    println!("result: {:?}", best_route);
}

struct Searcher {
    valves: HashMap<String, Valve>,
    graph: Graph<String, ()>,
    label_to_index: HashMap<String, NodeIndex>,
    max_open_count: usize,
}

impl Searcher {
    fn from_reader(r: impl BufRead) -> Self {
        let valves: HashMap<String, Valve> = Valve::from_reader(r)
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect();

        let max_open_count = valves.values().filter(|v| v.rate > 0).count();

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

        Self {
            valves,
            graph,
            label_to_index,
            max_open_count,
        }
    }

    ///
    /// route: current route
    /// results: result for stock
    fn search(&self, remain_time: u32, route: Route, results: &mut Vec<Route>) {
        if remain_time == 0 {
            // 時間が尽きたら終了
            results.push(route);
            return;
        }
        if self.max_open_count == route.opened_valves.len() {
            // 全てのバルブを開けてたら終了
            results.push(route);
            return;
        }

        // 今いる部屋から各部屋への最短距離を取得
        let current_index = self.label_to_index.get(route.path.last().unwrap()).unwrap();
        let distance_map = dijkstra(&self.graph, *current_index, None, |_| 1);

        let new_routes: Vec<(u32, Route)> = distance_map
            .iter()
            .filter_map(|(index, distance)| {
                let lead_label = &self.graph[*index];
                let lead_valve = &self.valves[lead_label];

                if lead_valve.rate == 0 || route.opened_valves.contains(lead_label) {
                    //　開ける意味がない、または既にバルブを開けている部屋は無視。
                    return None;
                }

                // 移動時間と、バルブを開ける時間
                let elapse_time = distance + 1;
                if remain_time < elapse_time {
                    // 行っても時間が足りない部屋は無視。
                    return None;
                }

                // 残り時間から解放できる圧力を計算
                let remain_time = remain_time - elapse_time;
                let pressure = remain_time * lead_valve.rate;

                let mut route = route.clone();
                route.path.push(lead_label.clone());
                route.opened_valves.insert(lead_label.clone());
                route.pressure_released += pressure;

                Some((remain_time, route))
            })
            .collect();

        if new_routes.is_empty() {
            // 進めるルートが一つもない場合は終了
            results.push(route);
            return;
        }

        for (remain_time, route) in new_routes {
            // 進めるルートごとに、再帰。
            self.search(remain_time, route, results);
        }
    }
}

#[derive(Debug, Clone)]
struct Route {
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
    fn test_sample() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let searcher = Searcher::from_reader(r);

        let mut results: Vec<Route> = Vec::new();
        searcher.search(
            30,
            Route {
                path: vec!["AA".to_string()],
                pressure_released: 0,
                opened_valves: HashSet::new(),
            },
            &mut results,
        );

        let best_route = results.iter().max_by_key(|r| r.pressure_released).unwrap();
        assert_eq!(best_route.pressure_released, 1651);
        assert_eq!(
            best_route.path,
            vec!["AA", "DD", "BB", "JJ", "HH", "EE", "CC"]
        );
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
