use std::fs::File;
use std::io::BufReader;

use day7::find_directory;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day7/data/input.txt")?);

    let directory = find_directory(r)?;
    println!("answer: {}", directory.size);

    Ok(())
}
