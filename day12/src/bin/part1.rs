use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use termion::{color, style};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day12/data/input.txt")?);
    //let r = BufReader::new(File::open("./day12/data/input2.txt")?);
    let ret = simulate(r).unwrap();
    println!("answer: {}", ret.len() - 1);

    Ok(())
}

fn simulate(r: impl BufRead) -> Option<Vec<Pos>> {
    let mut finder = Finder::with_reader(r);

    loop {
        finder.round();
        if finder.done {
            break;
        }

        // println!(
        //     "{}----------------------{}",
        //     color::Fg(color::Green),
        //     style::Reset,
        // );
        // finder.display_states();
    }

    // finder.states.iter().for_each(|(pos, state)| {
    //     println!("{:?} {:?}", pos, state);
    // });

    //let state = &finder.states[&Pos { x: 0, y: 0 }];
    let state = &finder.states[&finder.end_pos];
    match state {
        State::Fix(n) => {
            finder.display_path();
            Some(n.clone())
        }
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Infinite,
    Temp(Vec<Pos>),
    Fix(Vec<Pos>),
}

fn to_char(c: char) -> u32 {
    match c {
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        _ => c as u32,
    }
}

struct Finder {
    data: Vec<Vec<char>>,
    len_x: u32,
    len_y: u32,

    // 最短距離
    states: HashMap<Pos, State>,
    end_pos: Pos,
    done: bool,
}

impl Finder {
    fn with_reader(r: impl BufRead) -> Self {
        let data: Vec<Vec<char>> = r
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();

        // size
        let len_x = data[0].len() as u32;
        let len_y = data.len() as u32;

        let mut end_pos = None;

        // init states
        let mut states = HashMap::new();
        for y in 0..len_y {
            for x in 0..len_x {
                let pos = Pos { x, y };
                let state = match data[y as usize][x as usize] {
                    'S' => State::Temp(vec![pos]),
                    'E' => {
                        end_pos = Some(pos);
                        State::Infinite
                    }
                    _ => State::Infinite,
                };
                states.insert(pos, state);
            }
        }

        Self {
            data,
            states,
            len_x,
            len_y,
            end_pos: end_pos.unwrap(),
            done: false,
        }
    }

    fn get_square_toward(&self, pos: &Pos) -> Vec<Pos> {
        let data = &self.data;

        let current = to_char(data[pos.y as usize][pos.x as usize]);
        let range = ..=(current + 1);

        let mut candidates = Vec::new();

        if pos.x > 0 {
            let left = to_char(data[pos.y as usize][pos.x as usize - 1]);
            if range.contains(&left) {
                candidates.push(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
        }

        if pos.x < self.len_x - 1 {
            let right = to_char(data[pos.y as usize][pos.x as usize + 1]);
            if range.contains(&right) {
                candidates.push(Pos {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
        }

        if pos.y > 0 {
            let up = to_char(data[pos.y as usize - 1][pos.x as usize]);
            if range.contains(&up) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
        }

        if pos.y < self.len_y - 1 {
            let down = to_char(data[pos.y as usize + 1][pos.x as usize]);
            if range.contains(&down) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
        }

        candidates
    }

    fn get_square(&self, pos: &Pos) -> Vec<Pos> {
        let data = &self.data;

        let current = to_char(data[pos.y as usize][pos.x as usize]);
        let range = (current - 1)..;

        let mut candidates = Vec::new();

        if pos.x > 0 {
            let left = to_char(data[pos.y as usize][pos.x as usize - 1]);
            if range.contains(&left) {
                candidates.push(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
        }

        if pos.x < self.len_x - 1 {
            let right = to_char(data[pos.y as usize][pos.x as usize + 1]);
            if range.contains(&right) {
                candidates.push(Pos {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
        }

        if pos.y > 0 {
            let up = to_char(data[pos.y as usize - 1][pos.x as usize]);
            if range.contains(&up) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
        }

        if pos.y < self.len_y - 1 {
            let down = to_char(data[pos.y as usize + 1][pos.x as usize]);
            if range.contains(&down) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
        }

        candidates
    }

    fn round(&mut self) {
        // 未確定から、短いところを選定
        let (pos, path) = {
            let ret = self
                .states
                .iter_mut()
                .filter(|(_, state)| matches!(state, State::Temp(_)))
                .min_by_key(|(_, state)| match state {
                    State::Temp(path) => path.len(),
                    _ => unreachable!("unreachable"),
                });

            let Some((pos, state)) = ret else {
                self.done = true;
                return;
            };

            // 最小経路として確定
            let path = match state {
                State::Temp(path) => path.clone(),
                _ => unreachable!("unreachable"),
            };
            *state = State::Fix(path.clone());
            (*pos, path)
        };

        //  隣接座標を取得
        //let candidates = self.get_square(&pos);
        let candidates = self.get_square_toward(&pos);
        for x in candidates {
            let state = &self.states[&x];

            match state {
                State::Infinite => {
                    let mut path = path.clone();
                    path.push(x);
                    self.states.insert(x, State::Temp(path));
                }
                State::Temp(n2) => {
                    // 距離が小さければ採用。
                    if path.len() + 1 < n2.len() {
                        let mut path = path.clone();
                        path.push(x);
                        self.states.insert(x, State::Temp(path));
                    }
                }
                _ => continue,
            }
        }
    }

    fn display_states(&self) {
        for y in 0..self.len_y {
            for x in 0..self.len_x {
                let state = self.states.get(&Pos { x, y }).unwrap();

                let s = self.data[y as usize][x as usize].to_string();
                match state {
                    State::Infinite => {
                        print!("{}{}{}", color::Fg(color::White), s, style::Reset);
                    }
                    State::Temp(n) => {
                        print!("{}{}{}", color::Fg(color::LightBlue), s, style::Reset);
                    }
                    State::Fix(_) => {
                        print!("{}{}{}", color::Fg(color::LightGreen), s, style::Reset);
                    }
                };
            }
            println!()
        }
    }

    fn display_path(&self) {
        let a = &self.states[&self.end_pos];

        let path: HashSet<_> = match a {
            State::Fix(path) => path.iter().copied().collect(),
            _ => unreachable!("unreachable"),
        };

        for y in 0..self.len_y {
            for x in 0..self.len_x {
                let state = self.states.get(&Pos { x, y }).unwrap();

                let s = self.data[y as usize][x as usize].to_string();
                match state {
                    State::Infinite => {
                        print!("{}{}{}", color::Fg(color::White), s, style::Reset);
                    }
                    State::Temp(n) => {
                        print!("{}{}{}", color::Fg(color::LightBlue), s, style::Reset);
                    }
                    State::Fix(_) => {
                        if path.contains(&Pos { x, y }) {
                            print!(
                                "{}{}{}{}",
                                color::Bg(color::LightWhite),
                                color::Fg(color::LightGreen),
                                s,
                                style::Reset
                            );
                        } else {
                            print!("{}{}{}", color::Fg(color::LightGreen), s, style::Reset);
                        }
                    }
                };
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let ret = simulate(include_str!("../../data/sample.txt").as_bytes()).unwrap();
        println!("answer: {:?}", ret.len() - 1);
        assert_eq!(ret.len() - 1, 31);
    }

    #[test]
    fn test_get_square() {
        let height_map = r#"
ryxxl
szExk
tuvwj
"#
        .trim();

        let finder = Finder::with_reader(height_map.as_bytes());

        let candidates = finder.get_square(&Pos { x: 2, y: 1 });
        assert_eq!(candidates, vec![Pos { x: 1, y: 1 }]);

        let candidates = finder.get_square(&Pos { x: 4, y: 0 });
        assert_eq!(candidates, vec![Pos { x: 3, y: 0 }, Pos { x: 4, y: 1 }]);

        let candidates = finder.get_square(&Pos { x: 0, y: 2 });
        assert_eq!(candidates, vec![Pos { x: 1, y: 2 }, Pos { x: 0, y: 1 }]);
    }
}
