use day08::{Grid, GridData};
use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day08/data/input.txt")?);
    let grid = GridData::parse(r);
    let visible_map = VisibleMap::create(&grid);
    let count = visible_map.count_visible();

    println!("answer: {count}");
    Ok(())
}

struct VisibleMap {
    data: Vec<Vec<bool>>,
}

impl VisibleMap {
    fn create(grid: &GridData) -> Self {
        let y_len = grid.height();
        let x_len = grid.width();

        let mut data = vec![vec![false; x_len]; y_len];

        for y in 0..y_len {
            // left to right
            let mut max = None;
            for x in 0..x_len {
                let value = grid.get(x, y).unwrap();
                if max.is_none() || Some(value) > max {
                    data[y][x] = true;
                    max = Some(value);
                }
            }

            // right to left
            let mut max = None;
            for x in (0..x_len).rev() {
                let value = grid.get(x, y).unwrap();
                if max.is_none() || Some(value) > max {
                    data[y][x] = true;
                    max = Some(value);
                }
            }
        }

        for x in 0..x_len {
            // top to bottom
            let mut max = None;
            for y in 0..y_len {
                let value = grid.get(x, y).unwrap();
                if max.is_none() || Some(value) > max {
                    data[y][x] = true;
                    max = Some(value);
                }
            }

            // bottom to top
            let mut max = None;
            for y in (0..y_len).rev() {
                let value = grid.get(x, y).unwrap();
                if max.is_none() || Some(value) > max {
                    data[y][x] = true;
                    max = Some(value);
                }
            }
        }

        Self { data }
    }

    fn count_visible(&self) -> usize {
        self.data.iter().flatten().filter(|&&x| x).count()
    }
}

impl ToString for VisibleMap {
    fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|x| {
                x.iter()
                    .map(|b| if *b { 'o' } else { 'x' })
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use crate::VisibleMap;
    use day08::GridData;
    use indoc::indoc;

    #[test]
    fn test_visible_map() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let grid = GridData::parse(r);
        let visible_map = VisibleMap::create(&grid);

        assert_eq!(
            visible_map.to_string(),
            indoc! {r#"
            ooooo
            oooxo
            ooxoo
            oxoxo
            ooooo
            "#}
            .trim_end()
        );

        let count = visible_map.count_visible();
        assert_eq!(count, 21);
    }
}
