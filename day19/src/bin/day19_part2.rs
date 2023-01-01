use rayon::prelude::*;

use day19::{Blueprint, Processor, State, Stone};
use Stone::Geode;

fn main() {
    let input = include_str!("../../data/input.txt");
    let blueprints = Blueprint::parse_input(input);

    let score: i32 = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut max = State::new(0);
            let mut process = Processor::new(blueprint);
            process.dfs(State::new(32), &mut max);

            let best = max.get_resource(&Geode);
            println!("id:{} {:?}", blueprint.id, max);

            best
        })
        .product();

    println!("Score: {}", score);
}
