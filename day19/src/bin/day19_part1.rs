use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{multispace0, multispace1, space1};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::{read_to_string, BufReader, Read};

fn main() {
    let r = BufReader::new(File::open("./day19/data/input.txt").unwrap());
    let input = read_to_string(r).unwrap();
    let blueprints = Blueprint::parse_input(&input);

    for blueprint in blueprints {
        println!("{:?}", blueprint);
    }

    //println!("total: {}", total);
}

fn simulate(blueprint: &Blueprint) -> u32 {
    let mut robots = vec![Robot::OreRobot];
    let mut resources: HashMap<Resource, u32> = HashMap::new();
    for minutes in 1..=24 {
        println!("minute: {}", minutes);

        // collecting
        for robot in &robots {
            match robot {
                Robot::OreRobot => {
                    let a = resources.entry(Resource::Ore).or_default();
                    *a += 1;
                }
                Robot::ClayRobot => {
                    let a = resources.entry(Resource::Clay).or_default();
                    *a += 1;
                }
                Robot::ObsidianRobot => {
                    let a = resources.entry(Resource::Obsidian).or_default();
                    *a += 1;
                }
                Robot::GeodeRobot => {
                    let a = resources.entry(Resource::Geode).or_default();
                    *a += 1;
                }
            }
        }

        println!("resources: {:?}", resources);

        // creating robot
        'creating: for robot in &[
            Robot::GeodeRobot,
            Robot::ObsidianRobot,
            Robot::ClayRobot,
            Robot::OreRobot,
        ] {
            for (resource, need) in &blueprint.costs[robot] {
                // 材料足りてる場合

                // 足りてない場合、下位のロボットを作る

                let have = resources.entry(resource.clone()).or_default();
                if *have > *need {
                    *have -= need;
                    // created geode robot
                    robots.push(robot.clone());
                    println!("created robot: {:?}", robot);
                    break 'creating;
                }
            }
        }
    }

    resources[&Resource::Geode]
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
            value(Robot::OreRobot, tag("ore robot")),
            value(Robot::ClayRobot, tag("clay robot")),
            value(Robot::ObsidianRobot, tag("obsidian robot")),
            value(Robot::GeodeRobot, tag("geode robot")),
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
    use super::*;
    use crate::Resource::{Clay, Obsidian, Ore};
    use crate::Robot::{ClayRobot, GeodeRobot, ObsidianRobot, OreRobot};

    #[test]
    fn test_sample1() {
        let input = include_str!("../../data/sample.txt");
        let blueprints = Blueprint::parse_input(input);

        let ret = simulate(&blueprints[0]);
        println!("ret: {}", ret);
    }

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
