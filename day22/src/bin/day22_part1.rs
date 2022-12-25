use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::IResult;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let password = simulate(r);
    println!("password:{:?}", password);
}

fn simulate(r: impl BufRead) -> i32 {
    let stage = Stage::load(r);

    // Initial state
    let mut state = State {
        pos: stage.get_start_point(),
        facing: Direction::Right,
    };

    // Process operations
    for operation in &stage.operations {
        let d = match operation {
            Operation::TurnRight => {
                state.facing = state.facing.clockwise();
                continue;
            }
            Operation::TurnLeft => {
                state.facing = state.facing.counterclockwise();
                continue;
            }
            Operation::Forward(d) => d,
        };

        state.pos = stage.forward(&state.pos, &state.facing, *d);
    }

    // Final password
    calc_final_password(&state)
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
    fn counterclockwise(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    pos: Point,
    facing: Direction,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    TurnRight,
    TurnLeft,
    Forward(u32),
}

#[derive(Debug)]
struct Stage {
    data: Vec<String>,
    range_x: HashMap<usize, Range<usize>>,
    range_y: HashMap<usize, Range<usize>>,
    operations: Vec<Operation>,
    len_x: usize,
    len_y: usize,
}

fn calc_final_password(state: &State) -> i32 {
    let row = state.pos.y + 1;
    let col = state.pos.x + 1;
    let facing = state.facing.clone() as i32;

    (1000 * row) + (4 * col) + facing
}

impl Stage {
    fn load(mut r: impl BufRead) -> Self {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer).expect("read error");

        let fragments: Vec<&str> = buffer.split("\n\n").collect();

        let data: Vec<String> = fragments[0].split('\n').map(|s| s.to_string()).collect();
        let operations = parse_operations(fragments[1]);

        let len_x = data.iter().map(|s| s.len()).max().unwrap();
        let len_y = data.len();

        // x方向のレンジを保持する。
        let mut range_x = HashMap::new();
        for (y, line) in data.iter().enumerate() {
            let start = line.chars().position(|c| c != ' ').unwrap();
            let end = line.len();
            range_x.insert(y, start..end);
        }

        // Y方向のレンジを保持する。
        let mut range_y = HashMap::new();
        for x in 0..len_x {
            let start = (0..len_y)
                .find_map(|y| {
                    if data[y].chars().nth(x)? != ' ' {
                        Some(y)
                    } else {
                        None
                    }
                })
                .unwrap();

            let end = (0..len_y)
                .rev()
                .find_map(|y| {
                    if data[y].chars().nth(x)? != ' ' {
                        Some(y)
                    } else {
                        None
                    }
                })
                .unwrap()
                + 1;

            range_y.insert(x, start..end);
        }

        Self {
            data,
            range_x,
            range_y,
            operations,
            len_x,
            len_y,
        }
    }

    fn get_start_point(&self) -> Point {
        let y = 0;
        let x = self.data[y].chars().position(|c| c == '.').unwrap();

        Point {
            x: x as i32,
            y: y as i32,
        }
    }

    fn get_data(&self, pos: &Point) -> char {
        match self.data[pos.y as usize].chars().nth(pos.x as usize) {
            Some(c) => c,
            None => panic!("illegal pos:{:?}", pos),
        }
    }

    fn forward(&self, current_pos: &Point, direction: &Direction, d: u32) -> Point {
        match direction {
            Direction::Right => {
                let mut x = current_pos.x;
                let range = &self.range_x[&(current_pos.y as usize)];

                for _ in 0..d as i32 {
                    let mut new_x = x + 1;
                    if new_x >= range.end as i32 {
                        new_x = range.start as i32;
                    }

                    if self.get_data(&Point {
                        x: new_x,
                        y: current_pos.y,
                    }) == '#'
                    {
                        break;
                    }
                    x = new_x;
                }

                Point {
                    x,
                    y: current_pos.y,
                }
            }
            Direction::Left => {
                let mut x = current_pos.x;
                let range = &self.range_x[&(current_pos.y as usize)];

                for _ in 0..d as i32 {
                    let mut new_x = x - 1;
                    if new_x < range.start as i32 {
                        new_x = range.end as i32 - 1;
                    }

                    if self.get_data(&Point {
                        x: new_x,
                        y: current_pos.y,
                    }) == '#'
                    {
                        break;
                    }
                    x = new_x;
                }

                Point {
                    x,
                    y: current_pos.y,
                }
            }
            Direction::Down => {
                let mut y = current_pos.y;
                let range = &self.range_y[&(current_pos.x as usize)];

                for _ in 0..d as i32 {
                    let mut new_y = y + 1;
                    if new_y >= range.end as i32 {
                        new_y = range.start as i32;
                    }

                    if self.get_data(&Point {
                        x: current_pos.x,
                        y: new_y,
                    }) == '#'
                    {
                        break;
                    }
                    y = new_y;
                }

                Point {
                    x: current_pos.x,
                    y,
                }
            }
            Direction::Up => {
                let mut y = current_pos.y;
                let range = &self.range_y[&(current_pos.x as usize)];

                for _ in 0..d as i32 {
                    let mut new_y = y - 1;
                    if new_y < range.start as i32 {
                        new_y = range.end as i32 - 1;
                    }

                    if self.get_data(&Point {
                        x: current_pos.x,
                        y: new_y,
                    }) == '#'
                    {
                        break;
                    }
                    y = new_y;
                }

                Point {
                    x: current_pos.x,
                    y,
                }
            }
        }
    }
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(complete::u32, Operation::Forward),
        value(Operation::TurnRight, tag("R")),
        value(Operation::TurnLeft, tag("L")),
    ))(input)
}

fn parse_operations(input: &str) -> Vec<Operation> {
    let (_, list) = many1(parse_operation)(input).expect("parse error");
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        assert_eq!(simulate(r), 6032);
    }

    #[test]
    fn test_cal_final_password() {
        let state = State {
            pos: Point { x: 7, y: 5 },
            facing: Direction::Right,
        };
        assert_eq!(calc_final_password(&state), 6032);
    }

    #[test]
    fn test_forward() {
        let stage = Stage::load(include_str!("../../data/sample.txt").as_bytes());

        let pos = Point { x: 8, y: 0 };
        assert_eq!(
            stage.forward(&pos, &Direction::Right, 1),
            Point { x: 9, y: 0 }
        );
        assert_eq!(
            stage.forward(&pos, &Direction::Right, 2),
            Point { x: 10, y: 0 }
        );
        assert_eq!(
            stage.forward(&pos, &Direction::Right, 3),
            Point { x: 10, y: 0 }
        ); // #に阻まれるため

        let pos = Point { x: 8, y: 1 };
        assert_eq!(
            stage.forward(&pos, &Direction::Left, 1),
            Point { x: 11, y: 1 }
        );
        assert_eq!(
            stage.forward(&pos, &Direction::Left, 2),
            Point { x: 10, y: 1 }
        );
        assert_eq!(
            stage.forward(&pos, &Direction::Left, 3),
            Point { x: 10, y: 1 }
        ); // #に阻まれるため

        let pos = Point { x: 8, y: 0 };
        assert_eq!(stage.forward(&pos, &Direction::Up, 6), Point { x: 8, y: 6 });
        assert_eq!(stage.forward(&pos, &Direction::Up, 7), Point { x: 8, y: 6 }); // #に阻まれるため

        let pos = Point { x: 8, y: 6 };
        assert_eq!(
            stage.forward(&pos, &Direction::Down, 7),
            Point { x: 8, y: 1 }
        );
        assert_eq!(
            stage.forward(&pos, &Direction::Down, 8),
            Point { x: 8, y: 1 }
        ); // #に阻まれるため
    }

    #[test]
    fn test_load() {
        let stage = Stage::load(include_str!("../../data/sample.txt").as_bytes());
        println!("{:?}", stage);
        assert_eq!(stage.len_x, 16);
        assert_eq!(stage.len_y, 12);

        println!("range_x:{:?}", stage.range_x);
        assert_eq!(stage.range_x[&0], 8..12);
        assert_eq!(stage.range_x[&4], 0..12);
        assert_eq!(stage.range_x[&8], 8..16);

        println!("range_y:{:?}", stage.range_y);
        assert_eq!(stage.range_y[&0], 4..8);
        assert_eq!(stage.range_y[&8], 0..12);
        assert_eq!(stage.range_y[&12], 8..12);
    }

    #[test]
    fn test_parse_operations() {
        let list = parse_operations("5L24L19R49");

        assert_eq!(list.len(), 7);
        assert_eq!(list[0], Operation::Forward(5));
        assert_eq!(list[1], Operation::TurnLeft);
        assert_eq!(list[2], Operation::Forward(24));
        assert_eq!(list[3], Operation::TurnLeft);
        assert_eq!(list[4], Operation::Forward(19));
        assert_eq!(list[5], Operation::TurnRight);
        assert_eq!(list[6], Operation::Forward(49));
    }

    #[test]
    fn test_direction() {
        assert_eq!(Direction::Right as u8, 0);
        assert_eq!(Direction::Down as u8, 1);
        assert_eq!(Direction::Left as u8, 2);
        assert_eq!(Direction::Up as u8, 3);
    }
}
