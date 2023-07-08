use color_eyre::eyre::{eyre, Report, Result};
use common::select_and_solve;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day05.1", part1, "inputs/day05.2", part2)?;
    Ok(())
}

fn part1(input: Vec<String>) -> Result<String> {
    let (drawing, moves) = split_input(input);

    for line in &drawing {
        println!("{}", line);
    }

    let mut state = State::try_from(drawing)?;

    println!("{:?}", state);

    let moves: Vec<Move> = moves
        .iter()
        .map(|s| Move::try_from(s.as_str()))
        .collect::<Result<Vec<Move>>>()?;

    state.do_moves(moves);

    println!("{:?}", state);

    Ok(state.output())
}

fn part2(_input: Vec<String>) -> Result<String> {
    Ok("2".to_owned())
}

fn split_input(mut input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let index = input
        .iter()
        .enumerate()
        .find(|&r| r.1.starts_with("move"))
        .expect("input has move lines")
        .0;

    let moves = input.split_off(index);

    // Remove the last (blank) line of the diagram
    input.pop();

    (input, moves)
}

fn get_num_stacks(s: &str) -> Option<u32> {
    let last_char = s.trim().chars().last()?;
    last_char.to_digit(10)
}

fn stack_offset(i: usize) -> usize {
    // "[x] [x] [x] ..."
    1 + i * 4
}

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn new(num_stacks: u32) -> Self {
        State {
            stacks: vec![vec![]; num_stacks as usize],
        }
    }

    fn add_to_stacks(&mut self, line: &str) -> Result<()> {
        // parse a line and add to stacks
        let num_stacks = self.stacks.len();
        for i in 0..num_stacks {
            let p = stack_offset(i);
            let item = line.chars().nth(p).ok_or(eyre!("line too short"))?;
            if item.is_ascii_uppercase() {
                self.stacks[i].push(item);
            }
        }
        Ok(())
    }

    fn do_moves(&mut self, moves: Vec<Move>) {
        for mv in moves {
            self.do_move(mv);
        }
    }

    fn do_move(&mut self, mv: Move) {
        for _ in 0..mv.count {
            let item = self.stacks[mv.src as usize]
                .pop()
                .expect("stack is not empty");
            self.stacks[mv.dst as usize].push(item);
        }
    }

    fn output(&self) -> String {
        let value: String = self
            .stacks
            .iter()
            .map(|stack| stack.last().expect("valid item"))
            .collect();
        value
    }
}

impl TryFrom<Vec<String>> for State {
    type Error = Report;

    fn try_from(drawing: Vec<String>) -> Result<Self, Self::Error> {
        let stack_labels = drawing.last().ok_or(eyre!("no labels"))?;
        let num_stacks = get_num_stacks(stack_labels).ok_or(eyre!("no stacks"))?;

        // parse the stacks from the bottom up, skipping the label line:
        let mut state = State::new(num_stacks);
        let mut ri = drawing.iter().rev();
        ri.next();
        for line in ri {
            state.add_to_stacks(line)?;
        }
        Ok(state)
    }
}

#[derive(Debug)]
struct Move {
    count: u32,
    src: u32, // zero-based
    dst: u32, // zero-based
}

impl TryFrom<&str> for Move {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();

        if parts.len() < 4 {
            return Err(eyre!("bad move"));
        }

        let count = str::parse::<u32>(parts[1])?;
        let src = str::parse::<u32>(parts[3])? - 1;
        let dst = str::parse::<u32>(parts[5])? - 1;

        Ok(Move { count, src, dst })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "CMZ");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "2");
    }

    #[test]
    fn test_stack_offset() {
        assert_eq!(stack_offset(0), 1);
        assert_eq!(stack_offset(1), 5);
    }

    #[test]
    fn test_get_num_stacks() {
        assert_eq!(get_num_stacks(" 1   2   3 "), Some(3));
        assert_eq!(
            get_num_stacks(" 1   2   3   4   5   6   7   8   9 "),
            Some(9)
        );
        assert_eq!(get_num_stacks(" 1   2   3   4   5   6   7   8   A "), None);
    }

    #[test]
    fn test_try_from_drawing() {
        let drawing: Vec<String> = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let state = State::try_from(drawing).expect("");
        assert_eq!(state.stacks.len(), 3);
        assert_eq!(state.stacks[0], vec!['Z', 'N']);
        assert_eq!(state.stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(state.stacks[2], vec!['P']);
    }
}
