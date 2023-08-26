mod parse;

use crate::parse::load_all_monkeys;
use color_eyre::eyre::Result;
use common::{load_file, select};

fn main() -> Result<()> {
    color_eyre::install()?;
    let name = env!("CARGO_PKG_NAME");
    select(
        format!("inputs/{name}.1").as_str(),
        part1_load,
        format!("inputs/{name}.2").as_str(),
        part2_load,
    )?;
    Ok(())
}

fn part1_load(filename: &str) -> Result<String> {
    part1(load_file(filename)?.as_str())
}

fn part1(input: &str) -> Result<String> {
    let monkeys = load_all_monkeys(input)?;

    Ok(monkeys.len().to_string())
} // if true, if false

fn part2_load(filename: &str) -> Result<String> {
    part2(load_file(filename)?.as_str())
}

fn part2(_input: &str) -> Result<String> {
    Ok("2".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[fixture]
    fn input() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
    }

    #[rstest]
    fn test_part1(input: &str) {
        assert_eq!(part1(input).unwrap(), "1");
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(part2(input).unwrap(), "2");
    }
}
