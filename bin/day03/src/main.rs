use color_eyre::eyre::Result;
use common::select_and_solve;
use std::collections::HashSet;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day03.1", part1, "inputs/day03.2", part2)?;
    Ok(())
}

// A backpack is represented as two hashsets of items
struct Backpack(HashSet<char>, HashSet<char>);

impl TryFrom<&str> for Backpack {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        match len % 2 {
            0 => {
                let (a, b) = value.split_at(len / 2);
                Ok(Backpack(a.chars().collect(), b.chars().collect()))
            }
            _ => Err(color_eyre::eyre::eyre!("not even")),
        }
    }
}

// TODO: why can't we implement From<&str> and have it work below?
fn part1(input: Vec<String>) -> Result<String> {
    let backpacks = input
        .iter()
        .map(|s| s.as_str())
        .map(Backpack::try_from)
        .collect::<Result<Vec<_>, _>>();

    Ok("1".to_owned())
}

fn part2(_input: Vec<String>) -> Result<String> {
    Ok("2".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "
"
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {}

    #[rstest]
    fn test_backpack_tryfrom() {
        let b = Backpack::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(b.0, "abc");
        assert_eq!(b.1, "DEF");
    }

    #[rstest]
    fn test_backpack2_tryfrom() {
        let b = Backpack2::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(b.0, "abc".chars().collect());
        assert_eq!(b.1, "DEF".chars().collect());
    }
}
