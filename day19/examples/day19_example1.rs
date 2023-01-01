use rayon::prelude::*;

use day19::Stone::Geode;
use day19::{Blueprint, Processor, State};

fn main() {
    let input = include_str!("../data/sample.txt");
    let blueprints = Blueprint::parse_input(input);

    let score: u32 = blueprints
        .par_iter()
        .map(|blueprint| {
            let mut max = State::new(0);
            let mut process = Processor::new(blueprint);
            process.dfs(State::new(24), &mut max);

            let score = max.get_resource(&Geode) as u32 * blueprint.id;
            println!("id:{} score:{} {:?}", blueprint.id, score, max);
            score
        })
        .sum();

    println!("Score: {}", score);
    assert_eq!(score, 33);
}
