use anyhow::anyhow;
use std::io::BufRead;

pub mod simulate;

#[derive(Debug, PartialEq)]
pub enum Direction {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vertex3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Vertex3 {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut iter = s.split(',');
        let x: u32 = iter.next().and_then(|x| x.parse().ok())?;
        let y: u32 = iter.next().and_then(|x| x.parse().ok())?;
        let z: u32 = iter.next().and_then(|x| x.parse().ok())?;
        Some(Self { x, y, z })
    }

    pub fn from_reader(r: impl BufRead) -> anyhow::Result<Vec<Self>> {
        let mut list = Vec::new();
        for line in r.lines() {
            let line = line?;

            let v = match Vertex3::parse(&line) {
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
