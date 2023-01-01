use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day08/data/input.txt")?);

    let map = parse_map(r);
    let score = find_highest(&map);
    println!("answer: {}", score);

    Ok(())
}

fn find_highest(data: &[Vec<u8>]) -> u32 {
    let y_len = data.len();
    let x_len = data[0].len();

    let mut max = 0;

    // view right
    for x in 0..x_len {
        for y in 0..y_len {
            let counts = count_view_tree(x, y, data);
            let score = calc_scenic_score(counts);

            if score > max {
                max = score;
            }
        }
    }

    max
}

fn calc_scenic_score(count: (u32, u32, u32, u32)) -> u32 {
    count.0 * count.1 * count.2 * count.3
}

/// (left, top, right, bottom)
fn count_view_tree(pos_x: usize, pos_y: usize, data: &[Vec<u8>]) -> (u32, u32, u32, u32) {
    let y_len = data.len();
    let x_len = data[0].len();

    let height = data[pos_y][pos_x];

    // view right
    let mut count_right = 0;
    for x in (pos_x + 1)..x_len {
        let value = data[pos_y][x];
        count_right += 1;
        if value >= height {
            break;
        }
    }

    // view left
    let mut count_left = 0;
    if pos_x > 0 {
        for x in (0..=(pos_x - 1)).rev() {
            let value = data[pos_y][x];
            count_left += 1;
            if value >= height {
                break;
            }
        }
    }

    // view bottom
    let mut count_bottom = 0;
    for y in (pos_y + 1)..y_len {
        let value = data[y][pos_x];
        count_bottom += 1;
        if value >= height {
            break;
        }
    }

    // view top
    let mut count_top = 0;
    if pos_y > 0 {
        for y in (0..=(pos_y - 1)).rev() {
            let value = data[y][pos_x];
            count_top += 1;
            if value >= height {
                break;
            }
        }
    }

    (count_left, count_top, count_right, count_bottom)
}

fn parse_map(r: impl BufRead) -> Vec<Vec<u8>> {
    let mut list = Vec::new();
    for line in r.lines() {
        let line = line.unwrap();

        let mut array = Vec::new();
        for c in line.chars() {
            array.push(c.to_digit(10).unwrap() as u8);
        }
        list.push(array);
    }

    list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_map() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let map = parse_map(r);

        for x in &map {
            println!("{:?}", x);
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(map[1], vec![2, 5, 5, 1, 2]);
        assert_eq!(map[2], vec![6, 5, 3, 3, 2]);
        assert_eq!(map[3], vec![3, 3, 5, 4, 9]);
        assert_eq!(map[4], vec![3, 5, 3, 9, 0]);

        // y,x
        assert_eq!(map[2][1], 5);
    }

    #[test]
    fn test_count_view_tree() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let map = parse_map(r);

        let ret = count_view_tree(2, 1, &map);
        assert_eq!(ret, (1, 1, 2, 2));

        let ret = count_view_tree(2, 3, &map);
        assert_eq!(ret, (2, 2, 2, 1));
    }

    #[test]
    fn test_calc_scenic_score() {
        assert_eq!(calc_scenic_score((1, 1, 2, 2)), 4);
        assert_eq!(calc_scenic_score((2, 2, 2, 1)), 8);
    }
}
