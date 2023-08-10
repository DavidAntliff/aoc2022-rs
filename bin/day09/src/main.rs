use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;
use derive_more::{Add, Sub};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;

fn main() -> Result<()> {
    color_eyre::install()?;
    let name = env!("CARGO_PKG_NAME");
    select_and_solve(
        format!("inputs/{name}.1").as_str(),
        part1,
        format!("inputs/{name}.2").as_str(),
        part2,
    )?;
    Ok(())
}

fn part1(input: Vec<String>) -> Result<String> {
    // H, T starts at (0, 0).
    // For each instruction, H moves a number of steps in a single direction.
    // T then moves to "catch up", reducing the distance between T and H to at no
    // more than 1 unit orthogonally or diagonally.
    // When catching up, if a diagonal move is necessary, it occurs first.
    // Keep track of all the unique locations visited by T.

    let moves = parse_moves(input)?;

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut head = Coord(0, 0);
    let mut tail = head;

    // T starts at same location as H
    visited.insert(tail);

    for mv in moves {
        head = head.move_by(&mv);
        let tail_moves = catch_up(&head, &tail);
        for mv in tail_moves {
            tail = mv;
            visited.insert(tail);
        }
    }

    Ok(visited.iter().count().to_string())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Add, Sub)]
struct Coord(isize, isize);

impl Coord {
    fn move_by(self, by: &Coord) -> Coord {
        self + *by
    }

    fn chebyshev_distance(self, to: &Coord) -> usize {
        // Chebyshev distance: https://chris3606.github.io/GoRogue/articles/grid_components/measuring-distance.html#chebyshev-distance
        let d = *to - self;
        d.0.abs().max(d.1.abs()) as usize
    }
}

fn catch_up(head: &Coord, tail: &Coord) -> Vec<Coord> {
    let mut tail = *tail;
    let mut tail_moves = vec![];
    loop {
        let distance = tail.chebyshev_distance(head);
        if distance <= 1 {
            return tail_moves;
        }

        // prioritise diagonal movement
        let mv = match *head - tail {
            Coord(0, y) if y > 0 => Coord(0, 1),
            Coord(0, y) if y < 0 => Coord(0, -1),
            Coord(x, 0) if x > 0 => Coord(1, 0),
            Coord(x, 0) if x < 0 => Coord(-1, 0),
            Coord(x, y) => Coord(x.signum(), y.signum()),
            //_ => unreachable!(),
        };

        tail = tail + mv;
        tail_moves.push(tail);
    }
}

fn parse_move(input: &str) -> IResult<&str, Coord> {
    let (input, (_, direction, _, distance)) = tuple((
        multispace0,
        nom::character::complete::alpha1,
        multispace0,
        map_res(digit1, |s: &str| s.parse::<isize>()),
    ))(input)?;

    match direction {
        "U" => Ok((input, Coord(0, distance))),
        "D" => Ok((input, Coord(0, -distance))),
        "L" => Ok((input, Coord(-distance, 0))),
        "R" => Ok((input, Coord(distance, 0))),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "invalid direction",
            nom::error::ErrorKind::Alpha,
        ))),
    }
}

fn parse_moves(input: Vec<String>) -> Result<Vec<Coord>> {
    let parsed_moves: Result<Vec<(&str, Coord)>, nom::Err<nom::error::Error<&str>>> =
        input.iter().map(|s| parse_move(s)).collect();

    let moves: Result<Vec<Coord>, nom::Err<nom::error::Error<&str>>> =
        parsed_moves.map(|parsed_move| parsed_move.into_iter().map(|(_, m)| m).collect());

    moves.map_err(|e| eyre!("parse failed: {}", e))
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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "13");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "2");
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(parse_move("U 1"), Ok(("", Coord(0, 1))));
        assert_eq!(parse_move("D 2"), Ok(("", Coord(0, -2))));
        assert_eq!(parse_move("L 13"), Ok(("", Coord(-13, 0))));
        assert_eq!(parse_move("R 0"), Ok(("", Coord(0, 0))));
    }

    #[rstest]
    fn test_parse_moves(input: Vec<String>) {
        assert_eq!(
            parse_moves(input).unwrap(),
            vec![
                Coord(4, 0),
                Coord(0, 4),
                Coord(-3, 0),
                Coord(0, -1),
                Coord(4, 0),
                Coord(0, -1),
                Coord(-5, 0),
                Coord(2, 0),
            ]
        );
    }

    #[test]
    fn test_catch_up_right() {
        let head = Coord(2, 0);
        let tail = Coord(0, 0);
        assert_eq!(catch_up(&head, &tail), vec![Coord(1, 0)]);
    }

    #[test]
    fn test_catch_up_left() {
        let head = Coord(-3, 0);
        let tail = Coord(0, 0);
        assert_eq!(catch_up(&head, &tail), vec![Coord(-1, 0), Coord(-2, 0)]);
    }

    #[test]
    fn test_catch_up_up() {
        let head = Coord(3, 4);
        let tail = Coord(3, 1);
        assert_eq!(catch_up(&head, &tail), vec![Coord(3, 2), Coord(3, 3)]);
    }

    #[test]
    fn test_catch_up_diagonal() {
        let head = Coord(3, 3);
        let tail = Coord(0, 0);
        assert_eq!(catch_up(&head, &tail), vec![Coord(1, 1), Coord(2, 2)]);
    }

    #[test]
    fn test_catch_up_diagonal_then_up() {
        let head = Coord(1, 3);
        let tail = Coord(0, 0);
        assert_eq!(catch_up(&head, &tail), vec![Coord(1, 1), Coord(1, 2)]);
    }

    // #[test]
    // fn test_parse_steps() {
    //     assert_eq!(parse_steps("1"), Ok(("", 1)));
    //     assert_eq!(parse_steps("12"), Ok(("", 12)));
    // }

    // #[test]
    // fn test_parse_move() {
    //     assert_eq!(parse_move("U 1"), Move::Up(1));
    // }
}
