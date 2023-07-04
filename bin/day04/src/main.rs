// Let's learn something new: "nom"

use color_eyre::eyre::Result;
use common::select_and_solve;

// nom imports
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1},
    combinator::map_res,
    error::Error,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};
use std::str::FromStr;

fn main() -> Result<()> {
    color_eyre::install()?;
    select_and_solve("inputs/day04.1", part1, "inputs/day04.2", part2)?;
    Ok(())
}

// will recognize "NNN", "NNN" in "NNN-NNN"
fn parse_range(input: &str) -> IResult<&str, (&str, &str)> {
    let (i, start) = take_while(|c: char| c.is_numeric())(input)?;
    let (i, _) = tag("-")(i)?;
    let (i, end) = take_while(|c: char| c.is_numeric())(i)?;
    Ok((i, (start, end)))
}

// will recognize "XXX", "XXX" in "XXX,XXX"
fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    let (i, first) = take_while(|c: char| c != ',')(input)?;
    let (i, _) = tag(",")(i)?;
    let (i, second) = take_while(|c: char| c != ',')(i)?;
    Ok((i, (first, second)))
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize, // inclusive
}

impl FromStr for Range {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_range(s).finish() {
            Ok((_remaining, (start, end))) => Ok(Range {
                start: str::parse::<usize>(start).expect("should be convertible"),
                end: str::parse::<usize>(end).expect("should be convertible"),
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
struct Line {
    first: Range,
    second: Range,
}

impl FromStr for Line {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_line(s).finish() {
            Ok((_remaining, (first, second))) => Ok(Line {
                first: Range::from_str(first).expect("should be convertible"),
                second: Range::from_str(second).expect("should be convertible"),
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

// Now let's try a simpler parser:
fn nom_parse(s: &str) -> IResult<&str, Line> {
    // Since we wish to use 'position' multiple times in the parser, we need to
    // implement our own copy via a function wrapper:
    // https://stackoverflow.com/questions/70236597/why-cant-i-use-the-same-parser-twice-in-a-tuple
    //let mut position = map_res(digit1, |s: &str| s.parse::<usize>());
    fn position(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    let (s, ((pos1, pos2), _, (pos3, pos4))) = tuple((
        separated_pair(position, char('-'), position),
        char(','),
        separated_pair(position, char('-'), position),
    ))(s)?;

    // Or we can use nested seperated_pair calls:
    // let (s, (((pos1, pos2), (pos3, pos4)),)) = tuple((separated_pair(
    //     separated_pair(position, char('-'), position),
    //     char(','),
    //     separated_pair(position, char('-'), position),
    // ),))(s)?;

    Ok((
        s,
        Line {
            first: Range {
                start: pos1,
                end: pos2,
            },
            second: Range {
                start: pos3,
                end: pos4,
            },
        },
    ))
}

fn get_lines(input: Vec<String>) -> Result<Vec<Line>> {
    let lines_results: Vec<IResult<_, Line>> = input.iter().map(|s| nom_parse(s)).collect();

    let lines: Vec<Line> = lines_results.into_iter().map(|x| x.unwrap().1).collect();
    Ok(lines)
}

fn part1(input: Vec<String>) -> Result<String> {
    let num = get_lines(input)?
        .iter()
        .filter(|x| {
            (x.first.start >= x.second.start && x.first.end <= x.second.end)
                || (x.second.start >= x.first.start && x.second.end <= x.first.end)
        })
        .count();

    Ok(num.to_string())
}

fn part2(input: Vec<String>) -> Result<String> {
    let num = get_lines(input)?
        .iter()
        .filter(|x| (x.first.start <= x.second.end && x.first.end >= x.second.start))
        .count();

    Ok(num.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {
        assert_eq!(part1(input).unwrap(), "2");
    }

    #[rstest]
    fn test_part2(input: Vec<String>) {
        assert_eq!(part2(input).unwrap(), "4");
    }

    #[test]
    fn test_parse_range() {
        let r = Range::from_str("123-456").unwrap();
        dbg!(&r);
        assert_eq!(r.start, 123);
        assert_eq!(r.end, 456);
    }

    #[test]
    fn test_parse_line() {
        let l = Line::from_str("123-456,789-1012").unwrap();
        dbg!(&l);
        assert_eq!(l.first.start, 123);
        assert_eq!(l.first.end, 456);
        assert_eq!(l.second.start, 789);
        assert_eq!(l.second.end, 1012);
    }

    #[test]
    fn test_nom_parse() {
        let (_, l) = nom_parse("123-456,789-1012").unwrap();
        dbg!(&l);
        assert_eq!(l.first.start, 123);
        assert_eq!(l.first.end, 456);
        assert_eq!(l.second.start, 789);
        assert_eq!(l.second.end, 1012);
    }
}
