use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day06.1", part1, "inputs/day06.2", part2)?;
    Ok(())
}

fn part1(input: Vec<String>) -> Result<String> {
    // expect only one line
    let line = input.iter().next().ok_or(eyre!("no line"))?;
    let solution = find_sop_marker(line)?;
    Ok(solution.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    // expect only one line
    let line = input.iter().next().ok_or(eyre!("no line"))?;
    let solution = find_som_marker(line)?;
    Ok(solution.to_string())
}

fn find_sop_marker(s: &str) -> Result<usize> {
    find_marker(4, s)
}

fn find_som_marker(s: &str) -> Result<usize> {
    find_marker(14, s)
}

fn find_marker(length: usize, s: &str) -> Result<usize> {
    // Find the Start-of-Packet marker, consisting of the first set of four
    // consecutive characters that don't repeat. Return the number of characters
    // received at the point this is determined.

    let mut window = VecDeque::new();
    window.reserve_exact(length);

    let mut count = 0;

    // charge the set with the first 3 characters
    for (i, ch) in s.chars().enumerate() {
        if window.len() >= length {
            window.pop_front();
        }
        window.push_back(ch);
        eprintln!("{}: {:?}", ch, window);

        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == length {
            count = i + 1;
            break;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            find_sop_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
            7
        );
        assert_eq!(find_sop_marker("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(find_sop_marker("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(
            find_sop_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            10
        );
        assert_eq!(
            find_sop_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            11
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            find_som_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
            19
        );
        assert_eq!(find_som_marker("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(find_som_marker("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(
            find_som_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            29
        );
        assert_eq!(
            find_som_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            26
        );
    }
}
