// Partially based on https://fasterthanli.me/series/advent-of-code-2022/part-12

use color_eyre::eyre::Result;
use common::{load_file, select};
use day12::Grid;

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
    let grid = Grid::parse(&input);

    println!("{:?}", grid);

    Ok("1".to_owned())
}

fn part2_load(filename: &str) -> Result<String> {
    part2(load_file(filename)?.as_str())
}

fn part2(_input: &str) -> Result<String> {
    Ok("2".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
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
