use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    let r = BufReader::new(File::open("./day17/data/sample.txt").unwrap());
    //let r = BufReader::new(File::open("./day17/data/input.txt").unwrap());

    let directions = GasDirection::from_reader(r);
    let mut stage: Stage<7, 10000> = Stage::new(directions);
    for i in 0..1000_000_000_000usize {
        if i % 100_000 == 0 {
            println!("round:{}", i);
        }
        stage.round();
    }
    println!("tall:{}", stage.highest_point + 1)
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
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

struct Stage<const W: usize, const H: usize> {
    data: [[Symbol; W]; H],
    highest_point: i64,
    appear_count: usize,
    gas_count: usize,
    gas_directions: Vec<GasDirection>,
}

impl<const W: usize, const H: usize> Stage<W, H> {
    fn new(gas_directions: Vec<GasDirection>) -> Stage<W, H> {
        Self {
            data: [[Symbol::Air; W]; H],
            highest_point: -1,
            appear_count: 0,
            gas_count: 0,
            gas_directions,
        }
    }

    fn get_data(&self, x: usize, y: usize) -> Symbol {
        if self.highest_point - (H / 2) as i64 > y as i64 {
            unreachable!("invalid access: {}", y);
        }

        let y = y % H;
        self.data[y][x]
    }

    fn set_data(&mut self, x: usize, y: usize, s: Symbol) {
        if self.highest_point - (H / 2) as i64 > y as i64 {
            unreachable!("invalid");
        }

        let y = y % H;
        self.data[y][x] = s;
    }

    // appears and comes to rest
    fn round(&mut self) {
        // appear
        let shape = Shape::from_count(self.appear_count);
        self.appear_count += 1;
        let mut x = 2i64;
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

    fn comes_to_rest(&mut self, shape: &Shape, x: i64, y: i64) {
        // comes to rest
        for pos in &shape.points {
            let x = (x + pos.x) as usize;
            let y = (y + pos.y) as usize;

            self.set_data(x, y, Symbol::Rock);

            if y as i64 > self.highest_point {
                self.highest_point = y as i64;
            }
        }
    }

    fn hit_test(&self, shape: &Shape, x: i64, y: i64) -> bool {
        if x < 0 || W as i64 <= (x + shape.width as i64 - 1) {
            return true;
        }
        if y < 0 {
            return true;
        }

        for pos in &shape.points {
            let x = x + pos.x;
            let y = y + pos.y;

            if self.get_data(x as usize, y as usize) != Symbol::Air {
                return true;
            }
        }
        false
    }

    fn draw(&self, range: RangeInclusive<usize>) {
        for y in range.rev() {
            for x in 0..W {
                let s = self.get_data(x, y);
                let c = match s {
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

        let mut stage: Stage<7, 10000> = Stage::new(directions);
        for i in 0..1000000usize {
            //if i % 100 == 0 {
            println!("-- round {} -- ", i);
            //}
            stage.round();
            //stage.draw(0usize..=20usize);
        }

        println!("tall:{}", stage.highest_point + 1)
    }

    #[test]
    fn test_data() {
        let mut stage: Stage<7, 10> = Stage::new(vec![GasDirection::Left]);

        stage.set_data(0, 0, Symbol::Rock);
        stage.set_data(1, 9, Symbol::Rock);
        stage.set_data(2, 10, Symbol::Rock);
        stage.set_data(3, 11, Symbol::Rock);

        println!("{:?}", stage.get_data(0, 0));
        println!("{:?}", stage.get_data(1, 9));
        println!("{:?}", stage.get_data(2, 10));
        println!("{:?}", stage.get_data(3, 11));

        for (i, x) in stage.data.iter().enumerate() {
            println!("{:03} {:?}", i, x);
        }

        stage.highest_point = 10;

        //println!("{:?}", stage.get_data(0, 0));
        println!("{:?}", stage.get_data(1, 9));
        println!("{:?}", stage.get_data(2, 10));
        println!("{:?}", stage.get_data(3, 11));

        stage.set_data(4, 8, Symbol::Rock);
        stage.set_data(5, 10, Symbol::Rock);
        stage.set_data(6, 19, Symbol::Rock);
        for (i, x) in stage.data.iter().enumerate() {
            println!("{:03} {:?}", i, x);
        }

        println!("{:?}", stage.get_data(4, 8));
        println!("{:?}", stage.get_data(5, 10));
        println!("{:?}", stage.get_data(6, 19));
    }

    #[test]
    fn test_hit_test() {
        let mut stage: Stage<7, 1000> = Stage::new(vec![GasDirection::Left]);

        let shape = Shape::new(ShapeType::A);
        assert!(!stage.hit_test(&shape, 3, 0));
        assert!(stage.hit_test(&shape, 4, 0));
        assert!(stage.hit_test(&shape, -1, 0));

        stage.comes_to_rest(&shape, 2, 0);
        //stage.draw();

        let shape = Shape::new(ShapeType::B);
        assert!(!stage.hit_test(&shape, 2, 1));
        assert!(stage.hit_test(&shape, 2, 0));

        stage.comes_to_rest(&shape, 2, 1);
        //stage.draw();

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
