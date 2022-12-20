use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let r = BufReader::new(File::open("./day18/data/input.txt").unwrap());
    let cubes = Vertex3::from_reader(r).unwrap();
    let simulator = Simulator::new(&cubes);
    let total = simulator.simulate();
    println!("total: {}", total);
}

fn make_list_map(
    cubes: &[Vertex3],
    key: fn(&Vertex3) -> (u32, u32),
    value: fn(&Vertex3) -> u32,
) -> HashMap<(u32, u32), Vec<u32>> {
    let mut map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    for cube in cubes {
        let k = key(cube);
        let v = value(cube);
        map.entry(k).or_default().push(v);
    }

    for list in map.values_mut() {
        list.sort();
    }

    map
}

struct AirArea {
    points: HashSet<Vertex3>,
    max: Vertex3,
}

impl AirArea {
    fn new(cubes: &[Vertex3]) -> Self {
        let mut points = HashSet::new();

        let cube_map: HashSet<Vertex3> = cubes.iter().cloned().collect();
        let x = cubes.iter().map(|c| c.x).max().unwrap();
        let y = cubes.iter().map(|c| c.y).max().unwrap();
        let z = cubes.iter().map(|c| c.z).max().unwrap();
        let max = Vertex3 { x, y, z };

        fn find_air(
            pos: &Vertex3,
            max: &Vertex3,
            points: &mut HashSet<Vertex3>,
            cube_map: &HashSet<Vertex3>,
        ) {
            if cube_map.contains(pos) {
                return;
            }
            if points.contains(pos) {
                return;
            }
            points.insert(pos.clone());

            if pos.x > 0 {
                find_air(
                    &Vertex3 {
                        x: pos.x - 1,
                        y: pos.y,
                        z: pos.z,
                    },
                    max,
                    points,
                    cube_map,
                );
            }
            if pos.x < max.x {
                find_air(
                    &Vertex3 {
                        x: pos.x + 1,
                        y: pos.y,
                        z: pos.z,
                    },
                    max,
                    points,
                    cube_map,
                );
            }

            if pos.y > 0 {
                find_air(
                    &Vertex3 {
                        x: pos.x,
                        y: pos.y - 1,
                        z: pos.z,
                    },
                    max,
                    points,
                    cube_map,
                );
            }
            if pos.y < max.y {
                find_air(
                    &Vertex3 {
                        x: pos.x,
                        y: pos.y + 1,
                        z: pos.z,
                    },
                    max,
                    points,
                    cube_map,
                );
            }

            if pos.z > 0 {
                find_air(
                    &Vertex3 {
                        x: pos.x,
                        y: pos.y,
                        z: pos.z - 1,
                    },
                    max,
                    points,
                    cube_map,
                );
            }
            if pos.z < max.z {
                find_air(
                    &Vertex3 {
                        x: pos.x,
                        y: pos.y,
                        z: pos.z + 1,
                    },
                    max,
                    points,
                    cube_map,
                );
            }
        }

        find_air(&Vertex3 { x: 0, y: 0, z: 0 }, &max, &mut points, &cube_map);
        Self { points, max }
    }

    fn is_air(&self, pos: &Vertex3) -> bool {
        self.points.contains(pos)
    }
}

struct Simulator {
    x_map: HashMap<(u32, u32), Vec<u32>>,
    y_map: HashMap<(u32, u32), Vec<u32>>,
    z_map: HashMap<(u32, u32), Vec<u32>>,
    outside_area: AirArea,
}

#[derive(Debug, PartialEq)]
enum Direction {
    X,
    Y,
    Z,
}
impl Simulator {
    fn new(cubes: &[Vertex3]) -> Self {
        let x_map = make_list_map(cubes, |v| (v.y, v.z), |v| v.x);
        let y_map = make_list_map(cubes, |v| (v.x, v.z), |v| v.y);
        let z_map = make_list_map(cubes, |v| (v.x, v.y), |v| v.z);

        let outside_area = AirArea::new(cubes);

        Self {
            x_map,
            y_map,
            z_map,
            outside_area,
        }
    }

    fn simulate(&self) -> usize {
        let mut total_surface = 0;
        total_surface += self.cal_surface(&self.x_map, &Direction::X);
        total_surface += self.cal_surface(&self.y_map, &Direction::Y);
        total_surface += self.cal_surface(&self.z_map, &Direction::Z);

        total_surface
    }

    fn is_air_pocket(&self, p: &Vertex3) -> bool {
        !self.outside_area.is_air(p)
    }

    fn cal_surface(&self, map: &HashMap<(u32, u32), Vec<u32>>, d: &Direction) -> usize {
        let mut total_surface = 0;
        for (pos, list) in map {
            let mut covered = 0;

            for i in 0..list.len() - 1 {
                let a = list[i];
                let b = list[i + 1];

                // 隣り合っているかどうか
                if (a + 1) == b {
                    covered += 1;
                    continue;
                }

                // 空洞内であるかどうか
                let is_air_drop = match d {
                    Direction::X => self.is_air_pocket(&Vertex3 {
                        x: a + 1,
                        y: pos.0,
                        z: pos.1,
                    }),
                    Direction::Y => self.is_air_pocket(&Vertex3 {
                        x: pos.0,
                        y: a + 1,
                        z: pos.1,
                    }),
                    Direction::Z => self.is_air_pocket(&Vertex3 {
                        x: pos.0,
                        y: pos.1,
                        z: a + 1,
                    }),
                };

                if is_air_drop {
                    covered += 1;
                }
            }

            total_surface += (list.len() - covered) * 2;
        }
        total_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vertex3 {
    x: u32,
    y: u32,
    z: u32,
}

impl Vertex3 {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split(',');
        let x: u32 = iter.next().and_then(|x| x.parse().ok())?;
        let y: u32 = iter.next().and_then(|x| x.parse().ok())?;
        let z: u32 = iter.next().and_then(|x| x.parse().ok())?;
        Some(Self { x, y, z })
    }

    fn from_reader(r: impl BufRead) -> anyhow::Result<Vec<Self>> {
        let mut list = Vec::new();
        for line in r.lines() {
            let line = line?;

            let v = match Vertex3::from_str(&line) {
                None => {
                    return Err(anyhow!("Invalid vertex: {}", line));
                }
                Some(it) => it,
            };

            list.push(v);
        }
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let cubes = Vertex3::from_reader(r).unwrap();
        let simulator = Simulator::new(&cubes);
        let total = simulator.simulate();
        assert_eq!(total, 58);
    }

    #[test]
    fn test_is_air_drop() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let cubes = Vertex3::from_reader(r).unwrap();
        let simulator = Simulator::new(&cubes);

        assert!(!simulator.is_air_pocket(&Vertex3::new(0, 0, 0)));
        assert!(simulator.is_air_pocket(&Vertex3::new(2, 2, 5)));
    }
}
