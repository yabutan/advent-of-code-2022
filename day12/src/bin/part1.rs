use std::fs::read_to_string;


fn main() -> anyhow::Result<()> {
    let text = read_to_string("./day11/data/input.txt")?;

    //println!("answer: {}", monkey_business);

    Ok(())
}
