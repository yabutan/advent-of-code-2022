use std::collections::HashMap;
use std::path::PathBuf;

use crate::command::{ChangeDir, Command, LsOutput};

#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub size: u64,
}

pub trait IntoDirectories {
    fn into_directories(self) -> Vec<Directory>;
}

impl IntoDirectories for Vec<Command> {
    fn into_directories(self) -> Vec<Directory> {
        // ディレクトリ単体のサイズマップを取得
        let directory_sizes = size_of_dirs(&self);

        directory_sizes
            .keys()
            .map(|p| {
                // ディレクトリ配下のファイルサイズを合算
                let size = directory_sizes
                    .iter()
                    .filter_map(
                        |(dir, size)| {
                            if dir.starts_with(p) {
                                Some(size)
                            } else {
                                None
                            }
                        },
                    )
                    .sum();

                Directory {
                    path: p.clone(),
                    size,
                }
            })
            .collect()
    }
}

/// {dir: size_of_files, ...}
fn size_of_dirs(commands: &[Command]) -> HashMap<PathBuf, u64> {
    let mut pwd = PathBuf::new();
    let mut directory_sizes = HashMap::new();

    for cmd in commands {
        match cmd {
            Command::Cd(cd) => match cd {
                ChangeDir::Root => {
                    pwd = PathBuf::from("/");
                }
                ChangeDir::MoveOut => {
                    let parent = pwd.parent().expect("failed to get parent directory");
                    pwd = parent.to_path_buf();
                }
                ChangeDir::MoveIn(dir) => {
                    pwd.push(dir);
                }
            },
            Command::Ls(files) => {
                let size_of_files = files
                    .iter()
                    .filter_map(|output| match output {
                        LsOutput::File(_, size) => Some(size),
                        _ => None,
                    })
                    .sum();

                directory_sizes.insert(pwd.clone(), size_of_files);
            }
        }
    }

    directory_sizes
}

#[cfg(test)]
mod tests {
    use crate::command::CommandParse;

    use super::*;

    #[test]
    fn test_into_directories() {
        let mut dirs = include_str!("../data/sample.txt")
            .as_bytes()
            .parse_commands()
            .unwrap()
            .into_directories();

        dirs.sort_by_key(|d| d.path.clone());

        assert_eq!(dirs.len(), 4);

        assert_eq!(dirs[0].path, PathBuf::from("/"));
        assert_eq!(dirs[0].size, 48381165);

        assert_eq!(dirs[1].path, PathBuf::from("/a"));
        assert_eq!(dirs[1].size, 94853);

        assert_eq!(dirs[2].path, PathBuf::from("/a/e"));
        assert_eq!(dirs[2].size, 584);

        assert_eq!(dirs[3].path, PathBuf::from("/d"));
        assert_eq!(dirs[3].size, 24933642);
    }
}
