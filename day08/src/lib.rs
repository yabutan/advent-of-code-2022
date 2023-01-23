use std::io::BufRead;

pub trait Grid {
    fn get(&self, x: usize, y: usize) -> Option<&u32>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct GridData {
    width: usize,
    height: usize,
    cells: Vec<Vec<u32>>,
}

impl Grid for GridData {
    fn get(&self, x: usize, y: usize) -> Option<&u32> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl GridData {
    pub fn parse(r: impl BufRead) -> Self {
        let cells: Vec<Vec<u32>> = r
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        let width = cells[0].len();
        let height = cells.len();

        Self {
            width,
            height,
            cells,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_map() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let grid = GridData::parse(r);

        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);

        assert_eq!(grid.cells[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(grid.cells[1], vec![2, 5, 5, 1, 2]);
        assert_eq!(grid.cells[2], vec![6, 5, 3, 3, 2]);
        assert_eq!(grid.cells[3], vec![3, 3, 5, 4, 9]);
        assert_eq!(grid.cells[4], vec![3, 5, 3, 9, 0]);

        // y,x
        assert_eq!(grid.cells[2][1], 5);

        // get
        assert_eq!(grid.get(0, 0), Some(&3));
        assert_eq!(grid.get(1, 1), Some(&5));
        assert_eq!(grid.get(2, 2), Some(&3));
        assert_eq!(grid.get(9, 9), None);
    }
}
