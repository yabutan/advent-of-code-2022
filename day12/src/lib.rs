use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use termion::{color, style};

pub fn simulate(r: impl BufRead, start_edge: bool) -> Option<Vec<Pos>> {
    let mut finder = Finder::with_reader(r, start_edge);

    while finder.do_round() {}

    let state = &finder.states[&finder.end_pos];
    match state {
        State::Certain(path) => {
            finder.display_path();
            Some(path.clone())
        }
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Infinite,
    Tentative(Vec<Pos>),
    Certain(Vec<Pos>),
}

fn to_u32(c: char) -> u32 {
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
}

impl Finder {
    fn with_reader(
        r: impl BufRead,
        start_edge: bool, // for part2
    ) -> Self {
        // heightmap
        let data: Vec<Vec<char>> = r
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();

        // size
        let len_x = data[0].len() as u32;
        let len_y = data.len() as u32;

        // init states
        let (states, end_pos) = {
            let mut end_pos = None;
            let mut states = HashMap::new();
            for y in 0..len_y {
                for x in 0..len_x {
                    let pos = Pos { x, y };
                    let state = match data[y as usize][x as usize] {
                        'S' => State::Tentative(vec![pos]),
                        'a' => {
                            if start_edge && (x == 0 || x + 1 == len_x || y == 0 || y + 1 == len_y)
                            {
                                State::Tentative(vec![pos])
                            } else {
                                State::Infinite
                            }
                        }
                        'E' => {
                            end_pos = Some(pos);
                            State::Infinite
                        }
                        _ => State::Infinite,
                    };
                    states.insert(pos, state);
                }
            }

            (states, end_pos.expect("Failed to find end pos"))
        };

        Self {
            data,
            states,
            len_x,
            len_y,
            end_pos,
        }
    }

    fn get_square(&self, pos: &Pos) -> Vec<Pos> {
        let data = &self.data;

        let current = to_u32(data[pos.y as usize][pos.x as usize]);
        let range = ..=(current + 1);

        let mut candidates = Vec::new();

        if pos.x > 0 {
            let left = to_u32(data[pos.y as usize][pos.x as usize - 1]);
            if range.contains(&left) {
                candidates.push(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
        }

        if pos.x < self.len_x - 1 {
            let right = to_u32(data[pos.y as usize][pos.x as usize + 1]);
            if range.contains(&right) {
                candidates.push(Pos {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
        }

        if pos.y > 0 {
            let up = to_u32(data[pos.y as usize - 1][pos.x as usize]);
            if range.contains(&up) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
        }

        if pos.y < self.len_y - 1 {
            let down = to_u32(data[pos.y as usize + 1][pos.x as usize]);
            if range.contains(&down) {
                candidates.push(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
        }

        candidates
    }

    fn do_round(&mut self) -> bool {
        // 未確定ノードから、一番近いものを選定
        let (pos, path) = {
            let node = self
                .states
                .iter_mut()
                .filter(|(_, state)| matches!(state, State::Tentative(_)))
                .min_by_key(|(_, state)| match state {
                    State::Tentative(path) => path.len(),
                    _ => unreachable!("unreachable"),
                });

            let Some((pos, state)) = node else {
                // 未確定ノードがない場合、探索終了。
                return false;
            };

            // 最短経路として確定する
            let path = match state {
                State::Tentative(path) => path.clone(),
                _ => unreachable!("unreachable"),
            };
            *state = State::Certain(path.clone());
            (*pos, path)
        };

        //  隣接座標を取得
        let candidates = self.get_square(&pos);
        for candidate_pos in candidates {
            match &self.states[&candidate_pos] {
                State::Infinite => {
                    let mut path = path.clone();
                    path.push(pos);
                    self.states.insert(candidate_pos, State::Tentative(path));
                }
                State::Tentative(old_path) => {
                    // 距離が小さければ採用。
                    if path.len() + 1 < old_path.len() {
                        let mut path = path.clone();
                        path.push(pos);
                        self.states.insert(candidate_pos, State::Tentative(path));
                    }
                }
                _ => continue,
            }
        }
        true
    }

    fn display_path(&self) {
        let State::Certain(path) = &self.states[&self.end_pos] else {
            return;
        };

        let path: HashSet<_> = path.iter().collect();

        for y in 0..self.len_y {
            for x in 0..self.len_x {
                let pos = Pos { x, y };
                let color_code = match &(self.states[&pos]) {
                    State::Infinite => format!("{}", color::Fg(color::White)),
                    State::Tentative(_) => format!("{}", color::Fg(color::LightBlue)),
                    State::Certain(_) => {
                        if path.contains(&pos) {
                            format!(
                                "{}{}",
                                color::Bg(color::LightWhite),
                                color::Fg(color::LightGreen),
                            )
                        } else {
                            format!("{}", color::Fg(color::LightGreen))
                        }
                    }
                };

                print!(
                    "{}{}{}",
                    color_code,
                    self.data[y as usize][x as usize],
                    style::Reset
                );
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_simulate_part1() {
        let path = simulate(include_str!("../data/sample.txt").as_bytes(), false).unwrap();
        assert_eq!(path.len() - 1, 31);
    }

    #[test]
    fn test_simulate_part2() {
        let path = simulate(include_str!("../data/sample.txt").as_bytes(), true).unwrap();
        assert_eq!(path.len() - 1, 29);
    }

    #[test]
    fn test_get_square() {
        let height_map = indoc! {r#"
            ryxxl
            szExk
            tuvwj
        "#};

        let finder = Finder::with_reader(height_map.as_bytes(), true);

        let candidates = finder.get_square(&Pos { x: 0, y: 0 });
        assert_eq!(candidates, vec![Pos { x: 0, y: 1 }]);

        let candidates = finder.get_square(&Pos { x: 4, y: 0 });
        assert_eq!(candidates, vec![Pos { x: 4, y: 1 }]);

        let candidates = finder.get_square(&Pos { x: 0, y: 2 });
        assert_eq!(candidates, vec![Pos { x: 1, y: 2 }, Pos { x: 0, y: 1 }]);
    }
}
