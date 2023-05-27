fn main() -> color_eyre::Result<()> {
    common::select_and_solve("inputs/day03.1", part1, "inputs/day03.2", part2)
}

fn part1(_input: Vec<String>) -> String {
    "1".to_owned()
}

fn part2(_input: Vec<String>) -> String {
    "2".to_owned()
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
