use color_eyre::eyre::{eyre, Result};
use common::select_and_solve;
use derive_more::{Add, Sub};
use eframe::egui;
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
        part1_egui,
        format!("inputs/{name}.2").as_str(),
        part2,
    )?;
    Ok(())
}

fn part1_egui(input: Vec<String>) -> Result<String> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 - Day 9 Part 1",
        options,
        Box::new(|_cc| Box::new(MyApp { input })),
    )
    .expect("run_native failed");

    Ok("".to_string())
}

struct MyApp {
    input: Vec<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Moves:");
            ui.label("➡".repeat(8));
        });
    }
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

    let top_left = Coord(-5, 5);
    let bottom_right = Coord(5, -5);
    println!("start:");
    print_grid(&head, &tail, &visited, top_left, bottom_right);

    for mv in moves {
        head = head.move_by(&mv);
        println!("head moves to: {mv:?}");
        print_grid(&head, &tail, &visited, top_left, bottom_right);

        let tail_moves = catch_up(&head, &tail);
        for mv in tail_moves {
            tail = mv;
            visited.insert(tail);

            println!("tail moves to: {mv:?}");
            print_grid(&head, &tail, &visited, top_left, bottom_right);
        }
    }

    Ok(visited.len().to_string())
}

fn print_grid(
    head: &Coord,
    tail: &Coord,
    visited: &HashSet<Coord>,
    top_left: Coord,
    bottom_right: Coord,
) {
    let (min_x, max_x) = (top_left.0, bottom_right.0);
    let (max_y, min_y) = (top_left.1, bottom_right.1);
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if Coord(x, y) == *head {
                print!("H")
            } else if Coord(x, y) == *tail {
                print!("T");
            } else if visited.contains(&Coord(x, y)) {
                print!("#");
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
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

fn part2(input: Vec<String>) -> Result<String> {
    // H, T(1..9) starts at (0, 0).
    // For each instruction, H moves a number of steps in a single direction.
    // Each T in ascending order then moves to "catch up" with the preceding knot,
    // reducing the distance between it and the knot to at no
    // more than 1 unit orthogonally or diagonally.
    // When catching up, if a diagonal move is necessary, it occurs first.
    // Keep track of all the unique locations visited by T[9].

    let moves = parse_moves(input)?;

    // To simulate as per part 2, we need to split the moves into
    // single-move moves:
    let moves = split_moves(moves);

    let mut visited: HashSet<Coord> = HashSet::new();

    const NUM_KNOTS: usize = 10;
    const HEAD: usize = 0;
    const T9: usize = NUM_KNOTS - 1;

    let mut knots = vec![Coord(0, 0); NUM_KNOTS];

    // T9 starts at same location as T8, ..., H
    visited.insert(knots[T9]);

    let top_left = Coord(-11, 11);
    let bottom_right = Coord(15, -11);
    println!("start:");
    print_grid_v2(&knots, &visited, top_left, bottom_right);

    let last_idx = knots.len() - 1;

    for mv in moves {
        knots[HEAD] = knots[HEAD].move_by(&mv);
        println!("head moves to: {mv:?}");

        for i in 1..knots.len() {
            let prev_knot = knots[i - 1];
            let tail_moves = catch_up(&prev_knot, &knots[i]);
            for mv in tail_moves {
                knots[i] = mv;

                // Add T9 to the record of locations
                if i == last_idx {
                    visited.insert(knots[i]);
                }

                println!("T{i} moves to: {mv:?}");
            }
        }
    }
    print_grid_v2(&knots, &visited, top_left, bottom_right);

    Ok(visited.len().to_string())
}

fn split_moves(moves: Vec<Coord>) -> Vec<Coord> {
    // Split each move into a sequence of single-step moves.
    // E.g. Coord(3, 0) becomes [Coord(1, 0), Coord(1, 0), Coord(1, 0)]
    let mut new_moves: Vec<_> = vec![];
    for mv in moves {
        let c = mv.0 + mv.1; // since orthogonal, either is zero
        for _ in 0..c.abs() {
            new_moves.push(Coord(mv.0.signum(), mv.1.signum()));
        }
    }
    new_moves
}

fn print_grid_v2(knots: &[Coord], visited: &HashSet<Coord>, top_left: Coord, bottom_right: Coord) {
    let (min_x, max_x) = (top_left.0, bottom_right.0);
    let (max_y, min_y) = (top_left.1, bottom_right.1);
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let mut knot_printed = false;
            for (i, knot) in knots.iter().enumerate() {
                if Coord(x, y) == *knot {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{}", i);
                    }
                    knot_printed = true;
                    break;
                }
            }
            if !knot_printed {
                if visited.contains(&Coord(x, y)) {
                    print!("#");
                } else if x == 0 && y == 0 {
                    print!("s");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
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
        assert_eq!(part2(input).unwrap(), "1");
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

    #[fixture]
    fn larger_input() -> Vec<String> {
        "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part2_larger(larger_input: Vec<String>) {
        assert_eq!(part2(larger_input).unwrap(), "36");
    }

    #[test]
    fn test_split_moves() {
        assert_eq!(split_moves(vec![]), vec![]);
        assert_eq!(split_moves(vec![Coord(0, 0)]), vec![]);
        assert_eq!(split_moves(vec![Coord(1, 0)]), vec![Coord(1, 0)]);
        assert_eq!(split_moves(vec![Coord(-3, 0)]), vec![Coord(-1, 0); 3]);
    }
}
