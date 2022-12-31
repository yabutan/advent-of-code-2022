use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{multispace0, multispace1, space1};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use rayon::prelude::*;

use Stone::{Clay, Geode, Obsidian, Ore};

fn main() {
    //let input = include_str!("../../data/sample.txt");
    let input = include_str!("../../data/input.txt");
    let blueprints = Blueprint::parse_input(input);

    let score: i32 = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut max = State::new(0);
            let mut process = Processor::new(blueprint);
            process.dfs(State::new(32), &mut max);

            let best = max.get_resource(&Geode);
            println!("id:{} {:?}", blueprint.id, max);

            best
        })
        .product();

    println!("Score: {}", score);
}

type Key = (i32, i32, i32, i32, i32, i32, i32, i32, i32);

struct Processor<'a> {
    blueprint: &'a Blueprint,
    // この時間で保持している最大値
    // left_time => count
    max_geodes: HashMap<i32, i32>,
    need_ore: i32,
    need_clay: i32,
    need_obsidian: i32,

    cache: HashSet<Key>,
}

impl<'a> Processor<'a> {
    fn new(blueprint: &'a Blueprint) -> Processor<'a> {
        let need_ore = *blueprint
            .costs
            .values()
            .flat_map(|costs| {
                costs.iter().filter_map(
                    |(resource, n)| {
                        if resource == &Ore {
                            Some(n)
                        } else {
                            None
                        }
                    },
                )
            })
            .max()
            .unwrap();

        let need_clay = *blueprint
            .costs
            .values()
            .flat_map(|costs| {
                costs.iter().filter_map(
                    |(resource, n)| {
                        if resource == &Clay {
                            Some(n)
                        } else {
                            None
                        }
                    },
                )
            })
            .max()
            .unwrap();

        let need_obsidian = *blueprint
            .costs
            .values()
            .flat_map(|costs| {
                costs.iter().filter_map(
                    |(resource, n)| {
                        if resource == &Obsidian {
                            Some(n)
                        } else {
                            None
                        }
                    },
                )
            })
            .max()
            .unwrap();

        Self {
            blueprint,
            max_geodes: HashMap::new(),
            need_ore,
            need_clay,
            need_obsidian,
            cache: HashSet::new(),
        }
    }

    fn collect(&self, state: &mut State) {
        let robots = state.robots.clone();
        for (robot, count) in robots {
            state.add_resource(robot, count)
        }

        state.left_time -= 1;
    }

    /// 指定のロボットが作れない場合、Noneを返す。
    /// 作れた場合、は新しいステートを生成して返す。
    fn create_robot(&self, state: &State, robot_to_want: Stone) -> Option<State> {
        let costs = &self.blueprint.costs[&robot_to_want];
        let can_create = costs
            .iter()
            .all(|(resource, need)| state.get_resource(resource) >= *need);

        if !can_create {
            return None;
        }

        // 収集と時間消費
        let mut new_state = state.clone();
        self.collect(&mut new_state);

        // コストを引いて
        for (resource, need) in costs {
            new_state.add_resource(*resource, -*need);
        }

        // ロボットを作る
        new_state
            .robots
            .entry(robot_to_want)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        Some(new_state)
    }

    fn dfs(&mut self, mut state: State, max: &mut State) {
        // 残り時間にたいして、既にGeodeロボットの所持数がすくない場合、処理しない。
        let geode_possible =
            (state.get_robot_count(&Geode) * state.left_time) + state.get_resource(&Geode);

        let max_geode_robots = self.max_geodes.entry(state.left_time).or_insert(0);
        // if (*max_geode_robots / 2) > geode_possible {
        //     return;
        // }

        if *max_geode_robots < geode_possible {
            println!("max updated: id:{} {:?}", self.blueprint.id, state);
            *max_geode_robots = geode_possible;
        }

        if state.left_time == 0 {
            if max.get_resource(&Geode) < state.get_resource(&Geode) {
                *max = state;
            }
            return;
        }

        // actions
        let mut new_states = Vec::new();
        if let Some(new_state) = self.create_robot(&state, Geode) {
            new_states.push(new_state);
        } else {
            if state.get_robot_count(&Obsidian) < self.need_obsidian {
                if let Some(new_state) = self.create_robot(&state, Obsidian) {
                    new_states.push(new_state);
                }
            }
            if state.get_robot_count(&Clay) < self.need_clay {
                if let Some(new_state) = self.create_robot(&state, Clay) {
                    new_states.push(new_state);
                }
            }
            if state.get_robot_count(&Ore) < self.need_ore {
                if let Some(new_state) = self.create_robot(&state, Ore) {
                    new_states.push(new_state);
                }
            }
            {
                self.collect(&mut state);
                new_states.push(state);
            }
        }

        for state in new_states {
            let key = (
                state.left_time,
                state.get_resource(&Geode),
                state.get_resource(&Ore),
                state.get_resource(&Clay),
                state.get_resource(&Obsidian),
                state.get_robot_count(&Geode),
                state.get_robot_count(&Ore),
                state.get_robot_count(&Clay),
                state.get_robot_count(&Obsidian),
            );

            if self.cache.contains(&key) {
                continue;
            }

            self.cache.insert(key);

            self.dfs(state, max);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    robots: HashMap<Stone, i32>,
    resources: HashMap<Stone, i32>,
    left_time: i32,
}

impl State {
    fn new(left_time: i32) -> Self {
        Self {
            robots: vec![Ore].into_iter().map(|r| (r, 1)).collect(),
            resources: HashMap::new(),
            left_time,
        }
    }

    fn add_resource(&mut self, resource: Stone, num: i32) {
        let have = self.resources.entry(resource).or_default();
        *have += num;
    }
    fn get_resource(&self, resource: &Stone) -> i32 {
        *self.resources.get(resource).unwrap_or(&0)
    }

    fn get_robot_count(&self, robot: &Stone) -> i32 {
        *self.robots.get(robot).unwrap_or(&0)
    }
}

impl State {}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: HashMap<Stone, HashMap<Stone, i32>>,
}

impl Blueprint {
    fn parse_input(input: &str) -> Vec<Self> {
        let (_, blueprints) =
            separated_list1(multispace1, parse_blueprint)(input).expect("parse error");
        blueprints
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Stone {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
fn parse_costs(input: &str) -> IResult<&str, (Stone, Vec<(Stone, i32)>)> {
    let cost_parser = separated_pair(
        complete::i32,
        space1,
        alt((
            value(Ore, tag("ore")),
            value(Clay, tag("clay")),
            value(Obsidian, tag("obsidian")),
        )),
    );

    let (input, robot) = delimited(
        tag("Each "),
        alt((
            value(Ore, tag("ore robot")),
            value(Clay, tag("clay robot")),
            value(Obsidian, tag("obsidian robot")),
            value(Geode, tag("geode robot")),
        )),
        tag(" costs "),
    )(input)?;

    let (input, costs) = separated_list1(tag(" and "), cost_parser)(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((
        input,
        (robot, costs.into_iter().map(|(n, r)| (r, n)).collect()),
    ))
}

// Blueprint 1:
//   Each ore robot costs 4 ore.
//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
//   Each geode robot costs 2 ore and 7 obsidian.
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u32, tag(":"))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, costs) = separated_list1(multispace1, parse_costs)(input)?;

    let costs = costs
        .into_iter()
        .map(|(robot, costs)| (robot, costs.into_iter().collect()))
        .collect();

    Ok((input, Blueprint { id, costs }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_robot() {
        let blueprint = Blueprint {
            id: 1,
            costs: vec![
                (Ore, vec![(Ore, 4)].into_iter().collect()),
                (Clay, vec![(Ore, 2)].into_iter().collect()),
                (Obsidian, vec![(Ore, 3), (Clay, 14)].into_iter().collect()),
                (Geode, vec![(Ore, 2), (Obsidian, 7)].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };

        let processor = Processor::new(&blueprint);

        let state = State {
            robots: HashMap::new(),
            resources: vec![(Ore, 10), (Clay, 14)].into_iter().collect(),
            left_time: 10,
        };

        assert_eq!(
            processor.create_robot(&state, Ore),
            Some(State {
                robots: vec![(Ore, 1)].into_iter().collect(),
                resources: vec![(Ore, 6), (Clay, 14)].into_iter().collect(),
                left_time: 9,
            })
        );

        assert_eq!(
            processor.create_robot(&state, Obsidian),
            Some(State {
                robots: vec![(Obsidian, 1)].into_iter().collect(),
                resources: vec![(Ore, 7), (Clay, 0)].into_iter().collect(),
                left_time: 9,
            })
        );

        assert_eq!(processor.create_robot(&state, Geode), None);
    }

    #[test]
    fn test_parse_blueprints() {
        let input = include_str!("../../data/sample.txt");
        let blueprints = Blueprint::parse_input(input);

        assert_eq!(blueprints.len(), 2);

        assert_eq!(blueprints[0].id, 1);
        assert_eq!(blueprints[0].costs[&Ore][&Ore], 4);
        assert_eq!(blueprints[0].costs[&Clay][&Ore], 2);
        assert_eq!(blueprints[0].costs[&Obsidian][&Ore], 3);
        assert_eq!(blueprints[0].costs[&Obsidian][&Clay], 14);

        assert_eq!(blueprints[1].id, 2);
        assert_eq!(blueprints[1].costs[&Ore][&Ore], 2);
        assert_eq!(blueprints[1].costs[&Clay][&Ore], 3);
        assert_eq!(blueprints[1].costs[&Obsidian][&Ore], 3);
        assert_eq!(blueprints[1].costs[&Obsidian][&Clay], 8);
        assert_eq!(blueprints[1].costs[&Geode][&Ore], 3);
        assert_eq!(blueprints[1].costs[&Geode][&Obsidian], 12);
    }
}
