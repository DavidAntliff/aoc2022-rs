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
    let solution = find_sop_marker(line).ok_or(eyre!("not found"))?;
    Ok(solution.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    // expect only one line
    let line = input.iter().next().ok_or(eyre!("no line"))?;
    let solution = find_som_marker(line).ok_or(eyre!("not found"))?;
    Ok(solution.to_string())
}

fn find_sop_marker(s: &str) -> Option<usize> {
    find_marker_alt(4, s)
}

fn find_som_marker(s: &str) -> Option<usize> {
    find_marker_alt(14, s)
}

fn find_marker(length: usize, s: &str) -> Option<usize> {
    // Find the Start-of-Packet marker, consisting of the first set of four
    // consecutive characters that don't repeat. Return the number of characters
    // received at the point this is determined.

    let mut window = VecDeque::new();
    window.reserve_exact(length);

    let mut count = None;

    // charge the set with the first 3 characters
    for (i, ch) in s.chars().enumerate() {
        if window.len() >= length {
            window.pop_front();
        }
        window.push_back(ch);
        //eprintln!("{}: {:?}", ch, window);

        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == length {
            count = Some(i + 1);
            break;
        }
    }

    count
}

// https://fasterthanli.me/series/advent-of-code-2022/part-6
fn find_marker_alt(length: usize, s: &str) -> Option<usize> {
    s.as_bytes()
        .windows(length)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == length)
        .map(|pos| pos + length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_sop_marker(index: usize, input: &str) {
        assert_eq!(Some(index), find_sop_marker(input));
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_som_marker(index: usize, input: &str) {
        assert_eq!(Some(index), find_som_marker(input));
    }
}
