use std::fs::File;
use std::io::BufReader;

fn main() {
    let r = BufReader::new(File::open("./day19/data/input.txt").unwrap());
    //println!("total: {}", total);
}
