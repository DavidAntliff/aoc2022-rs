use color_eyre::eyre::Result;
use common::select_and_solve;
use std::collections::HashSet;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day03.1", part1, "inputs/day03.2", part2)?;
    Ok(())
}

// A rucksack is represented as two hashsets of items
struct Rucksack(HashSet<char>, HashSet<char>);

impl TryFrom<&str> for Rucksack {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        match len % 2 {
            0 => {
                let (a, b) = value.split_at(len / 2);
                Ok(Rucksack(a.chars().collect(), b.chars().collect()))
            }
            _ => Err(color_eyre::eyre::eyre!("not even")),
        }
    }
}

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for HashSet<&char> {
    fn priority(&self) -> i32 {
        let mut sum = 0;
        for c in self {
            sum += c.priority();
        }
        sum
    }
}

// TODO: Why can't I get this to work?
// impl<U> Priority for std::collections::hash_set::Intersection<'_, char, U>
// where
//     U: std::hash::BuildHasher,
// {
//     fn priority(&self) -> i32 {
//         let mut sum = 0;
//         for c in self {
//             sum += 1;
//         }
//         sum
//     }
// }

impl Priority for char {
    fn priority(&self) -> i32 {
        if self.is_ascii_lowercase() {
            *self as i32 - 'a' as i32 + 1
        } else if self.is_ascii_uppercase() {
            *self as i32 - 'A' as i32 + 27
        } else {
            0
        }
    }
}

fn part1(input: Vec<String>) -> Result<String> {
    let rucksacks = input
        .iter()
        .map(|s| s.as_str())
        .map(Rucksack::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    // for each item in ruckscks, get intersection and calculate priority
    let mut priority_sum = 0;
    for rucksack in rucksacks {
        //let in_both: HashSet<_> = rucksack.0.intersection(&rucksack.1);
        let in_both: HashSet<_> = rucksack.0.intersection(&rucksack.1).collect();
        println!("{:?}", in_both);

        priority_sum += in_both.priority();
    }

    Ok(priority_sum.to_string())
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
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "157");
    }

    #[rstest]
    fn test_rucksack_tryfrom() {
        let b = Rucksack::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(b.0, "abc".chars().collect());
        assert_eq!(b.1, "DEF".chars().collect());
    }

    #[test]
    fn test_char_priority() {
        assert_eq!('a'.priority(), 1);
        assert_eq!('z'.priority(), 26);
        assert_eq!('A'.priority(), 27);
        assert_eq!('Z'.priority(), 52);
    }

    #[test]
    fn test_intersection_priority() {
        let a: HashSet<_> = "abc".chars().collect();
        let b: HashSet<_> = "bcd".chars().collect();

        let i: HashSet<_> = a.intersection(&b).collect();
        assert_eq!(i.priority(), 'b'.priority() + 'c'.priority());
    }
}
