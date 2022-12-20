use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let r = BufReader::new(File::open("./day18/data/input.txt").unwrap());

    //println!("tall:{}", stage.highest_point + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {}
}
