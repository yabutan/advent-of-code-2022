use std::collections::HashMap;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{multispace0, multispace1, space1};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use crate::Robot::{ClayRobot, GeodeRobot, ObsidianRobot, OreRobot};

fn main() {
    // TODO: まだ解けてない。
    run_sample1();

    // let r = BufReader::new(File::open("./day19/data/input.txt").unwrap());
    // let input = read_to_string(r).unwrap();
    // let blueprints = Blueprint::parse_input(&input);
    //
    // for blueprint in blueprints {
    //     println!("{:?}", blueprint);
    // }

    //println!("total: {}", total);
}

fn run_sample1() {
    let input = include_str!("../../data/sample.txt");
    let blueprints = Blueprint::parse_input(input);

    let mut results = Vec::new();
    let process = Process::new(&blueprints[0]);
    process.simulate(&State::default(), 24, &mut results);

    // for state in &results {
    //     println!("{:?}", state);
    // }
}

struct Process<'a> {
    blueprint: &'a Blueprint,
}

impl<'a> Process<'a> {
    fn new(blueprint: &'a Blueprint) -> Process<'a> {
        Self { blueprint }
    }

    fn create_robot(&self, state: &State, robot_to_want: &Robot) -> Option<State> {
        let costs = &self.blueprint.costs[robot_to_want];
        let can_create = costs.iter().all(|(resource, need)| {
            let have = state.resources.get(resource).unwrap_or(&0);
            *have >= *need
        });

        if !can_create {
            return None;
        }

        let mut resources = state.resources.clone();
        for (resource, need) in costs {
            let have = resources.get_mut(resource).unwrap();
            *have -= *need;
        }

        let mut robots = state.robots.clone();
        robots
            .entry(robot_to_want.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        Some(State { resources, robots })
    }

    fn simulate(&self, state: &State, left_time: u32, results: &mut Vec<State>) {
        if left_time == 0 {
            println!("{:?}", state);
            results.push(state.clone());
            return;
        }

        // actions
        let mut next_states: Vec<State> = vec![
            self.create_robot(state, &GeodeRobot),
            self.create_robot(state, &ObsidianRobot),
            self.create_robot(state, &ClayRobot),
            self.create_robot(state, &OreRobot),
            Some(state.clone()),
        ]
        .into_iter()
        .flatten()
        .collect();

        for mut state in next_states {
            state.collect();
            self.simulate(&state, left_time - 1, results);
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    robots: HashMap<Robot, u32>,
    resources: HashMap<Resource, u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: vec![OreRobot].into_iter().map(|r| (r, 1)).collect(),
            resources: HashMap::new(),
        }
    }
}

impl State {
    fn collect(&mut self) {
        for (robot, count) in &self.robots {
            match robot {
                OreRobot => {
                    let have = self.resources.entry(Resource::Ore).or_default();
                    *have += count;
                }
                ClayRobot => {
                    let have = self.resources.entry(Resource::Clay).or_default();
                    *have += count;
                }
                ObsidianRobot => {
                    let have = self.resources.entry(Resource::Obsidian).or_default();
                    *have += count;
                }
                GeodeRobot => {
                    let have = self.resources.entry(Resource::Geode).or_default();
                    *have += count;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: HashMap<Robot, Vec<(Resource, u32)>>,
}

impl Blueprint {
    fn parse_input(input: &str) -> Vec<Self> {
        let (_, blueprints) =
            separated_list1(multispace1, parse_blueprint)(input).expect("parse error");
        blueprints
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Robot {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
fn parse_costs(input: &str) -> IResult<&str, (Robot, Vec<(Resource, u32)>)> {
    let cost_parser = separated_pair(
        complete::u32,
        space1,
        alt((
            value(Resource::Ore, tag("ore")),
            value(Resource::Clay, tag("clay")),
            value(Resource::Obsidian, tag("obsidian")),
        )),
    );

    let (input, robot) = delimited(
        tag("Each "),
        alt((
            value(OreRobot, tag("ore robot")),
            value(ClayRobot, tag("clay robot")),
            value(ObsidianRobot, tag("obsidian robot")),
            value(GeodeRobot, tag("geode robot")),
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

    Ok((
        input,
        Blueprint {
            id,
            costs: costs.into_iter().collect(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::Resource::{Clay, Obsidian, Ore};

    use super::*;

    #[test]
    fn test_parse_blueprints() {
        let input = include_str!("../../data/sample.txt");
        let blueprints = Blueprint::parse_input(input);

        assert_eq!(blueprints.len(), 2);

        assert_eq!(blueprints[0].id, 1);
        assert_eq!(blueprints[0].costs[&OreRobot][0], (Ore, 4));
        assert_eq!(blueprints[0].costs[&ClayRobot][0], (Ore, 2));
        assert_eq!(
            blueprints[0].costs[&ObsidianRobot],
            vec![(Ore, 3), (Clay, 14)]
        );

        assert_eq!(blueprints[1].id, 2);
        assert_eq!(blueprints[1].costs[&OreRobot][0], (Ore, 2));
        assert_eq!(blueprints[1].costs[&ClayRobot][0], (Ore, 3));
        assert_eq!(
            blueprints[1].costs[&ObsidianRobot],
            vec![(Ore, 3), (Clay, 8)]
        );
        assert_eq!(
            blueprints[1].costs[&GeodeRobot],
            vec![(Ore, 3), (Obsidian, 12)]
        );
    }
}
