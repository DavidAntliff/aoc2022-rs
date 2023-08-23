use color_eyre::eyre::Result;
use common::select_and_solve;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::sequence::preceded;
use nom::{Finish, IResult};
use std::str::FromStr;

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

// https://stackoverflow.com/a/74809016/
fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}

fn parse_addx(i: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), parse_i32), Instruction::Addx)(i)
}

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_addx, parse_noop))(i)
}

fn parse_instructions(input: &Vec<String>) -> Result<Vec<Instruction>> {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|l| all_consuming(parse_instruction)(l).finish().unwrap().1)
        .collect();
    Ok(instructions)
}

struct Registers {
    program_counter: usize, // always points to the currently-executing instruction
    x: i32,
    addi_pending: i32,
}

struct Cpu {
    halted: bool,
    instructions: Vec<Instruction>, // instruction memory
    cycles: u32,                    // number of cycles remaining for the current instruction
    registers: Registers,
}

impl Cpu {
    fn new(instructions: &Vec<Instruction>) -> Self {
        let mut cpu = Self {
            halted: false,
            instructions: instructions.clone(),
            cycles: 0,
            registers: Registers {
                program_counter: 0,
                x: 1,
                addi_pending: 0,
            },
        };

        cpu.load_instruction();

        cpu
    }

    fn load_instruction(&mut self) {
        if self.registers.program_counter < self.instructions.len() {
            match self.instructions[self.registers.program_counter] {
                Instruction::Addx(x) => {
                    self.registers.addi_pending = x;
                    self.cycles = 2;
                }
                Instruction::Noop => {
                    self.cycles = 1;
                }
            }
        } else {
            self.halted = true;
        }
    }

    fn tick(&mut self) {
        if self.halted {
            return;
        }

        let current_instruction = self.instructions[self.registers.program_counter];
        self.cycles -= 1;
        if self.cycles == 0 {
            match current_instruction {
                Instruction::Addx(_) => {
                    self.registers.x += self.registers.addi_pending;
                }
                Instruction::Noop => {}
            }
            self.registers.program_counter += 1;
            self.load_instruction();
        }
    }

    fn execute(&mut self) {
        let mut i = 0;
        while !self.halted {
            self.tick();
            i += 1;
            println!("{i}: {}", self.registers.x);
        }
    }
}

fn part1(input: Vec<String>) -> Result<String> {
    let instructions = parse_instructions(&input)?;
    let mut cpu = Cpu::new(&instructions);

    cpu.execute();

    Ok("1".to_owned())
}

fn part2(_input: Vec<String>) -> Result<String> {
    Ok("2".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("0"), Ok(("", 0)));
        assert_eq!(parse_i32("-1"), Ok(("", -1)));
        assert_eq!(parse_i32("45"), Ok(("", 45)));
        assert_eq!(parse_i32("-123"), Ok(("", -123)));
    }

    #[test]
    fn test_parse_addx() {
        assert_eq!(parse_addx("addx 0"), Ok(("", Instruction::Addx(0))));
        assert_eq!(parse_addx("addx -1"), Ok(("", Instruction::Addx(-1))));
        assert_eq!(parse_addx("addx 45"), Ok(("", Instruction::Addx(45))));
        assert_eq!(parse_addx("addx -123"), Ok(("", Instruction::Addx(-123))));
    }

    #[test]
    fn test_parse_noop() {
        assert_eq!(parse_noop("noop"), Ok(("", Instruction::Noop)));
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("addx 0"), Ok(("", Instruction::Addx(0))));
        assert_eq!(
            parse_instruction("addx -123"),
            Ok(("", Instruction::Addx(-123)))
        );
        assert_eq!(parse_instruction("noop"), Ok(("", Instruction::Noop)));
    }

    #[test]
    fn test_parse_instructions() {
        let input: Vec<String> = "
addx 15
addx -11
noop
addx 6
noop
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

        assert_eq!(
            parse_instructions(&input).unwrap(),
            vec![
                Instruction::Addx(15),
                Instruction::Addx(-11),
                Instruction::Noop,
                Instruction::Addx(6),
                Instruction::Noop,
            ]
        )
    }

    #[fixture]
    fn input() -> Vec<String> {
        "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
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
