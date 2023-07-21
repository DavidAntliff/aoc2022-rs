use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;

use camino::Utf8PathBuf;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{all_consuming, map};
use nom::sequence::{preceded, separated_pair};
use nom::{Finish, IResult};

use id_tree::{InsertBehavior, Node, Tree};

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day07.1", part1, "inputs/day07.2", part2)?;
    Ok(())
}

fn make_tree(input: Vec<String>) -> Result<Tree<FsEntry>> {
    // Parse the input
    let lines = input
        .iter()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    // Initialise a tree
    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            size: 0,
            path: "/".into(),
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    // Build the tree
    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => { // ignore
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => { // ignore
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            size: 0,
                            path: path.clone(),
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => { // ignore
                }
                Entry::File { size, path } => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr));
                }
            },
        }
    }

    // Print the tree
    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    Ok(tree)
}

fn part1(input: Vec<String>) -> Result<String> {
    let tree = make_tree(input)?;

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        // only consider folders:
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();

    Ok(sum.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    let tree = make_tree(input)?;

    let total_space = 70000000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min()
        .ok_or(eyre!("bad"))?;

    Ok(size_to_remove.to_string())
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
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
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

// Use id_tree for tree structure
#[derive(Debug)]
struct FsEntry {
    size: u64,
    path: Utf8PathBuf,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
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
        assert_eq!(part1(input).unwrap(), "95437");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "24933642");
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
            Ok(("", Command::Cd("abc.def".into())))
        );
        assert_eq!(parse_command("$ ls\na\nb"), Ok(("\na\nb", Command::Ls)));
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
        assert_eq!(parse_line("$ ls"), Ok(("", Line::Command(Command::Ls))));
        assert_eq!(
            parse_line("$ cd foo"),
            Ok(("", Line::Command(Command::Cd("foo".into()))))
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
