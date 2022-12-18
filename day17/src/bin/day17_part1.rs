use std::fs::File;
use std::io::BufReader;

fn main() {
    let r = BufReader::new(File::open("./day16/data/input.txt").unwrap());
    //println!("result: {:?}", best_route);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {

    }
}
