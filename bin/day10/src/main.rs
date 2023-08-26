use color_eyre::eyre::Result;
use common::select_and_solve;
use common::stack::Stack;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map, value};
use nom::sequence::preceded;
use nom::IResult;
use tracing::debug;

fn main() -> Result<()> {
    color_eyre::install()?;

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

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
    // let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
    //     i32::from_str(s)
    // })(input)?;
    //
    // Ok((i, number))
    nom::character::complete::i32(input)
}

fn parse_addx(i: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), parse_i32), Instruction::Addx)(i)
}

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    //map(tag("noop"), |_| Instruction::Noop)(i)
    value(Instruction::Noop, tag("noop"))(i)
}

// or we can write the parse function as an associated function:
impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((parse_addx, parse_noop))(input)
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_addx, parse_noop))(i)
}

fn parse_instructions(input: &[String]) -> Result<Vec<Instruction>> {
    // Lifetime issues if we try to avoid the .unwrap() here, due to
    // the lifetime of 'l' escaping the closure.
    // let instructions: Vec<Instruction> = input
    //     .iter()
    //     .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
    //     .collect();
    // Ok(instructions)

    // E.g. this does not compile:
    // let instructions: Result<Vec<Instruction>> = input
    //     .iter()
    //     .cloned()
    //     .map(|l| {
    //         let (_, instruction) = all_consuming(Instruction::parse)(&l)?;
    //         Ok(instruction)
    //     })
    //     .collect();
    //
    // instructions

    // The issue is that the error type of the parser contains the &str,
    // but if we break this reference by mapping the error to an owned string, it will compile:
    // https://stackoverflow.com/a/73506323/
    let instructions: Result<Vec<Instruction>> = input
        .iter()
        .map(|l| {
            let (_, instruction) =
                all_consuming(Instruction::parse)(l).map_err(|e| e.to_owned())?;
            Ok(instruction)
        })
        .collect();

    instructions
}

struct Registers {
    program_counter: usize, // always points to the *next* instruction
    x: i32,
}

struct Cpu {
    steps: Stack<Instruction>,
    registers: Registers,

    // flags
    halted: bool,
}

impl Cpu {
    fn new() -> Self {
        Self {
            steps: Stack::new(),
            registers: Registers {
                program_counter: 0,
                x: 1,
            },
            halted: false,
        }
    }

    fn load_instruction(&mut self, instruction: &Instruction) {
        debug!("load {instruction:?}");
        match instruction {
            Instruction::Addx(x) => {
                // 2 cycles
                self.steps.push(Instruction::Addx(*x));
                self.steps.push(Instruction::Noop);
            }
            Instruction::Noop => {
                // 1 cycle
                self.steps.push(*instruction);
            }
        }
    }

    fn tick(&mut self) {
        if self.halted {
            debug!("halted");
            return;
        }

        // pop the next step off the internal stack
        let step = self.steps.pop().unwrap();
        match step {
            Instruction::Addx(x) => {
                self.registers.x += x;
                debug!("add {x}, x is now {}", self.registers.x);
            }
            Instruction::Noop => {
                debug!("noop");
            }
        }
    }
}

struct Memory {
    instructions: Vec<Instruction>, // instruction memory
}

impl Memory {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            instructions: instructions.to_vec(),
        }
    }
}

fn cycle(cpu: &mut Cpu, memory: &Memory) {
    // Execute one clock tick
    if cpu.steps.is_empty() {
        let instruction = memory.instructions.get(cpu.registers.program_counter);
        match instruction {
            Some(i) => {
                cpu.registers.program_counter += 1;
                cpu.load_instruction(i);
            }
            None => {
                cpu.halted = true;
            }
        }
    }

    cpu.tick();
}

fn execute(cpu: &mut Cpu, memory: &Memory) -> Vec<i32> {
    let mut trace = vec![];
    let mut cycles = 0;
    while !cpu.halted {
        cycles += 1;
        debug!("cycle {cycles}");
        cycle(cpu, memory);
        trace.push(cpu.registers.x);
    }
    trace
}

fn part1(input: Vec<String>) -> Result<String> {
    let memory = Memory::new(&parse_instructions(&input)?);
    let mut cpu = Cpu::new();

    let trace = execute(&mut cpu, &memory);

    for (i, v) in trace.iter().enumerate() {
        println!("end of cycle {}: x {v}", i + 1);
    }

    println!("during cycle 20: x {}", trace[20 - 1 - 1]);
    println!("during cycle 60: x {}", trace[60 - 1 - 1]);
    println!("during cycle 100: x {}", trace[100 - 1 - 1]);
    println!("during cycle 140: x {}", trace[140 - 1 - 1]);
    println!("during cycle 180: x {}", trace[180 - 1 - 1]);
    println!("during cycle 220: x {}", trace[220 - 1 - 1]);

    let signal_strength = |x: u32| -> i32 { trace[x as usize - 1 - 1] * x as i32 };

    let sum: i32 = [20u32, 60, 100, 140, 180, 220]
        .into_iter()
        .map(signal_strength)
        .sum();

    Ok(sum.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    let memory = Memory::new(&parse_instructions(&input)?);
    let mut cpu = Cpu::new();

    let trace = execute(&mut cpu, &memory);

    for (i, _v) in trace.iter().enumerate() {
        let hor_pos = (i % 40) as i32;

        if hor_pos == 0 {
            println!();
        }

        let during = if i == 0 { 1 } else { trace[i - 1] };
        let range = (during - 1)..=(during + 1);
        if range.contains(&hor_pos) {
            print!("#");
        } else {
            print!(".");
        }
    }

    Ok("2".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use test_log::test; // enable tracing during tests, set RUST_LOG=debug

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
        assert_eq!(part1(input).unwrap(), "13140");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "2");
    }
}
