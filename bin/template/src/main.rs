use color_eyre::eyre::Result;
use common::select_and_solve;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/dayXX.1", part1, "inputs/dayXX.2", part2)?;
    Ok(())
}

fn part1(_input: Vec<String>) -> Result<String> {
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
}
