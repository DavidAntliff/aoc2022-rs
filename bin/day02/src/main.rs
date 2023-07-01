use color_eyre::Result;
use std::cmp::Ordering;

fn main() -> Result<()> {
    color_eyre::install()?;
    common::select_and_solve("inputs/day02.1", part1, "inputs/day02.2", part2)?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

trait Score {
    fn score(&self) -> i32;
}

impl Score for Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,     // A X
            Move::Paper => 2,    // B Y
            Move::Scissors => 3, // C Z
        }
    }
}

impl Score for Ordering {
    fn score(&self) -> i32 {
        match self {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Rock) => Some(Ordering::Equal),
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Paper) => Some(Ordering::Equal),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            (Move::Scissors, Move::Scissors) => Some(Ordering::Equal),
        }
    }
}

fn parse_moves(input: Vec<String>) -> Vec<(Move, Move)> {
    let moves = input
        .iter()
        .map(|s| {
            let v = s
                .split(' ')
                .map(|t| match t {
                    "A" | "X" => Move::Rock,
                    "B" | "Y" => Move::Paper,
                    "C" | "Z" => Move::Scissors,
                    _ => panic!("invalid"),
                })
                .collect::<Vec<Move>>();
            let mut i = v.into_iter();
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect::<Vec<(Move, Move)>>();

    moves
}

fn part1(input: Vec<String>) -> Result<String> {
    let mut score = 0;
    let rounds: Vec<(Move, Move)> = parse_moves(input);

    for (move1, move2) in rounds {
        if let Some(c) = move2.partial_cmp(&move1) {
            let outcome_score = c.score();
            let shape_score = move2.score();
            println!(
                "{score}: {:?} {:?} -> {} {}",
                move1, move2, outcome_score, shape_score
            );
            score += outcome_score + shape_score;
        }
    }

    Ok(score.to_string())
}

fn lose(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn win(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn parse_line2(line: &str) -> (Move, Move) {
    let v: Vec<&str> = line.split(' ').collect();
    let (theirs, ours) = (v[0], v[1]);

    let move1 = match theirs {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("invalid"),
    };

    let move2 = match ours {
        "X" => lose(&move1),
        "Y" => move1,
        "Z" => win(&move1),
        _ => panic!("invalid"),
    };

    (move1, move2)
}

fn parse_moves2(input: Vec<String>) -> Vec<(Move, Move)> {
    let moves = input
        .iter()
        .map(|s| parse_line2(s))
        .collect::<Vec<(Move, Move)>>();

    moves
}

fn part2(input: Vec<String>) -> Result<String> {
    let mut score = 0;
    let rounds: Vec<(Move, Move)> = parse_moves2(input);

    for (move1, move2) in rounds {
        if let Some(c) = move2.partial_cmp(&move1) {
            let outcome_score = c.score();
            let shape_score = move2.score();
            println!(
                "{score}: {:?} {:?} -> {} {}",
                move1, move2, outcome_score, shape_score
            );
            score += outcome_score + shape_score;
        }
    }

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Move::{Paper, Rock, Scissors};
    use common::vec_of_strings;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        vec_of_strings!["A Y", "B X", "C Z"]
        //         "A Y
        // B X
        // C Z
        // "
        //         .split('\n')
        //         //.map(|s| s.to_string())
        //         .map(String::from)
        //         .collect()
    }

    #[rstest]
    fn test_parse_moves(input: Vec<String>) {
        let result = parse_moves(input);
        assert_eq!(
            result,
            vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        );
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "15");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "12");
    }
}
