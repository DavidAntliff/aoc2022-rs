#![allow(dead_code)]

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
    let line = input.first().ok_or(eyre!("no line"))?;
    let solution = find_sop_marker(line).ok_or(eyre!("not found"))?;
    Ok(solution.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    // expect only one line
    let line = input.first().ok_or(eyre!("no line"))?;
    let solution = find_som_marker(line).ok_or(eyre!("not found"))?;
    Ok(solution.to_string())
}

fn find_sop_marker(s: &str) -> Option<usize> {
    find_marker(4, s)
    //find_marker_alt(4, s)
}

fn find_som_marker(s: &str) -> Option<usize> {
    //find_marker(14, s) // 4.7ms
    //find_marker_alt(14, s) // 4.4ms
    find_marker_alt2(14, s) // 3.2ms
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

// Another implementation from
// https://fasterthanli.me/series/advent-of-code-2022/part-6
struct State {
    data: [u8; 256],
}

impl Default for State {
    fn default() -> Self {
        Self { data: [0; 256] }
    }
}

impl State {
    fn push(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_add(1).unwrap();
    }

    fn pop(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_sub(1).unwrap();
    }

    fn is_unique(&self) -> bool {
        self.data.iter().all(|&x| x <= 1)
    }
}

fn find_marker_alt2(length: usize, input: &str) -> Option<usize> {
    assert!(input.len() > length);

    let mut state = State::default();
    input.bytes().take(length).for_each(|c| state.push(c));
    if state.is_unique() {
        return Some(0);
    }

    for (index, window) in input.as_bytes().windows(length + 1).enumerate() {
        let removed = window[0];
        let added = window[length];

        state.pop(removed);
        state.push(added);

        if state.is_unique() {
            return Some(index + 1 + length);
        }
    }

    None
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
