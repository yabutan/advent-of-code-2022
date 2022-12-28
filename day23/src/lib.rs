use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

type ElfId = usize;

#[derive(Debug)]
struct Elf {
    id: ElfId,
    pos: Point,
}

#[derive(Debug)]
pub struct State {
    elves: HashMap<ElfId, Elf>,
    round: usize,
}

impl State {
    pub fn read_from(r: impl BufRead) -> Self {
        let elves = read_stage(r);
        Self {
            elves: elves.into_iter().map(|e| (e.id, e)).collect(),
            round: 0,
        }
    }

    pub fn get_round(&self) -> usize {
        self.round
    }

    pub fn count_of_spaces(&self) -> usize {
        let x_min = self.elves.values().map(|e| e.pos.x).min().unwrap();
        let x_max = self.elves.values().map(|e| e.pos.x).max().unwrap();
        let y_min = self.elves.values().map(|e| e.pos.y).min().unwrap();
        let y_max = self.elves.values().map(|e| e.pos.y).max().unwrap();

        let all = (x_max - x_min + 1) * (y_max - y_min + 1);

        all as usize - self.elves.len()
    }

    fn display(&self) -> String {
        let x_min = self.elves.values().map(|e| e.pos.x).min().unwrap();
        let x_max = self.elves.values().map(|e| e.pos.x).max().unwrap();
        let y_min = self.elves.values().map(|e| e.pos.y).min().unwrap();
        let y_max = self.elves.values().map(|e| e.pos.y).max().unwrap();

        // point map
        let point_map: HashMap<Point, ElfId> = self.elves.values().map(|e| (e.pos, e.id)).collect();

        let mut buffer = String::new();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let pos = Point { x, y };
                if point_map.contains_key(&pos) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }
        buffer
    }

    pub fn do_round(&mut self) -> bool {
        // check order
        let checks = match self.round % 4 {
            0 => vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            1 => vec![
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
            ],
            2 => vec![
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
            ],
            3 => vec![
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ],
            _ => unreachable!(),
        };

        // point map
        let point_map: HashMap<Point, ElfId> = self.elves.values().map(|e| (e.pos, e.id)).collect();

        // propose map
        let mut propose_map: HashMap<Point, Vec<ElfId>> = HashMap::new();

        for elf in self.elves.values() {
            // check adjacent
            if elf
                .pos
                .neighbors()
                .iter()
                .all(|p| !point_map.contains_key(p))
            {
                continue;
            }

            // check directions
            for direction in &checks {
                match direction {
                    Direction::North => {
                        if !point_map.contains_key(&Point {
                            x: elf.pos.x,
                            y: elf.pos.y - 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x - 1,
                            y: elf.pos.y - 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x + 1,
                            y: elf.pos.y - 1,
                        }) {
                            propose_map
                                .entry(Point {
                                    x: elf.pos.x,
                                    y: elf.pos.y - 1,
                                })
                                .or_default()
                                .push(elf.id);
                            break;
                        }
                    }
                    Direction::South => {
                        if !point_map.contains_key(&Point {
                            x: elf.pos.x,
                            y: elf.pos.y + 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x - 1,
                            y: elf.pos.y + 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x + 1,
                            y: elf.pos.y + 1,
                        }) {
                            propose_map
                                .entry(Point {
                                    x: elf.pos.x,
                                    y: elf.pos.y + 1,
                                })
                                .or_default()
                                .push(elf.id);
                            break;
                        }
                    }
                    Direction::West => {
                        if !point_map.contains_key(&Point {
                            x: elf.pos.x - 1,
                            y: elf.pos.y,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x - 1,
                            y: elf.pos.y - 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x - 1,
                            y: elf.pos.y + 1,
                        }) {
                            propose_map
                                .entry(Point {
                                    x: elf.pos.x - 1,
                                    y: elf.pos.y,
                                })
                                .or_default()
                                .push(elf.id);
                            break;
                        }
                    }
                    Direction::East => {
                        if !point_map.contains_key(&Point {
                            x: elf.pos.x + 1,
                            y: elf.pos.y,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x + 1,
                            y: elf.pos.y - 1,
                        }) && !point_map.contains_key(&Point {
                            x: elf.pos.x + 1,
                            y: elf.pos.y + 1,
                        }) {
                            propose_map
                                .entry(Point {
                                    x: elf.pos.x + 1,
                                    y: elf.pos.y,
                                })
                                .or_default()
                                .push(elf.id);
                            break;
                        }
                    }
                }
            }
        }

        let mut moved = false;
        for (pos, elves) in propose_map {
            if elves.len() != 1 {
                continue;
            }
            moved = true;

            let elf_id = elves[0];
            self.elves.get_mut(&elf_id).unwrap().pos = pos;
        }

        self.round += 1;

        moved
    }
}

fn read_stage(r: impl BufRead) -> Vec<Elf> {
    let mut elves = Vec::new();

    for (y, line) in r.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elf {
                    id: 1 + elves.len(),
                    pos: Point {
                        x: x as i32,
                        y: y as i32,
                    },
                });
            }
        }
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample2() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let elves = read_stage(r);

        let mut state = State {
            elves: elves.into_iter().map(|e| (e.id, e)).collect(),
            round: 0,
        };

        while state.do_round() {}
        assert_eq!(20, state.round);
    }

    #[test]
    fn test_sample() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let elves = read_stage(r);

        let mut state = State {
            elves: elves.into_iter().map(|e| (e.id, e)).collect(),
            round: 0,
        };

        let text = r"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
"
        .trim_start();
        assert_eq!(text, state.display());

        // End of Round 1
        state.do_round();
        println!("{}", state.display());

        let text = r"
.....#...
...#...#.
.#..#.#..
.....#..#
..#.#.##.
#..#.#...
#.#.#.##.
.........
..#..#...
"
        .trim_start();
        assert_eq!(text, state.display());

        // End of Round 2
        state.do_round();
        let text = r"
......#....
...#.....#.
..#..#.#...
......#...#
..#..#.#...
#...#.#.#..
...........
.#.#.#.##..
...#..#....
"
        .trim_start();
        assert_eq!(text, state.display());

        // End of Round 3
        state.do_round();
        let text = r"
......#....
....#....#.
.#..#...#..
......#...#
..#..#.#...
#..#.....#.
......##...
.##.#....#.
..#........
......#....
"
        .trim_start();
        assert_eq!(text, state.display());

        // End of Round 10
        for _ in 3..10 {
            state.do_round();
        }

        let text = r"
......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..
"
        .trim_start();
        assert_eq!(text, state.display());

        assert_eq!(110, state.count_of_spaces());
    }

    #[test]
    fn test_read_stage() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let elves = read_stage(r);
        assert_eq!(elves.len(), 22);
    }
}
