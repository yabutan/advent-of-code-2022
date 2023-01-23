use day08::{Grid, GridData};
use itertools::iproduct;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day08/data/input.txt")?);
    let grid = GridData::parse(r);
    let score = find_highest(&grid);
    println!("answer: {score:?}");

    Ok(())
}

fn find_highest(grid: &GridData) -> Option<ScenicScore> {
    iproduct!(0..grid.width(), 0..grid.height())
        .map(|(x, y)| ScenicScore::new(x, y, grid))
        .max_by_key(|s| s.score)
}

#[derive(Debug)]
struct ScenicScore {
    pos_x: usize,
    pos_y: usize,

    /// (left, top, right, bottom)
    counts: (u32, u32, u32, u32),
    score: u32,
}

impl ScenicScore {
    /// (left, top, right, bottom)
    fn new(pos_x: usize, pos_y: usize, grid: &GridData) -> Self {
        let y_len = grid.height();
        let x_len = grid.width();

        let height = grid.get(pos_x, pos_y).unwrap();

        // view right
        let mut count_right = 0;
        for x in (pos_x + 1)..x_len {
            let value = grid.get(x, pos_y).unwrap();
            count_right += 1;
            if value >= height {
                break;
            }
        }

        // view left
        let mut count_left = 0;
        if pos_x > 0 {
            for x in (0..=(pos_x - 1)).rev() {
                let value = grid.get(x, pos_y).unwrap();
                count_left += 1;
                if value >= height {
                    break;
                }
            }
        }

        // view bottom
        let mut count_bottom = 0;
        for y in (pos_y + 1)..y_len {
            let value = grid.get(pos_x, y).unwrap();
            count_bottom += 1;
            if value >= height {
                break;
            }
        }

        // view top
        let mut count_top = 0;
        if pos_y > 0 {
            for y in (0..=(pos_y - 1)).rev() {
                let value = grid.get(pos_x, y).unwrap();
                count_top += 1;
                if value >= height {
                    break;
                }
            }
        }

        let counts = (count_left, count_top, count_right, count_bottom);
        let score = count_left * count_top * count_right * count_bottom;

        Self {
            pos_x,
            pos_y,
            counts,
            score,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scenic_score() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let map = GridData::parse(r);

        let s = ScenicScore::new(2, 1, &map);
        assert_eq!(s.counts, (1, 1, 2, 2));
        assert_eq!(s.score, 4);

        let s = ScenicScore::new(2, 3, &map);
        assert_eq!(s.counts, (2, 2, 2, 1));
        assert_eq!(s.score, 8);
    }
}
