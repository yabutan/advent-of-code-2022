use num::integer::Roots;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();
    let stage = Stage::read(r);
    if let Some(state) = bfs(&stage) {
        println!("minutes:{}", state.minutes + 1);
    }
}

fn bfs(stage: &Stage) -> Option<State> {
    let mut q: VecDeque<State> = VecDeque::new();
    q.push_back(State::new());

    // 目的地（一番右下）
    let end_point = Point {
        x: stage.w as i32 - 1,
        y: stage.h as i32 - 1,
    };

    // 行ったことある場所は記録
    let mut seen: HashSet<(Point, usize)> = HashSet::new();

    // 進捗わかるように距離と経過時間
    let mut d = i32::MAX;
    let mut m = 0;

    loop {
        let Some(state) = q.pop_front() else {
            break None;
        };
        if state.current == end_point {
            break Some(state);
        }

        // 進捗わかるように最新値を出力
        let new_d = distance(&state.current, &end_point);
        if new_d < d {
            d = new_d;
            println!("distance: {:?} len:{}", d, q.len());
        }
        if m < state.minutes {
            m = state.minutes;
            println!("minutes: {:?} len:{}", m, q.len());
        }

        let minutes = state.minutes + 1;

        state
            .current
            .neighbors()
            .into_iter()
            .filter(|p| 0 <= p.x && p.x < stage.w as i32 && 0 <= p.y && p.y < stage.h as i32)
            .filter(|p| stage.get_chars(minutes, p).is_empty())
            .for_each(|p| {
                let new_state = State {
                    minutes,
                    current: p,
                };

                // 同じ時間で行ったことないところだけを、キューに追加
                if seen.insert((p, minutes)) {
                    q.push_back(new_state);
                }
            });
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Char {
    Air,
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        vec![
            *self,
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
}

#[derive(Debug)]
struct State {
    current: Point,
    minutes: usize,
}

impl State {
    fn new() -> Self {
        State {
            current: Point { x: 0, y: -1 },
            minutes: 0,
        }
    }
}

#[derive(Debug)]
struct Stage {
    w: usize,
    h: usize,
    left_map: Vec<Char>,
    up_map: Vec<Char>,
    right_map: Vec<Char>,
    down_map: Vec<Char>,
}

impl Stage {
    fn get_chars(&self, minutes: usize, pos: &Point) -> Vec<Char> {
        let mut chars = vec![];
        if self.contains(minutes, pos, &Char::Left) {
            chars.push(Char::Left);
        }
        if self.contains(minutes, pos, &Char::Right) {
            chars.push(Char::Right);
        }
        if self.contains(minutes, pos, &Char::Up) {
            chars.push(Char::Up);
        }
        if self.contains(minutes, pos, &Char::Down) {
            chars.push(Char::Down);
        }
        chars
    }

    fn contains(&self, minutes: usize, pos: &Point, c: &Char) -> bool {
        match c {
            Char::Left => {
                let x = (pos.x + minutes as i32).rem_euclid(self.w as i32);
                let y = pos.y;
                let i = (y * self.w as i32) + x;
                self.left_map[i as usize] == *c
            }
            Char::Right => {
                let x = (pos.x - minutes as i32).rem_euclid(self.w as i32);
                let y = pos.y;
                let i = (y * self.w as i32) + x;
                self.right_map[i as usize] == *c
            }
            Char::Up => {
                let x = pos.x;
                let y = (pos.y + minutes as i32).rem_euclid(self.h as i32);
                let i = (y * self.w as i32) + x;
                self.up_map[i as usize] == *c
            }
            Char::Down => {
                let x = pos.x;
                let y = (pos.y - minutes as i32).rem_euclid(self.h as i32);
                let i = (y * self.w as i32) + x;
                self.down_map[i as usize] == *c
            }
            _ => unreachable!("not supported"),
        }
    }

    fn read(r: impl BufRead) -> Self {
        let buffer: Vec<String> = r
            .lines()
            .flatten()
            .map(|x| x[1..x.len() - 1].to_string())
            .collect();
        let buffer = &buffer[1..buffer.len() - 1];

        let h = buffer.len();
        let w = buffer[0].len();

        let mut left_map = Vec::new();
        let mut right_map = Vec::new();
        let mut up_map = Vec::new();
        let mut down_map = Vec::new();

        for line in buffer {
            for c in line.chars() {
                match c {
                    '.' => {
                        left_map.push(Char::Air);
                        right_map.push(Char::Air);
                        up_map.push(Char::Air);
                        down_map.push(Char::Air);
                    }
                    '<' => {
                        left_map.push(Char::Left);
                        right_map.push(Char::Air);
                        up_map.push(Char::Air);
                        down_map.push(Char::Air);
                    }
                    '>' => {
                        left_map.push(Char::Air);
                        right_map.push(Char::Right);
                        up_map.push(Char::Air);
                        down_map.push(Char::Air);
                    }
                    '^' => {
                        left_map.push(Char::Air);
                        right_map.push(Char::Air);
                        up_map.push(Char::Up);
                        down_map.push(Char::Air);
                    }
                    'v' => {
                        left_map.push(Char::Air);
                        right_map.push(Char::Air);
                        up_map.push(Char::Air);
                        down_map.push(Char::Down);
                    }
                    _ => unreachable!("unknown char:{}", c),
                }
            }
        }

        Self {
            w,
            h,
            left_map,
            right_map,
            up_map,
            down_map,
        }
    }
}

fn distance(a: &Point, b: &Point) -> i32 {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chars() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let stage = Stage::read(r);
        assert_eq!(stage.get_chars(0, &Point { x: 2, y: 0 }), vec![]);
        assert_eq!(
            stage.get_chars(1, &Point { x: 2, y: 0 }),
            vec![Char::Left, Char::Right, Char::Down]
        );
        assert_eq!(stage.get_chars(2, &Point { x: 2, y: 0 }), vec![Char::Right]);
        assert_eq!(stage.get_chars(3, &Point { x: 2, y: 0 }), vec![Char::Left]);
        assert_eq!(stage.get_chars(4, &Point { x: 2, y: 0 }), vec![]);
    }

    #[test]
    fn test_contains() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let stage = Stage::read(r);
        assert!(stage.contains(0, &Point { x: 0, y: 0 }, &Char::Right));
        assert!(stage.contains(0, &Point { x: 1, y: 0 }, &Char::Right));
        assert!(!stage.contains(0, &Point { x: 2, y: 0 }, &Char::Right));
        assert!(stage.contains(0, &Point { x: 0, y: 0 }, &Char::Right));
        assert!(!stage.contains(0, &Point { x: 0, y: 1 }, &Char::Right));
        assert!(stage.contains(0, &Point { x: 0, y: 2 }, &Char::Right));
        assert!(!stage.contains(0, &Point { x: 0, y: 3 }, &Char::Right));
    }

    #[test]
    fn test_distance() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 10, y: 10 };
        assert_eq!(distance(&a, &b), 14);
    }
}
