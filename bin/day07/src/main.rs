use color_eyre::eyre::Result;
use common::select_and_solve;

use camino::Utf8PathBuf;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{all_consuming, map};
use nom::sequence::{preceded, separated_pair};
use nom::{Finish, IResult};

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day07.1", part1, "inputs/day07.2", part2)?;
    Ok(())
}

fn part1(_input: Vec<String>) -> Result<String> {
    Ok("1".to_owned())
}

fn part2(_input: Vec<String>) -> Result<String> {
    Ok("2".to_owned())
}

// https://fasterthanli.me/series/advent-of-code-2022/part-7#part-1
fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

// Parse commands:
#[derive(Debug, PartialEq)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug, PartialEq)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug, PartialEq)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(i)
}

// Parse entries
#[derive(Debug, PartialEq)]
enum Entry {
    Dir(Utf8PathBuf),
    File { size: u64, path: Utf8PathBuf },
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    alt((
        map(
            separated_pair(nom::character::complete::u64, tag(" "), parse_path),
            |(size, path)| Entry::File { size, path },
        ),
        map(preceded(tag("dir "), parse_path), Entry::Dir),
    ))(i)
}

// Parse lines
#[derive(Debug, PartialEq)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "1");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "2");
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(
            parse_path("abcde fghij"),
            Ok((" fghij", Utf8PathBuf::from("abcde")))
        );
    }

    #[test]
    fn test_parse_ls() {
        assert_eq!(parse_ls("ls abcde"), Ok((" abcde", Ls)));
    }

    #[test]
    fn test_parse_cd() {
        assert_eq!(
            parse_cd("cd abc.def"),
            Ok(("", Cd(Utf8PathBuf::from("abc.def"))))
        );
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("$ cd abc.def"),
            Ok(("", Command::Cd(Cd(Utf8PathBuf::from("abc.def")))))
        );
        assert_eq!(parse_command("$ ls\na\nb"), Ok(("\na\nb", Command::Ls(Ls))));
    }

    #[test]
    fn test_parse_entry() {
        assert_eq!(parse_entry("dir abc"), Ok(("", Entry::Dir("abc".into()))));
        assert_eq!(
            parse_entry("12345 def"),
            Ok((
                "",
                Entry::File {
                    size: 12345,
                    path: "def".into()
                }
            ))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("$ ls"), Ok(("", Line::Command(Command::Ls(Ls)))));
        assert_eq!(
            parse_line("$ cd foo"),
            Ok(("", Line::Command(Command::Cd(Cd("foo".into())))))
        );
        assert_eq!(
            parse_line("dir abcdef"),
            Ok(("", Line::Entry(Entry::Dir("abcdef".into()))))
        );
        assert_eq!(
            parse_line("98765 bar"),
            Ok((
                "",
                Line::Entry(Entry::File {
                    size: 98765,
                    path: "bar".into()
                })
            ))
        );
    }

    #[rstest]
    fn test_parse_sample_input(input: Vec<String>) {
        let lines = input
            .iter()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
        for line in lines {
            println!("{line:?}");
        }
    }
}
