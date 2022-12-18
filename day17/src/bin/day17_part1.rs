use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let r = BufReader::new(File::open("./day17/data/input.txt").unwrap());

    let directions = GasDirection::from_reader(r);
    let mut stage: Stage<7> = Stage::new(directions);
    for _ in 0..2022 {
        stage.round();
    }
    println!("tall:{}", stage.highest_point + 1)
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum ShapeType {
    A, // -
    B, // +
    C, // L
    D, // I
    E, // o
}

struct Shape {
    shape_type: ShapeType,
    points: Vec<Point>,
    width: usize,
    height: usize,
}

impl Shape {
    fn new(shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::A => {
                // -
                Shape {
                    shape_type,
                    points: vec![
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 2, y: 0 },
                        Point { x: 3, y: 0 },
                    ],
                    width: 4,
                    height: 1,
                }
            }
            ShapeType::B => {
                // +
                Shape {
                    shape_type,
                    points: vec![
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 1, y: 1 },
                        Point { x: 2, y: 1 },
                        Point { x: 1, y: 2 },
                    ],
                    width: 3,
                    height: 3,
                }
            }
            ShapeType::C => {
                // L
                Shape {
                    shape_type,
                    points: vec![
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 2, y: 0 },
                        Point { x: 2, y: 1 },
                        Point { x: 2, y: 2 },
                    ],
                    width: 3,
                    height: 3,
                }
            }
            ShapeType::D => {
                // I
                Shape {
                    shape_type,
                    points: vec![
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 2 },
                        Point { x: 0, y: 3 },
                    ],
                    width: 1,
                    height: 4,
                }
            }
            ShapeType::E => {
                // o
                Shape {
                    shape_type,
                    points: vec![
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 1, y: 1 },
                    ],
                    width: 2,
                    height: 2,
                }
            }
        }
    }

    fn from_count(count: usize) -> Self {
        match (count) % 5 {
            0 => Self::new(ShapeType::A),
            1 => Self::new(ShapeType::B),
            2 => Self::new(ShapeType::C),
            3 => Self::new(ShapeType::D),
            4 => Self::new(ShapeType::E),
            _ => unreachable!("unexpected"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum GasDirection {
    Left,  // <
    Right, // >
}

impl GasDirection {
    fn from_reader(r: impl BufRead) -> Vec<GasDirection> {
        let mut list = Vec::new();
        for b in r.bytes() {
            let b = b.unwrap();
            list.push(match b {
                b'<' => GasDirection::Left,
                b'>' => GasDirection::Right,
                _ => unreachable!("invalid input"),
            });
        }
        list
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    Air,
    Rock,
}

struct Stage<const W: usize> {
    data: Vec<[Symbol; W]>,
    highest_point: i32,
    appear_count: usize,
    gas_count: usize,
    gas_directions: Vec<GasDirection>,
}

impl<const W: usize> Stage<W> {
    fn new(gas_directions: Vec<GasDirection>) -> Stage<W> {
        Self {
            data: vec![[Symbol::Air; W]; 10],
            highest_point: -1,
            appear_count: 0,
            gas_count: 0,
            gas_directions,
        }
    }

    // appears and comes to rest
    fn round(&mut self) {
        // extend buffer
        if (self.highest_point + 10) as usize > self.data.len() {
            for _ in 0..10 {
                self.data.push([Symbol::Air; W]);
            }
        }

        // appear
        let shape = Shape::from_count(self.appear_count);
        self.appear_count += 1;
        let mut x = 2i32;
        let mut y = self.highest_point + 3 + 1;

        loop {
            // gas flow
            let direction = &self.gas_directions[self.gas_count % self.gas_directions.len()];
            self.gas_count += 1;

            match direction {
                GasDirection::Left => {
                    if !self.hit_test(&shape, x - 1, y) {
                        x -= 1;
                    }
                }
                GasDirection::Right => {
                    let ret = self.hit_test(&shape, x + 1, y);
                    if !ret {
                        x += 1;
                    }
                }
            }

            // falling
            if self.hit_test(&shape, x, y - 1) {
                // comes to rest
                self.comes_to_rest(&shape, x, y);
                break;
            }
            y -= 1;
        }
    }

    fn comes_to_rest(&mut self, shape: &Shape, x: i32, y: i32) {
        // comes to rest
        for pos in &shape.points {
            let x = (x + pos.x) as usize;
            let y = (y + pos.y) as usize;

            self.data[y][x] = Symbol::Rock;

            if y as i32 > self.highest_point {
                self.highest_point = y as i32;
            }
        }
    }

    fn hit_test(&self, shape: &Shape, x: i32, y: i32) -> bool {
        if x < 0 || W as i32 <= (x + shape.width as i32 - 1) {
            return true;
        }
        if y < 0 {
            return true;
        }

        for pos in &shape.points {
            let x = x + pos.x;
            let y = y + pos.y;

            if self.data[y as usize][x as usize] != Symbol::Air {
                return true;
            }
        }
        false
    }

    fn draw(&self) {
        for y in (0..self.data.len()).rev() {
            for symbol in &self.data[y] {
                let c = match symbol {
                    Symbol::Air => '.',
                    Symbol::Rock => '#',
                };
                print!("{}", c);
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let directions =
            GasDirection::from_reader(include_str!("../../data/sample.txt").as_bytes());

        let mut stage: Stage<7> = Stage::new(directions);
        for _ in 0..2022 {
            stage.round();
        }
        println!("tall:{}", stage.highest_point + 1)
    }

    #[test]
    fn test_hit_test() {
        let mut stage: Stage<7> = Stage::new(vec![GasDirection::Left]);

        let shape = Shape::new(ShapeType::A);
        assert!(!stage.hit_test(&shape, 3, 0));
        assert!(stage.hit_test(&shape, 4, 0));
        assert!(stage.hit_test(&shape, -1, 0));

        stage.comes_to_rest(&shape, 2, 0);
        stage.draw();

        let shape = Shape::new(ShapeType::B);
        assert!(!stage.hit_test(&shape, 2, 1));
        assert!(stage.hit_test(&shape, 2, 0));

        stage.comes_to_rest(&shape, 2, 1);
        stage.draw();

        //stage.hit_test(&shape, 2, 0);
    }

    #[test]
    fn test_read_gas_direction() {
        let directions = GasDirection::from_reader(">>><<>".as_bytes());
        assert_eq!(
            directions,
            vec![
                GasDirection::Right,
                GasDirection::Right,
                GasDirection::Right,
                GasDirection::Left,
                GasDirection::Left,
                GasDirection::Right
            ]
        );
    }
}
