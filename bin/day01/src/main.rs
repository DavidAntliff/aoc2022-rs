use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day01.1", part1_alt, "inputs/day01.2", part2_alt)?;
    Ok(())
}

fn part1(input: Vec<String>) -> Result<String> {
    let mut sum = 0;
    let mut max = 0;
    for line in input {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }

    Ok(max.to_string())
}

// https://fasterthanli.me/series/advent-of-code-2022/part-1
fn part1_alt(input: Vec<String>) -> Result<String> {
    let lines = input
        .iter()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let elven_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .ok_or(eyre!("bad"))?;

    Ok(elven_lead.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    let mut sum = 0;
    let mut values = vec![];
    for line in input {
        if line.is_empty() {
            values.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }

    values.sort();
    values.reverse();
    Ok((values[0] + values[1] + values[2]).to_string())
}

// https://fasterthanli.me/series/advent-of-code-2022/part-1
fn part2_alt(input: Vec<String>) -> Result<String> {
    use itertools::Itertools;
    use std::cmp::Reverse;

    let answer = input
        .iter()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "24000");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "45000");
    }
}
