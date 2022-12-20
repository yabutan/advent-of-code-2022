use crate::Direction;
use crate::Vertex3;
use std::collections::{HashMap, HashSet};

pub struct Simulator {
    x_map: HashMap<(u32, u32), Vec<u32>>,
    y_map: HashMap<(u32, u32), Vec<u32>>,
    z_map: HashMap<(u32, u32), Vec<u32>>,
    outside_area: Option<OutsideArea>,
}

impl Simulator {
    pub fn new(cubes: &[Vertex3], considering_air_pockets: bool) -> Self {
        let x_map = Self::make_list_map(cubes, |v| (v.y, v.z), |v| v.x);
        let y_map = Self::make_list_map(cubes, |v| (v.x, v.z), |v| v.y);
        let z_map = Self::make_list_map(cubes, |v| (v.x, v.y), |v| v.z);

        let outside_area = if considering_air_pockets {
            let outside_area = OutsideArea::new(cubes);
            Some(outside_area)
        } else {
            None
        };

        Self {
            x_map,
            y_map,
            z_map,
            outside_area,
        }
    }

    pub fn simulate(&self) -> usize {
        let mut total_surface = 0;
        total_surface += self.cal_surface(&self.x_map, &Direction::X);
        total_surface += self.cal_surface(&self.y_map, &Direction::Y);
        total_surface += self.cal_surface(&self.z_map, &Direction::Z);
        total_surface
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

                let Some(outside_area) = &self.outside_area else {
                    continue;
                };

                // 空洞内であるかどうか
                let is_air_pocket = match d {
                    Direction::X => !outside_area.is_outside(&Vertex3 {
                        x: a + 1,
                        y: pos.0,
                        z: pos.1,
                    }),
                    Direction::Y => !outside_area.is_outside(&Vertex3 {
                        x: pos.0,
                        y: a + 1,
                        z: pos.1,
                    }),
                    Direction::Z => !outside_area.is_outside(&Vertex3 {
                        x: pos.0,
                        y: pos.1,
                        z: a + 1,
                    }),
                };

                if is_air_pocket {
                    covered += 1;
                }
            }

            total_surface += (list.len() - covered) * 2;
        }
        total_surface
    }
}

struct OutsideArea {
    cube_map: HashSet<Vertex3>,
    points: HashSet<Vertex3>,
    max: Vertex3,
}

impl OutsideArea {
    fn new(cubes: &[Vertex3]) -> Self {
        let x = cubes.iter().map(|c| c.x).max().unwrap();
        let y = cubes.iter().map(|c| c.y).max().unwrap();
        let z = cubes.iter().map(|c| c.z).max().unwrap();

        let mut area = Self {
            cube_map: cubes.iter().cloned().collect(),
            points: HashSet::new(),
            max: Vertex3 { x, y, z },
        };
        area.scan();

        area
    }

    fn is_outside(&self, pos: &Vertex3) -> bool {
        self.points.contains(pos)
    }

    fn scan(&mut self) {
        self.find_air(&Vertex3 { x: 0, y: 0, z: 0 });
    }

    fn find_air(&mut self, pos: &Vertex3) {
        if self.cube_map.contains(pos) {
            return;
        }
        if self.points.contains(pos) {
            return;
        }
        self.points.insert(pos.clone());

        if pos.x > 0 {
            self.find_air(&Vertex3 {
                x: pos.x - 1,
                y: pos.y,
                z: pos.z,
            });
        }
        if pos.x < self.max.x {
            self.find_air(&Vertex3 {
                x: pos.x + 1,
                y: pos.y,
                z: pos.z,
            });
        }

        if pos.y > 0 {
            self.find_air(&Vertex3 {
                x: pos.x,
                y: pos.y - 1,
                z: pos.z,
            });
        }
        if pos.y < self.max.y {
            self.find_air(&Vertex3 {
                x: pos.x,
                y: pos.y + 1,
                z: pos.z,
            });
        }

        if pos.z > 0 {
            self.find_air(&Vertex3 {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            });
        }
        if pos.z < self.max.z {
            self.find_air(&Vertex3 {
                x: pos.x,
                y: pos.y,
                z: pos.z + 1,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let cubes = Vertex3::from_reader(r).unwrap();
        let simulator = Simulator::new(&cubes, false);
        let total = simulator.simulate();
        assert_eq!(total, 64);
    }

    #[test]
    fn test_sample_part2() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let cubes = Vertex3::from_reader(r).unwrap();
        let simulator = Simulator::new(&cubes, true);
        let total = simulator.simulate();
        assert_eq!(total, 58);
    }

    #[test]
    fn test_is_outside() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let cubes = Vertex3::from_reader(r).unwrap();

        let outside_area = OutsideArea::new(&cubes);

        assert!(outside_area.is_outside(&Vertex3::new(0, 0, 0)));
        assert!(!outside_area.is_outside(&Vertex3::new(2, 2, 5)));
    }
}
