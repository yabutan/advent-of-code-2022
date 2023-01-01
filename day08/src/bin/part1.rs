use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day08/data/input.txt")?);

    let map = parse_map(r);
    let visible_map = make_visible_map(&map);
    let count = count_visible(&visible_map);

    println!("answer: {}", count);

    Ok(())
}

fn count_visible(visible_map: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    for x in visible_map {
        // print o or x
        for y in x {
            if *y {
                count += 1;
            }
        }
    }
    count
}

fn make_visible_map(data: &[Vec<u8>]) -> Vec<Vec<bool>> {
    let y_len = data.len();
    let x_len = data[0].len();

    let mut visible_map = vec![vec![false; x_len]; y_len];

    for y in 0..y_len {
        // left to right
        let mut max = None;
        for x in 0..x_len {
            let value = data[y][x];
            if max.is_none() || Some(value) > max {
                visible_map[y][x] = true;
                max = Some(value);
            }
        }

        // right to left
        let mut max = None;
        for x in (0..x_len).rev() {
            let value = data[y][x];
            if max.is_none() || Some(value) > max {
                visible_map[y][x] = true;
                max = Some(value);
            }
        }
    }

    for x in 0..x_len {
        // top to bottom
        let mut max = None;
        for y in 0..y_len {
            let value = data[y][x];
            if max.is_none() || Some(value) > max {
                visible_map[y][x] = true;
                max = Some(value);
            }
        }

        // bottom to top
        let mut max = None;
        for y in (0..y_len).rev() {
            let value = data[y][x];
            if max.is_none() || Some(value) > max {
                visible_map[y][x] = true;
                max = Some(value);
            }
        }
    }

    visible_map
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
    use crate::{count_visible, make_visible_map, parse_map};

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
    fn test_make_visible_map() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let map = parse_map(r);

        let visible_map = make_visible_map(&map);

        let mut table = String::new();
        for x in &visible_map {
            // print o or x
            for y in x {
                if *y {
                    table.push_str(" o ");
                } else {
                    table.push_str(" x ");
                }
            }
            table.push('\n');
        }
        assert_eq!(
            table,
            r#"
 o  o  o  o  o 
 o  o  o  x  o 
 o  o  x  o  o 
 o  x  o  x  o 
 o  o  o  o  o 
"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn test_count_visible() {
        let r = include_str!("../../data/sample.txt").as_bytes();
        let map = parse_map(r);

        let visible_map = make_visible_map(&map);
        let count = count_visible(&visible_map);
        assert_eq!(count, 21);
    }
}
