use color_eyre::eyre::{eyre, Context, Result};
use std::env;
use std::io::Read;

// https://stackoverflow.com/a/45145246
#[macro_export]
macro_rules! vec_of_strings {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

pub fn load_input(filename: &str) -> Result<Vec<String>> {
    let mut file = std::fs::File::open(filename).wrap_err(format!("opening {}", filename))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .wrap_err(format!("reading {}", filename))?;
    Ok(content.lines().map(String::from).collect())

    // Alternatively, this function could just return the
    // BufRead 'content', permitting client code to call
    // lines() or whatever is required.
}

pub fn solve(input_filename: &str, func: impl Fn(Vec<String>) -> Result<String>) -> Result<String> {
    let input_data = load_input(input_filename)?;
    func(input_data)
}

pub fn select_and_solve<F1, F2>(input1: &str, part1: F1, input2: &str, part2: F2) -> Result<String>
where
    F1: Fn(Vec<String>) -> Result<String>,
    F2: Fn(Vec<String>) -> Result<String>,
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(eyre!("Specify part number 1 or 2"));
    }

    //let solution = match args.get(1).map(|s| s.as_str()) {
    let solution = match args[1].as_str() {
        "1" => solve(input1, part1),
        "2" => solve(input2, part2),
        _ => Err(eyre!("Invalid part number {}", args[1])),
    };

    match solution {
        Ok(result) => {
            println!("{}", result);
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}
