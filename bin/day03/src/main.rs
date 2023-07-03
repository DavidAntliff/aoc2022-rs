use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;
use std::collections::HashSet;

fn main() -> Result<()> {
    color_eyre::install()?;

    let _a = Item::try_from('a');

    select_and_solve("inputs/day03.1", part1, "inputs/day03.2", part2)?;
    Ok(())
}

trait Priority {
    fn priority(&self) -> i32;
}

// Abstraction of an Item, hidden in a mod to make the constructor private (i.e. unusable)
// https://fasterthanli.me/series/advent-of-code-2022/part-3
mod item {
    #[repr(transparent)] // not needed?
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Item(char);

    impl TryFrom<char> for Item {
        type Error = color_eyre::eyre::Report;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'a'..='z' | 'A'..='Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!("{} is not a valid item", value)),
            }
        }
    }

    use super::Priority;

    impl Priority for Item {
        fn priority(&self) -> i32 {
            match self {
                Item('a'..='z') => 1 + (self.0 as i32 - 'a' as i32) as i32,
                Item('A'..='Z') => 27 + (self.0 as i32 - 'A' as i32) as i32,
                _ => unreachable!(),
            }
        }
    }
}

use item::Item;

// A rucksack is represented as two hashsets of items
#[derive(Debug, PartialEq)]
struct Rucksack(HashSet<Item>, HashSet<Item>);

impl TryFrom<&str> for Rucksack {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        match len % 2 {
            0 => {
                let (a, b) = value.split_at(len / 2);
                let first_items = a
                    .chars()
                    .map(Item::try_from)
                    .collect::<Result<HashSet<Item>>>()?;
                let second_items = b
                    .chars()
                    .map(Item::try_from)
                    .collect::<Result<HashSet<Item>>>()?;
                Ok(Rucksack(first_items, second_items))
            }
            _ => Err(eyre!("not even")),
        }
    }
}

impl Priority for HashSet<Item> {
    fn priority(&self) -> i32 {
        let mut sum = 0;
        for c in self {
            sum += c.priority();
        }
        sum
    }
}

// TODO: Why can't I get this to work on an Intersection?
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

fn get_rucksacks(input: Vec<String>) -> Result<Vec<Rucksack>> {
    input
        .iter()
        .map(|s| s.as_str())
        .map(Rucksack::try_from)
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input: Vec<String>) -> Result<String> {
    // for each item in rucksacks, get intersection and calculate priority
    // let rucksacks = get_rucksacks(input)?;
    // let mut priority_sum = 0;
    // for rucksack in rucksacks {
    //     let in_both: HashSet<_> = rucksack.0.intersection(&rucksack.1).cloned().collect();
    //     //dbg!("{:?}", &in_both);
    //
    //     priority_sum += in_both.priority();
    // }
    //Ok(priority_sum.to_string())

    // Alternative using combinators:
    Ok(get_rucksacks(input)?
        .iter()
        .map(|r| r.0.intersection(&r.1).cloned().collect::<HashSet<Item>>())
        .map(|r| r.priority())
        .sum::<i32>()
        .to_string())
}

fn get_groups(rucksacks: &Vec<Rucksack>) -> Result<Vec<&[Rucksack]>> {
    // split Vec into groups of three
    if rucksacks.len() % 3 != 0 {
        return Err(eyre!(
            "Vector length is not a multiple of 3 ({})",
            rucksacks.len()
        ));
    }
    Ok(rucksacks.chunks_exact(3).collect())
}

fn part2(input: Vec<String>) -> Result<String> {
    let rucksacks = get_rucksacks(input)?;

    let groups = get_groups(&rucksacks)?;

    let mut sum = 0;
    for group in groups {
        assert_eq!(group.len(), 3);

        let unions: Vec<HashSet<Item>> = group
            .iter()
            .map(|r| r.0.union(&r.1).cloned().collect::<HashSet<Item>>())
            .collect();

        assert_eq!(unions.len(), 3);

        // Verbose intersection of 3:
        //let i: HashSet<char> = unions[0].intersection(&unions[1]).cloned().collect();
        //let i: HashSet<char> = i.intersection(&unions[2]).cloned().collect();

        // Concise intersection of 3:
        //let i = &(&unions[0] & &unions[1]) & &unions[2];

        // Use fold over N items:
        // https://www.reddit.com/r/rust/comments/5v35l6/comment/ddz06ho/
        let mut iter = unions.iter();
        let i = iter
            .next()
            .map(|set| {
                iter.fold(set.clone(), |set1, set2| {
                    set1.intersection(&set2).cloned().collect()
                })
            })
            .ok_or(eyre!("bad"))?;

        sum += i.priority();
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "157");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "70")
    }

    #[rstest]
    fn test_rucksack_tryfrom() {
        let b = Rucksack::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(
            b.0,
            HashSet::from([
                Item::try_from('a').unwrap(),
                Item::try_from('b').unwrap(),
                Item::try_from('c').unwrap()
            ])
        );
        assert_eq!(
            b.1,
            HashSet::from([
                Item::try_from('D').unwrap(),
                Item::try_from('E').unwrap(),
                Item::try_from('F').unwrap()
            ])
        );
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
        let a: HashSet<_> = "abc"
            .chars()
            .map(Item::try_from)
            .collect::<Result<HashSet<Item>>>()
            .unwrap();
        let b: HashSet<_> = "bcd"
            .chars()
            .map(Item::try_from)
            .collect::<Result<HashSet<Item>>>()
            .unwrap();

        let i: HashSet<_> = a.intersection(&b).cloned().collect();
        assert_eq!(i.priority(), 'b'.priority() + 'c'.priority());
    }

    // -- Part 2

    #[rstest]
    fn test_group(input: Vec<String>) {
        let rucksacks = get_rucksacks(input).unwrap();
        let groups = get_groups(&rucksacks).unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0][0], rucksacks[0]);
        assert_eq!(groups[0][1], rucksacks[1]);
        assert_eq!(groups[0][2], rucksacks[2]);
        assert_eq!(groups[1][0], rucksacks[3]);
        assert_eq!(groups[1][1], rucksacks[4]);
        assert_eq!(groups[1][2], rucksacks[5]);
    }
}
