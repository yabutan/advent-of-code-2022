use std::fs::File;
use std::io::BufReader;


fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day13/data/input.txt")?);
    //println!("answer: {}", ret);
    Ok(())
}
