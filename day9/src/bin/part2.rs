use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, value};
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day9/data/input.txt")?);

    let mut rope = Rope::new(10);
    let ret = simulate_visible_tails(r, &mut rope);
    println!("answer: {}", ret.len());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(len: usize) -> Rope {
        assert!(len >= 2);

        let mut knots = Vec::new();
        for _ in 0..len {
            knots.push(Point { x: 0, y: 0 });
        }

        Rope { knots }
    }

    fn visited_tail(&self) -> Option<Point> {
        let head = self.knots.first().unwrap();
        let tail = self.knots.last().unwrap();

        if head == tail {
            return None;
        }

        Some(*tail)
    }

    fn move_to_direction(&mut self, direction: Direction) {
        let mut prev = self.knots[0];
        match direction {
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
        }

        let mut lead = self.knots[0];
        for next in self.knots.iter_mut().skip(1) {
            if prev == *next {
                return;
            }
            if is_touching(&lead, next) {
                // 移動後にHとTが触れている状態なら、そのまま。
                return;
            }

            let new_next = match follow_to_head(&lead, next) {
                Some(it) => it,
                None => prev,
            };

            prev = *next;
            *next = new_next;
            lead = new_next;
        }
    }
}

fn parse_direction(input: &str) -> IResult<&str, (Direction, u32)> {
    let (input, direction) = alt((
        value(Direction::Left, tag("L")),
        value(Direction::Up, tag("U")),
        value(Direction::Right, tag("R")),
        value(Direction::Down, tag("D")),
    ))(input)?;

    let (input, _) = space1(input)?;
    let (input, value) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;

    Ok((input, (direction, value)))
}

fn is_touching(a: &Point, b: &Point) -> bool {
    let dx = if a.x > b.x {
        i32::abs(a.x - b.x)
    } else {
        i32::abs(b.x - a.x)
    };

    let dy = if a.y > b.y {
        i32::abs(a.y - b.y)
    } else {
        i32::abs(b.y - a.y)
    };

    // 距離がXYともに1以内なら隣接していると判定
    dx <= 1 && dy <= 1
}

fn simulate_visible_tails(r: impl BufRead, rope: &mut Rope) -> Vec<Point> {
    let mut list = Vec::new();

    for line in r.lines() {
        let line = line.unwrap();
        let (_, (direction, times)) = parse_direction(&line).expect("parse error");

        for _ in 0..times {
            rope.move_to_direction(direction);
            if let Some(p) = rope.visited_tail() {
                list.push(p);
            }
        }
    }

    // unique list
    list.into_iter().unique().collect()
}

fn follow_to_head(head: &Point, tail: &Point) -> Option<Point> {
    let right = Point {
        x: head.x + 1,
        y: head.y,
    };
    if is_touching(&right, tail) {
        return Some(right);
    }
    let left = Point {
        x: head.x - 1,
        y: head.y,
    };
    if is_touching(&left, tail) {
        return Some(left);
    }

    let up = Point {
        x: head.x,
        y: head.y + 1,
    };
    if is_touching(&up, tail) {
        return Some(up);
    }
    let down = Point {
        x: head.x,
        y: head.y - 1,
    };
    if is_touching(&down, tail) {
        return Some(down);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_follow_to_head() {
        assert_eq!(
            follow_to_head(&Point { x: 1, y: 2 }, &Point { x: 0, y: 0 }),
            Some(Point { x: 1, y: 1 })
        );

        assert_eq!(
            follow_to_head(&Point { x: 2, y: 1 }, &Point { x: 0, y: 0 }),
            Some(Point { x: 1, y: 1 })
        );
    }

    #[test]
    fn test_simulate_visible_tails() {
        let mut rope = Rope::new(2);
        let points =
            simulate_visible_tails(include_str!("../../data/sample.txt").as_bytes(), &mut rope);
        for x in &points {
            println!("{:?}", x);
        }
        assert_eq!(points.len(), 13);
    }

    #[test]
    fn test_sample2() {
        let mut rope = Rope::new(10);
        let points =
            simulate_visible_tails(include_str!("../../data/sample2.txt").as_bytes(), &mut rope);
        for x in &points {
            println!("{:?}", x);
        }
        assert_eq!(points.len(), 36);
    }

    #[test]
    fn test_2() {
        let mut rope = Rope::new(10);

        println!("--- step1");
        rope.move_to_direction(Direction::Right);
        rope.move_to_direction(Direction::Right);
        rope.move_to_direction(Direction::Right);
        rope.move_to_direction(Direction::Right);

        for (i, x) in rope.knots.iter().enumerate() {
            println!("{} {:?}", i, x);
        }

        println!("--- UP 1");
        rope.move_to_direction(Direction::Up);
        for (i, x) in rope.knots.iter().enumerate() {
            println!("{} {:?}", i, x);
        }

        println!("--- UP 2");
        /*
        ......
        ......
        ....H.
        .4321.
        5.....  (5 covers 6, 7, 8, 9, s)
         */
        rope.move_to_direction(Direction::Up);
        for (i, x) in rope.knots.iter().enumerate() {
            println!("{} {:?}", i, x);
        }

        println!("--- UP 3");
        /*
        ......
        ....H.
        ....1.
        .432..
        5.....  (5 covers 6, 7, 8, 9, s)
        */
        rope.move_to_direction(Direction::Up);
        for (i, x) in rope.knots.iter().enumerate() {
            println!("{} {:?}", i, x);
        }

        println!("--- UP 4");
        /*
        ....H.
        ....1.
        ..432.
        .5....
        6.....  (6 covers 7, 8, 9, s)
        */

        rope.move_to_direction(Direction::Up);
        for (i, x) in rope.knots.iter().enumerate() {
            println!("{} {:?}", i, x);
        }
    }

    #[test]
    fn test_parse_direction() {
        assert_eq!(parse_direction("L 1"), Ok(("", (Direction::Left, 1))));
        assert_eq!(parse_direction("U 2"), Ok(("", (Direction::Up, 2))));
        assert_eq!(parse_direction("R 3"), Ok(("", (Direction::Right, 3))));
        assert_eq!(parse_direction("D 4"), Ok(("", (Direction::Down, 4))));
    }

    #[test]
    fn test_is_touching() {
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 0, y: 0 }));

        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 1, y: 0 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: -1, y: 0 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 0, y: 1 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 0, y: -1 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 1, y: 1 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: 1, y: -1 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: -1, y: -1 }));
        assert!(is_touching(&Point { x: 0, y: 0 }, &Point { x: -1, y: 1 }));

        assert!(!is_touching(&Point { x: 0, y: 0 }, &Point { x: 2, y: 0 }));
        assert!(!is_touching(&Point { x: 0, y: 0 }, &Point { x: 2, y: 1 }));
    }

    #[test]
    fn test_move_direction() {
        let mut rope = Rope::new(2);
        rope.move_to_direction(Direction::Right);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Right);
        println!("{:?}", rope);

        assert_eq!(rope.knots[0], Point { x: 2, y: 0 });
        assert_eq!(rope.knots[1], Point { x: 1, y: 0 });

        println!("---");

        let mut rope = Rope::new(2);
        rope.move_to_direction(Direction::Down);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Down);
        println!("{:?}", rope);

        assert_eq!(rope.knots[0], Point { x: 0, y: -2 });
        assert_eq!(rope.knots[1], Point { x: 0, y: -1 });

        println!("---");

        let mut rope = Rope::new(2);
        rope.move_to_direction(Direction::Right);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Up);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Up);
        println!("{:?}", rope);

        assert_eq!(rope.knots[0], Point { x: 1, y: 2 });
        assert_eq!(rope.knots[1], Point { x: 1, y: 1 });

        println!("---");

        let mut rope = Rope::new(2);
        rope.move_to_direction(Direction::Right);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Up);
        println!("{:?}", rope);
        rope.move_to_direction(Direction::Right);
        println!("{:?}", rope);

        assert_eq!(rope.knots[0], Point { x: 2, y: 1 });
        assert_eq!(rope.knots[1], Point { x: 1, y: 1 });
    }
}
