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

    // 1面 50マス
    let mut stage = Stage::<50>::load(r);

    // グリッドを定義
    let grid1 = Point { x: 1, y: 0 };
    let grid2 = Point { x: 2, y: 0 };
    let grid3 = Point { x: 1, y: 1 };
    let grid4 = Point { x: 0, y: 2 };
    let grid5 = Point { x: 1, y: 2 };
    let grid6 = Point { x: 0, y: 3 };

    // グリッド同士のつながりを定義
    stage.set_connection(&grid1, &Direction::Up, &grid6, &Direction::Left);
    stage.set_connection(&grid1, &Direction::Left, &grid4, &Direction::Left);
    stage.set_connection(&grid2, &Direction::Up, &grid6, &Direction::Down);
    stage.set_connection(&grid2, &Direction::Right, &grid5, &Direction::Right);
    stage.set_connection(&grid2, &Direction::Down, &grid3, &Direction::Right);
    stage.set_connection(&grid3, &Direction::Left, &grid4, &Direction::Up);
    stage.set_connection(&grid5, &Direction::Down, &grid6, &Direction::Right);

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

        for _ in 0..*d {
            let (new_pos, new_d) = stage.next(&state.pos, &state.facing);
            let c = stage.get_data(&new_pos);
            if c == '#' {
                break;
            }
            state.pos = new_pos;
            state.facing = new_d;
        }
    }

    // Final password
    let password = calc_final_password(&state);
    println!("password:{:?}", password);
}

fn simulate<const SURFACE_SIZE: usize>(r: impl BufRead) -> i32 {
    let stage = Stage::<SURFACE_SIZE>::load(r);

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

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    pos: Point,
    facing: Direction,
}

fn calc_final_password(state: &State) -> i32 {
    let row = state.pos.y + 1;
    let col = state.pos.x + 1;
    let facing = state.facing.clone() as i32;

    (1000 * row) + (4 * col) + facing
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    TurnRight,
    TurnLeft,
    Forward(u32),
}

#[derive(Debug)]
struct Stage<const N: usize> {
    data: Vec<String>,
    range_x: HashMap<usize, Range<usize>>,
    range_y: HashMap<usize, Range<usize>>,
    operations: Vec<Operation>,
    len_x: usize,
    len_y: usize,
    connections: HashMap<(Point, Direction), (Point, Direction)>,
}

impl<const N: usize> Stage<N> {
    fn set_connection(&mut self, ag: &Point, ad: &Direction, bg: &Point, bd: &Direction) {
        self.connections.insert(
            (ag.clone(), ad.clone()),
            (bg.clone(), bd.clockwise().clockwise()),
        );

        self.connections.insert(
            (bg.clone(), bd.clone()),
            (ag.clone(), ad.clockwise().clockwise()),
        );
    }

    fn is_edge(&self, grid: &Point, current_pos: &Point, direction: &Direction) -> bool {
        match direction {
            Direction::Left => grid.x * N as i32 == current_pos.x,
            Direction::Right => (grid.x + 1) * N as i32 - 1 == current_pos.x,
            Direction::Down => (grid.y + 1) * N as i32 - 1 == current_pos.y,
            Direction::Up => grid.y * N as i32 == current_pos.y,
        }
    }

    fn get_edge(&self, grid: &Point, direction: &Direction) -> i32 {
        match direction {
            Direction::Left => grid.x * N as i32,
            Direction::Right => (grid.x + 1) * N as i32 - 1,
            Direction::Down => (grid.y + 1) * N as i32 - 1,
            Direction::Up => grid.y * N as i32,
        }
    }

    fn next(&self, current_pos: &Point, direction: &Direction) -> (Point, Direction) {
        let grid = self.get_grid(current_pos);

        if self.is_edge(&grid, current_pos, direction) {
            if let Some((connection_grid, new_direction)) =
                self.connections.get(&(grid.clone(), direction.clone()))
            {
                let pos = match (direction, new_direction) {
                    (Direction::Left, Direction::Down) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = connection_grid.x * N as i32 + d;
                        let y = connection_grid.y * N as i32;
                        Point { x, y }
                    }
                    (Direction::Up, Direction::Right) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = connection_grid.x * N as i32;
                        let y = connection_grid.y * N as i32 + d;
                        Point { x, y }
                    }
                    (Direction::Up, Direction::Left) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - 1;
                        let y = (connection_grid.y + 1) * N as i32 - d - 1;
                        Point { x, y }
                    }
                    (Direction::Up, Direction::Up) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = connection_grid.x * N as i32 + d;
                        let y = (connection_grid.y + 1) * N as i32 - 1;
                        Point { x, y }
                    }
                    (Direction::Up, Direction::Down) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - d - 1;
                        let y = connection_grid.y * N as i32;
                        Point { x, y }
                    }
                    (Direction::Down, Direction::Up) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - d - 1;
                        let y = (connection_grid.y + 1) * N as i32 - 1;
                        Point { x, y }
                    }
                    (Direction::Down, Direction::Down) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = connection_grid.x * N as i32 + d;
                        let y = connection_grid.y * N as i32;
                        Point { x, y }
                    }
                    (Direction::Down, Direction::Right) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = connection_grid.x * N as i32;
                        let y = (connection_grid.y + 1) * N as i32 - d - 1;
                        Point { x, y }
                    }
                    (Direction::Down, Direction::Left) => {
                        let d = current_pos.x - (grid.x * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - 1;
                        let y = connection_grid.y * N as i32 + d;
                        Point { x, y }
                    }
                    (Direction::Right, Direction::Left) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - 1;
                        let y = (connection_grid.y + 1) * N as i32 - d - 1;
                        Point { x, y }
                    }
                    (Direction::Right, Direction::Down) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - d - 1;
                        let y = connection_grid.y * N as i32;
                        Point { x, y }
                    }
                    (Direction::Right, Direction::Up) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = connection_grid.x * N as i32 + d;
                        let y = (connection_grid.y + 1) * N as i32 - 1;
                        Point { x, y }
                    }
                    (Direction::Left, Direction::Right) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = connection_grid.x * N as i32;
                        let y = (connection_grid.y + 1) * N as i32 - d - 1;
                        Point { x, y }
                    }
                    (Direction::Left, Direction::Up) => {
                        let d = current_pos.y - (grid.y * N as i32);
                        let x = (connection_grid.x + 1) * N as i32 - d - 1;
                        let y = (connection_grid.y + 1) * N as i32 - 1;
                        Point { x, y }
                    }
                    _ => unimplemented!("{:?}", (direction, new_direction)),
                };

                return (pos, new_direction.clone());
            }
        }

        let pos = match direction {
            Direction::Right => {
                let mut x = current_pos.x;
                let range = &self.range_x[&(current_pos.y as usize)];

                let mut new_x = x + 1;
                if new_x >= range.end as i32 {
                    new_x = range.start as i32;
                }
                x = new_x;

                Point {
                    x,
                    y: current_pos.y,
                }
            }
            Direction::Left => {
                let mut x = current_pos.x;
                let range = &self.range_x[&(current_pos.y as usize)];

                let mut new_x = x - 1;
                if new_x < range.start as i32 {
                    new_x = range.end as i32 - 1;
                }
                x = new_x;

                Point {
                    x,
                    y: current_pos.y,
                }
            }
            Direction::Down => {
                let mut y = current_pos.y;
                let range = &self.range_y[&(current_pos.x as usize)];

                let mut new_y = y + 1;
                if new_y >= range.end as i32 {
                    new_y = range.start as i32;
                }
                y = new_y;

                Point {
                    x: current_pos.x,
                    y,
                }
            }
            Direction::Up => {
                let mut y = current_pos.y;
                let range = &self.range_y[&(current_pos.x as usize)];

                let mut new_y = y - 1;
                if new_y < range.start as i32 {
                    new_y = range.end as i32 - 1;
                }
                y = new_y;

                Point {
                    x: current_pos.x,
                    y,
                }
            }
        };

        (pos, direction.clone())
    }

    fn get_grid(&self, pos: &Point) -> Point {
        let x = pos.x / N as i32;
        let y = pos.y / N as i32;
        Point { x, y }
    }

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
            connections: HashMap::new(),
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
    fn test_modifier() {
        let grid1 = Point { x: 2, y: 0 };
        let grid2 = Point { x: 0, y: 1 };
        let grid3 = Point { x: 1, y: 1 };
        let grid4 = Point { x: 2, y: 1 };
        let grid5 = Point { x: 2, y: 2 };
        let grid6 = Point { x: 3, y: 2 };

        let mut stage = Stage::<4>::load(include_str!("../../data/sample.txt").as_bytes());
        stage.set_connection(&grid1, &Direction::Left, &grid3, &Direction::Up);
        stage.set_connection(&grid1, &Direction::Up, &grid2, &Direction::Up);
        stage.set_connection(&grid1, &Direction::Right, &grid6, &Direction::Right);
        stage.set_connection(&grid2, &Direction::Down, &grid5, &Direction::Down);
        stage.set_connection(&grid2, &Direction::Left, &grid6, &Direction::Down);
        stage.set_connection(&grid3, &Direction::Down, &grid5, &Direction::Left);
        stage.set_connection(&grid4, &Direction::Right, &grid6, &Direction::Up);

        // 1 -> 3
        let (pos, d) = stage.next(&Point { x: 8, y: 0 }, &Direction::Left);
        assert_eq!(pos, Point { x: 4, y: 4 });
        assert_eq!(d, Direction::Down);
        // 3 -> 1
        let (pos, d) = stage.next(&Point { x: 4, y: 4 }, &Direction::Up);
        assert_eq!(pos, Point { x: 8, y: 0 });
        assert_eq!(d, Direction::Right);

        // 1 -> 2
        let (pos, d) = stage.next(&Point { x: 8, y: 0 }, &Direction::Up);
        assert_eq!(pos, Point { x: 3, y: 4 });
        assert_eq!(d, Direction::Down);
        // 2 -> 1
        let (pos, d) = stage.next(&Point { x: 0, y: 4 }, &Direction::Up);
        assert_eq!(pos, Point { x: 11, y: 0 });
        assert_eq!(d, Direction::Down);

        // 1 -> 6
        let (pos, d) = stage.next(&Point { x: 11, y: 0 }, &Direction::Right);
        assert_eq!(pos, Point { x: 15, y: 11 });
        assert_eq!(d, Direction::Left);
        // 6 -> 1
        let (pos, d) = stage.next(&Point { x: 15, y: 8 }, &Direction::Right);
        assert_eq!(pos, Point { x: 11, y: 3 });
        assert_eq!(d, Direction::Left);

        // 2 -> 5
        let (pos, d) = stage.next(&Point { x: 3, y: 7 }, &Direction::Down);
        assert_eq!(pos, Point { x: 8, y: 11 });
        assert_eq!(d, Direction::Up);
        // 5 -> 2
        let (pos, d) = stage.next(&Point { x: 8, y: 11 }, &Direction::Down);
        assert_eq!(pos, Point { x: 3, y: 7 });
        assert_eq!(d, Direction::Up);

        // 2 -> 6
        let (pos, d) = stage.next(&Point { x: 0, y: 4 }, &Direction::Left);
        assert_eq!(pos, Point { x: 15, y: 11 });
        assert_eq!(d, Direction::Up);
        // 6 -> 2
        let (pos, d) = stage.next(&Point { x: 12, y: 11 }, &Direction::Down);
        assert_eq!(pos, Point { x: 0, y: 7 });
        assert_eq!(d, Direction::Right);

        // 3 -> 5
        let (pos, d) = stage.next(&Point { x: 4, y: 7 }, &Direction::Down);
        assert_eq!(pos, Point { x: 8, y: 11 });
        assert_eq!(d, Direction::Right);
        // 5 -> 3
        let (pos, d) = stage.next(&Point { x: 8, y: 11 }, &Direction::Left);
        assert_eq!(pos, Point { x: 4, y: 7 });
        assert_eq!(d, Direction::Up);

        // 4 -> 6
        let (pos, d) = stage.next(&Point { x: 11, y: 4 }, &Direction::Right);
        assert_eq!(pos, Point { x: 15, y: 8 });
        assert_eq!(d, Direction::Down);
        // 6 -> 4
        let (pos, d) = stage.next(&Point { x: 15, y: 8 }, &Direction::Up);
        assert_eq!(pos, Point { x: 11, y: 4 });
        assert_eq!(d, Direction::Left);
    }

    #[test]
    fn test_grid() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let stage = Stage::<4>::load(r);

        assert_eq!(stage.get_grid(&Point { x: 0, y: 0 }), Point { x: 0, y: 0 });
        assert_eq!(stage.get_grid(&Point { x: 4, y: 0 }), Point { x: 1, y: 0 });
        assert_eq!(stage.get_grid(&Point { x: 0, y: 4 }), Point { x: 0, y: 1 });
        assert_eq!(stage.get_grid(&Point { x: 8, y: 8 }), Point { x: 2, y: 2 });
    }

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let mut stage = Stage::<4>::load(r);

        let grid1 = Point { x: 2, y: 0 };
        let grid2 = Point { x: 0, y: 1 };
        let grid3 = Point { x: 1, y: 1 };
        let grid4 = Point { x: 2, y: 1 };
        let grid5 = Point { x: 2, y: 2 };
        let grid6 = Point { x: 3, y: 2 };

        stage.set_connection(&grid1, &Direction::Left, &grid3, &Direction::Up);
        stage.set_connection(&grid1, &Direction::Up, &grid2, &Direction::Up);
        stage.set_connection(&grid1, &Direction::Right, &grid6, &Direction::Right);
        stage.set_connection(&grid2, &Direction::Down, &grid5, &Direction::Down);
        stage.set_connection(&grid2, &Direction::Left, &grid6, &Direction::Down);
        stage.set_connection(&grid3, &Direction::Down, &grid5, &Direction::Left);
        stage.set_connection(&grid4, &Direction::Right, &grid6, &Direction::Up);

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

            for _ in 0..*d {
                let (new_pos, new_d) = stage.next(&state.pos, &state.facing);
                let c = stage.get_data(&new_pos);
                if c == '#' {
                    break;
                }
                state.pos = new_pos;
                state.facing = new_d;
            }
        }

        // Final password
        assert_eq!(calc_final_password(&state), 5031);
    }

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        assert_eq!(simulate::<4>(r), 6032);
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
        let stage = Stage::<4>::load(include_str!("../../data/sample.txt").as_bytes());

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
        let stage = Stage::<4>::load(include_str!("../../data/sample.txt").as_bytes());
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
