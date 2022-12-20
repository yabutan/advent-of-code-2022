use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let r = BufReader::new(File::open("./day18/data/input.txt").unwrap());

    let cubes = Vertex3::from_reader(r).unwrap();
    let total = simulate(&cubes);
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

fn simulate(cubes: &[Vertex3]) -> usize {
    let x_map = make_list_map(cubes, |v| (v.y, v.z), |v| v.x);
    let y_map = make_list_map(cubes, |v| (v.x, v.z), |v| v.y);
    let z_map = make_list_map(cubes, |v| (v.x, v.y), |v| v.z);

    fn cal_surface(map: &HashMap<(u32, u32), Vec<u32>>) -> usize {
        let mut total_surface = 0;
        for list in map.values() {
            let mut covered = 0;

            for i in 0..list.len() - 1 {
                let a = list[i];
                let b = list[i + 1];

                // 隣り合っているかどうか
                if (a + 1) == b {
                    covered += 1;
                }
            }

            total_surface += (list.len() - covered) * 2;
        }
        total_surface
    }

    let mut total_surface = 0;
    total_surface += cal_surface(&x_map);
    total_surface += cal_surface(&y_map);
    total_surface += cal_surface(&z_map);

    total_surface
}

#[derive(Debug)]
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
        let total = simulate(&cubes);
        println!("total: {}", total);
    }
}
