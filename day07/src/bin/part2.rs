use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use day07::{calc_size_of_directories, Directory};

const TOTAL_STORAGE: u64 = 70000000;
const NEED_STORAGE: u64 = 30000000;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day07/data/input.txt")?);

    let directory = find_directory_to_delete(r)?;
    println!("answer: {}", directory.size);

    Ok(())
}

fn find_directory_to_delete(r: impl BufRead) -> anyhow::Result<Directory> {
    let directories = calc_size_of_directories(r)?;

    let root_size = directories
        .iter()
        .find(|d| d.path == Path::new("/"))
        .expect("root directory not found")
        .size;

    let unused_size = TOTAL_STORAGE - root_size;
    let threshold = NEED_STORAGE - unused_size;
    println!("unused_size: {}", unused_size);
    println!("threshold: {}", threshold);

    Ok(directories
        .into_iter()
        .filter(|d| d.size >= threshold)
        .min_by_key(|d| d.size)
        .unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pick_should_delete() {
        let d = find_directory_to_delete(include_str!("../../data/sample.txt").as_bytes()).unwrap();
        assert_eq!(d.size, 24933642);
    }
}
