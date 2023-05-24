use std::env;
use std::error::Error;
use common::load_input;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let input_filename = match args[1].as_str() {
        "1" => "inputs/day01.1",
        "2" => "inputs/day01.2",
        _ => panic!("Invalid part {}", args[1]),
    };
    let input = load_input(input_filename)?;

    let solution = match args[1].as_str() {
        "1" => part1(input),
        "2" => part2(input),
        _ => panic!("Invalid part"),
    };

    println!("{}", solution);
    Ok(())
}

fn part1(input: Vec<String>) -> String {
    let mut sum = 0;
    let mut max = 0;
    for line in input {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    max.to_string()
}

fn part2(input: Vec<String>) -> String {
    let mut sum = 0;
    let mut values = vec![];
    for line in input {
        if line.is_empty() {
            values.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    values.sort();
    values.reverse();
    (values[0] + values[1] + values[2]).to_string()
}
