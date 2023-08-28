// Partially based on https://fasterthanli.me/series/advent-of-code-2022/part-12

use color_eyre::eyre::Result;
use common::select_and_solve;
use std::fmt;
use std::fmt::Formatter;

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
    let grid = parse(&input);

    println!("{:?}", grid);

    Ok("1".to_owned())
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in 0..self.0.rows() {
            for c in 0..self.0.cols() {
                write!(f, "{:?}", self.0[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Height(elevation) => (b'a' + elevation) as char,
        };
        write!(f, "{c}")?;
        Ok(())
    }
}

#[derive(Default)]
enum Cell {
    #[default]
    Start,
    End,
    Height(u8),
}

struct Grid(grid::Grid<Cell>);

fn parse(input: &[String]) -> Grid {
    let num_cols = input[0].len();
    let sdata = input.join("");
    let data: Vec<Cell> = sdata
        .chars()
        .map(|c| match c {
            'S' => Cell::Start,
            'E' => Cell::End,
            'a'..='z' => Cell::Height(c as u8 - b'a'),
            _ => panic!("invalid character: {c}"),
        })
        .collect();
    Grid(grid::Grid::from_vec(data, num_cols))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

fn in_bounds(grid: &Grid, coord: GridCoord) -> bool {
    coord.x < grid.0.cols() && coord.y < grid.0.rows()
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
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "1");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "2");
    }
}
