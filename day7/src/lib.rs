use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline, space1};
use nom::combinator::{map, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::PathBuf;

pub struct Directory {
    pub path: PathBuf,
    pub size: u64,
}

pub fn calc_size_of_directories(r: impl BufRead) -> anyhow::Result<Vec<Directory>> {
    let commands = read_commands(r)?;
    let mut path = PathBuf::new();

    // path => size of files
    let mut size_map = HashMap::new();

    for cmd in commands {
        match cmd {
            Command::Cd(cd) => match cd {
                ChangeDir::Root => {
                    path = PathBuf::from("/");
                }
                ChangeDir::MoveOut => {
                    let parent = path.parent().unwrap();
                    path = parent.to_path_buf();
                }
                ChangeDir::MoveIn(dir) => {
                    path.push(dir);
                }
            },
            Command::Ls(files) => {
                let mut sum_file_size = 0;
                for n in &files {
                    let Node::File(_, size) = n else {
                        continue;
                    };
                    sum_file_size += size;
                }
                size_map.insert(path.clone(), sum_file_size);
                println!("{:?} files:{:?} sum:{}", path, files, sum_file_size);
            }
        }
    }

    println!("size_map: {:#?}", size_map);

    let mut list = Vec::new();

    for target_dir in size_map.keys() {
        let mut sum_size = 0;
        for (dir, size) in &size_map {
            if dir.starts_with(target_dir) {
                sum_size += size;
            }
        }

        list.push(Directory {
            path: target_dir.clone(),
            size: sum_size,
        });
    }

    Ok(list)
}

#[derive(Debug, PartialEq)]
enum ChangeDir {
    Root,
    MoveOut,
    MoveIn(String),
}

#[derive(Debug, PartialEq)]
enum Node {
    Dir(String),
    File(String, u64),
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd(ChangeDir),
    Ls(Vec<Node>),
}

fn read_commands(r: impl BufRead) -> anyhow::Result<Vec<Command>> {
    let mut list = Vec::new();

    let mut buffer = String::new();
    for line in r.lines() {
        let line = line.unwrap();

        if !buffer.is_empty() && line.starts_with('$') {
            let (_, cmd) = parse_command(buffer.trim()).expect("failed to parse command");
            list.push(cmd);

            buffer.clear();
        }

        buffer.push_str(&line);
        buffer.push('\n');
    }

    if !buffer.is_empty() {
        let (_, cmd) = parse_command(buffer.trim()).expect("failed to parse command");
        list.push(cmd);
    }

    Ok(list)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((map(parse_cd, Command::Cd), map(parse_ls, Command::Ls)))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    separated_list1(newline, parse_node)(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let dir_parser = map(preceded(tag("dir "), alphanumeric1), |s: &str| {
        Node::Dir(s.to_string())
    });
    let file_parser = map(
        tuple((
            alphanumeric1,
            space1,
            recognize(tuple((
                alphanumeric1,
                opt(tuple((tag("."), alphanumeric1))),
            ))),
        )),
        |(size, _, name): (&str, &str, &str)| {
            Node::File(name.to_string(), size.parse::<u64>().unwrap())
        },
    );

    alt((dir_parser, file_parser))(input)
}

fn parse_cd(input: &str) -> IResult<&str, ChangeDir> {
    let (input, value) = preceded(tag("$ cd "), alt((tag("/"), tag(".."), alphanumeric1)))(input)?;

    let ret = match value {
        "/" => ChangeDir::Root,
        ".." => ChangeDir::MoveOut,
        _ => ChangeDir::MoveIn(value.to_string()),
    };

    Ok((input, ret))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cd() {
        assert_eq!(parse_cd("$ cd /"), Ok(("", ChangeDir::Root)));
        assert_eq!(parse_cd("$ cd .."), Ok(("", ChangeDir::MoveOut)));
        assert_eq!(
            parse_cd("$ cd somewhere"),
            Ok(("", ChangeDir::MoveIn("somewhere".to_string())))
        );
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node("dir a"), Ok(("", Node::Dir("a".to_string()))));
        assert_eq!(
            parse_node("29116 f"),
            Ok(("", Node::File("f".to_string(), 29116)))
        );
        assert_eq!(
            parse_node("14848514 b.txt"),
            Ok(("", Node::File("b.txt".to_string(), 14848514)))
        );
    }

    #[test]
    fn test_parse_ls() {
        let text = r#"$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d"#;

        let (_, nodes) = parse_ls(text).unwrap();
        println!("{:?}", nodes);

        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0], Node::Dir("a".to_string()));
        assert_eq!(nodes[1], Node::File("b.txt".to_string(), 14848514));
        assert_eq!(nodes[2], Node::File("c.dat".to_string(), 8504156));
        assert_eq!(nodes[3], Node::Dir("d".to_string()));
    }

    #[test]
    fn test_read_buffers() {
        let commands = read_commands(include_str!("../data/sample.txt").as_bytes()).unwrap();
        println!("{:#?}", commands);

        assert_eq!(commands.len(), 10);
        assert_eq!(commands[0], Command::Cd(ChangeDir::Root));
        assert_eq!(
            commands[1],
            Command::Ls(vec![
                Node::Dir("a".to_string()),
                Node::File("b.txt".to_string(), 14848514),
                Node::File("c.dat".to_string(), 8504156),
                Node::Dir("d".to_string()),
            ])
        );
        assert_eq!(commands[2], Command::Cd(ChangeDir::MoveIn("a".to_string())));
        assert_eq!(commands[6], Command::Cd(ChangeDir::MoveOut));
    }
}
