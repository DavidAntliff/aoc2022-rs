fn main() -> color_eyre::Result<()> {
    common::select_and_solve("inputs/day01.1", part1,
                             "inputs/day01.2", part2)
}

fn part1(input: Vec<String>) -> String {
    let mut sum = 0;
    let mut max = 0;
    for line in input {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    max.to_string()
}

fn part2(input: Vec<String>) -> String {
    let mut sum = 0;
    let mut values = vec![];
    for line in input {
        if line.is_empty() {
            values.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    values.sort();
    values.reverse();
    (values[0] + values[1] + values[2]).to_string()
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
        assert_eq!(part1(input), "24000");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input), "45000");
    }
}
