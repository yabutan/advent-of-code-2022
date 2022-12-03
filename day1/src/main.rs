use std::fs;

fn main() {
    println!("Hello, world!");

    let lines: Vec<_> = fs::read_to_string("./day1/data/input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_owned())
        .collect();

    println!("Lines: {:?}", lines);
}
