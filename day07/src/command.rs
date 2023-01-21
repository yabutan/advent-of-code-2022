use std::io::BufRead;

use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, newline, space1};
use nom::combinator::{map, opt, recognize, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};

#[derive(Debug, PartialEq)]
pub enum Command {
    Cd(ChangeDir),
    Ls(Vec<LsOutput>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChangeDir {
    // cd /
    Root,

    // cd ..
    MoveOut,

    // cd xxx
    MoveIn(String),
}

#[derive(Debug, PartialEq)]
pub enum LsOutput {
    Dir(String),
    File(String, u64),
}

pub trait CommandParse: BufRead {
    fn parse_commands(&mut self) -> anyhow::Result<Vec<Command>>;
}

impl<R: BufRead> CommandParse for R {
    fn parse_commands(&mut self) -> anyhow::Result<Vec<Command>> {
        let mut buffer = String::new();
        let _ = self.read_to_string(&mut buffer)?;

        return match separated_list1(newline, alt((cd, ls)))(&buffer) {
            Ok((_, commands)) => Ok(commands),
            Err(e) => Err(anyhow!("failed to parse caused by {}", e.to_string())),
        };
    }
}

/// cdコマンド
fn cd(input: &str) -> IResult<&str, Command> {
    use ChangeDir::*;

    let (input, _) = tag("$ cd ")(input)?;
    let (input, cd) = alt((
        value(Root, tag("/")),
        value(MoveOut, tag("..")),
        map(alpha1, |s: &str| MoveIn(s.to_string())),
    ))(input)?;

    Ok((input, Command::Cd(cd)))
}

/// lsコマンド
fn ls<'a>(input: &'a str) -> IResult<&'a str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;

    let dir = |i: &'a str| {
        map(preceded(tag("dir "), alpha1), |s: &str| {
            LsOutput::Dir(s.to_string())
        })(i)
    };

    let file = |i: &'a str| {
        map(
            separated_pair(
                complete::u64,
                space1,
                recognize(tuple((alpha1, opt(tuple((tag("."), alpha1)))))),
            ),
            |(size, filename): (u64, &str)| LsOutput::File(filename.to_string(), size),
        )(i)
    };

    let (input, outputs) = separated_list1(newline, alt((dir, file)))(input)?;

    Ok((input, Command::Ls(outputs)))
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_cd() {
        use ChangeDir::*;
        use Command::Cd;

        assert_eq!(cd("$ cd /"), Ok(("", Cd(Root))));
        assert_eq!(cd("$ cd .."), Ok(("", Cd(MoveOut))));
        assert_eq!(
            cd("$ cd somewhere"),
            Ok(("", Cd(MoveIn("somewhere".to_string()))))
        );
    }

    #[test]
    fn test_parse_ls() {
        let text = indoc! {r#"
            $ ls
            dir a
            14848514 b.txt
            29116 f
            8504156 c.dat
            dir d
        "#};

        let (_, nodes) = ls(text).unwrap();
        println!("{:?}", nodes);

        let Command::Ls(nodes)  = nodes else {
            panic!("failed to parse ls command")
        };

        assert_eq!(nodes.len(), 5);
        assert_eq!(nodes[0], LsOutput::Dir("a".to_string()));
        assert_eq!(nodes[1], LsOutput::File("b.txt".to_string(), 14848514));
        assert_eq!(nodes[2], LsOutput::File("f".to_string(), 29116));
        assert_eq!(nodes[3], LsOutput::File("c.dat".to_string(), 8504156));
        assert_eq!(nodes[4], LsOutput::Dir("d".to_string()));
    }

    #[test]
    fn test_parse_commands() {
        let commands = include_str!("../data/sample.txt")
            .as_bytes()
            .parse_commands()
            .unwrap();
        println!("{:#?}", commands);

        assert_eq!(commands.len(), 10);
        assert_eq!(commands[0], Command::Cd(ChangeDir::Root));
        assert_eq!(
            commands[1],
            Command::Ls(vec![
                LsOutput::Dir("a".to_string()),
                LsOutput::File("b.txt".to_string(), 14848514),
                LsOutput::File("c.dat".to_string(), 8504156),
                LsOutput::Dir("d".to_string()),
            ])
        );
        assert_eq!(commands[2], Command::Cd(ChangeDir::MoveIn("a".to_string())));
        assert_eq!(commands[6], Command::Cd(ChangeDir::MoveOut));
    }
}
