use color_eyre::eyre::Result;
use common::select_and_solve;
use grid::Grid;

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

fn make_grid(input: Vec<String>) -> Grid<u32> {
    let num_cols = input[0].len();
    let sdata = input.join("");
    let data: Vec<u32> = sdata.chars().map(|c| c as u32 - '0' as u32).collect();
    Grid::from_vec(data, num_cols)
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn mark_visible(heights: &Grid<u32>, visible: &mut Grid<bool>, from_the: Direction) {
    let (rows, cols) = heights.size();

    fn update(r: usize, c: usize, max: &mut i32, heights: &Grid<u32>, visible: &mut Grid<bool>) {
        if heights[r][c] as i32 > *max {
            visible[r][c] = true;
            *max = heights[r][c] as i32;
        }
    }

    match from_the {
        Direction::North => {
            for c in 0..cols {
                let mut max = -1;
                for r in 0..rows {
                    update(r, c, &mut max, heights, visible);
                }
            }
        }
        Direction::East => {
            for r in 0..rows {
                let mut max = -1;
                for c in (0..cols).rev() {
                    update(r, c, &mut max, heights, visible);
                }
            }
        }
        Direction::South => {
            for c in 0..cols {
                let mut max = -1;
                for r in (0..rows).rev() {
                    update(r, c, &mut max, heights, visible);
                }
            }
        }
        Direction::West => {
            for r in 0..rows {
                let mut max = -1;
                for c in 0..cols {
                    update(r, c, &mut max, heights, visible);
                }
            }
        }
    }
}

fn how_many_visible(heights: Grid<u32>) -> usize {
    let (rows, cols) = heights.size();

    let mut visible = Grid::init(rows, cols, false);

    mark_visible(&heights, &mut visible, Direction::North);
    mark_visible(&heights, &mut visible, Direction::East);
    mark_visible(&heights, &mut visible, Direction::South);
    mark_visible(&heights, &mut visible, Direction::West);

    // count the number of visible trees
    visible.iter().filter(|&&x| x).count()
}

fn part1(input: Vec<String>) -> Result<String> {
    let grid = make_grid(input);
    let num_visible = how_many_visible(grid);
    Ok(num_visible.to_string())
}

fn max_scenic_score(heights: &Grid<u32>) -> u32 {
    println!("r c h n s e w s");

    let (rows, cols) = heights.size();
    let mut max_score = 0;
    for r in 0..rows {
        for c in 0..cols {
            let score = scenic_score(heights, r, c);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn scenic_score(heights: &Grid<u32>, or: usize, oc: usize) -> u32 {
    let (rows, cols) = heights.size();
    let h = heights[or][oc];

    // to the north:
    let mut north_score = 0;
    if or > 0 {
        for r in (0..=or - 1).rev() {
            north_score += 1;
            if heights[r][oc] >= h {
                break;
            }
        }
    }

    // to the south:
    let mut south_score = 0;
    if or < rows - 1 {
        for r in or + 1..rows {
            south_score += 1;
            if heights[r][oc] >= h {
                break;
            }
        }
    }

    // to the east:
    let mut east_score = 0;
    if oc < cols - 1 {
        for c in oc + 1..cols {
            east_score += 1;
            if heights[or][c] >= h {
                break;
            }
        }
    }

    // to the west:
    let mut west_score = 0;
    if oc > 0 {
        for c in (0..=oc - 1).rev() {
            west_score += 1;
            if heights[or][c] >= h {
                break;
            }
        }
    }

    north_score * south_score * east_score * west_score
}

fn part2(input: Vec<String>) -> Result<String> {
    let heights = make_grid(input);
    let score = max_scenic_score(&heights);
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "
30373
25512
65332
33549
35390
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "21");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "8");
    }

    #[test]
    fn test_make_grid() {
        let input = "
12
34
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        let grid = make_grid(input);
        assert_eq!(grid.size(), (2, 2));
        assert_eq!(grid, grid![[1, 2][3, 4]]);
        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(0, 1), Some(&2));
        assert_eq!(grid.get(1, 0), Some(&3));
        assert_eq!(grid.get(1, 1), Some(&4));
    }

    #[test]
    fn test_mark_visible_from_north() {
        let heights = grid![
            [3, 1, 2]
            [0, 4, 5]
            [2, 6, 1]];
        let mut visible = Grid::init(3, 3, false);
        mark_visible(&heights, &mut visible, Direction::North);
        assert_eq!(
            visible,
            grid![
            [true, true, true]
            [false, true, true]
            [false, true, false]]
        );
    }

    #[test]
    fn test_mark_visible_from_south() {
        let heights = grid![
            [3, 1, 2]
            [0, 4, 5]
            [2, 6, 1]];
        let mut visible = Grid::init(3, 3, false);
        mark_visible(&heights, &mut visible, Direction::South);
        assert_eq!(
            visible,
            grid![
            [true, false, false]
            [false, false, true]
            [true, true, true]]
        );
    }

    #[test]
    fn test_mark_visible_from_east() {
        let heights = grid![
            [3, 1, 2]
            [0, 4, 5]
            [2, 6, 1]];
        let mut visible = Grid::init(3, 3, false);
        mark_visible(&heights, &mut visible, Direction::East);
        assert_eq!(
            visible,
            grid![
            [true, false, true]
            [false, false, true]
            [false, true, true]]
        );
    }

    #[test]
    fn test_mark_visible_from_west() {
        let heights = grid![
            [3, 1, 2]
            [0, 4, 5]
            [2, 6, 1]];
        let mut visible = Grid::init(3, 3, false);
        mark_visible(&heights, &mut visible, Direction::West);
        assert_eq!(
            visible,
            grid![
            [true, false, false]
            [true, true, true]
            [true, true, false]]
        );
    }
}
