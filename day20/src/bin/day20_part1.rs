use std::io::BufRead;

fn main() {
    let r = include_str!("../../data/sample.txt").as_bytes();

    let list: Vec<i32> = r
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let value: i32 = line.parse().unwrap();
            value
        })
        .collect();

    let len = list.len();
    let move_list = list.clone();

    for m in move_list {}

    println!("{:?}", list);
}
