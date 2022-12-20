use day18::simulate::Simulator;
use day18::Vertex3;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let r = BufReader::new(File::open("./day18/data/input.txt").unwrap());
    let cubes = Vertex3::from_reader(r).unwrap();
    let simulator = Simulator::new(&cubes, false);
    let total = simulator.simulate();
    println!("total: {}", total);
}
