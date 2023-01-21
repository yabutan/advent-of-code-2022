use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use day07::command::CommandParse;
use day07::directory::{Directory, IntoDirectories};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(File::open("./day07/data/input.txt")?);

    let directory = find_directory_to_delete(r)?;
    println!("dir: {}", directory.path.to_string_lossy());
    println!("answer: {}", directory.size);
    Ok(())
}

fn find_directory_to_delete(mut r: impl BufRead) -> anyhow::Result<Directory> {
    const STORAGE_MAX: u64 = 70000000;
    const STORAGE_NEED_SPACE: u64 = 30000000;

    let directories = r.parse_commands()?.into_directories();

    let root_size = directories
        .iter()
        .find(|d| d.path == Path::new("/"))
        .expect("root directory not found")
        .size;

    let unused_size = STORAGE_MAX - root_size;
    let threshold = STORAGE_NEED_SPACE - unused_size;

    let dir = directories
        .into_iter()
        .filter(|d| d.size >= threshold)
        .min_by_key(|d| d.size)
        .unwrap();

    Ok(dir)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_directory_to_delete() {
        let d = find_directory_to_delete(include_str!("../../data/sample.txt").as_bytes()).unwrap();
        assert_eq!(d.size, 24933642);
    }
}
