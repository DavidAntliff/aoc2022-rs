use color_eyre::eyre::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, space0};
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Item(u32);

#[derive(Debug, PartialEq)]
pub struct MonkeyId(usize);

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug, PartialEq)]
pub struct ThrowTo(MonkeyId, MonkeyId);

#[derive(Debug, PartialEq)]
pub struct Monkey {
    id: MonkeyId,
    items: VecDeque<Item>,
    operation: Operation,
    divisor: u32,
    throw_to: ThrowTo,
}

pub fn parse_monkey_id(i: &str) -> IResult<&str, MonkeyId> {
    let (i, _) = tag("Monkey ")(i)?;
    let (i, id) = nom::character::complete::u32(i)?;
    let (i, _) = char(':')(i)?;
    Ok((i, MonkeyId(id as usize)))

    // map(
    //     preceded(
    //         tag("Monkey "),
    //         terminated(nom::character::complete::u32, char(':')),
    //     ),
    //     |e| MonkeyId(e as usize),
    // )(i)
}

pub fn parse_starting_items(i: &str) -> IResult<&str, Vec<Item>> {
    let (i, _) = space0(i)?;
    let (i, _) = tag("Starting items: ")(i)?;
    let (i, v) = separated_list1(tag(", "), nom::character::complete::u32)(i)?;
    let starting_items: Vec<Item> = v.into_iter().map(Item).collect();
    Ok((i, starting_items))
}

fn parse_operation_add(i: &str) -> IResult<&str, Operation> {
    map(preceded(tag("+ "), nom::character::complete::u32), |x| {
        Operation::Add(x)
    })(i)
}

fn parse_operation_multiply(i: &str) -> IResult<&str, Operation> {
    map(preceded(tag("* "), nom::character::complete::u32), |x| {
        Operation::Multiply(x)
    })(i)
}

fn parse_operation_square(i: &str) -> IResult<&str, Operation> {
    map(tag("* old"), |_| Operation::Square)(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, _) = space0(i)?;
    let (i, _) = tag("Operation: new = old ")(i)?;
    alt((
        parse_operation_add,
        parse_operation_multiply,
        parse_operation_square,
    ))(i)
}

pub fn parse_divisor(i: &str) -> IResult<&str, u32> {
    preceded(tag("  Test: divisible by "), nom::character::complete::u32)(i)
}

pub fn parse_throw_to(i: &str) -> IResult<&str, ThrowTo> {
    let (i, (_, _, if_true, _, _, _, if_false)) = tuple((
        space0,
        tag("If true: throw to monkey "),
        nom::character::complete::u32,
        newline,
        space0,
        tag("If false: throw to monkey "),
        nom::character::complete::u32,
    ))(i)?;
    Ok((
        i,
        ThrowTo(MonkeyId(if_true as usize), MonkeyId(if_false as usize)),
    ))
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, (id, _, items, _, operation, _, divisor, _, throw_to, _)) = tuple((
        parse_monkey_id,
        newline,
        parse_starting_items,
        newline,
        parse_operation,
        newline,
        parse_divisor,
        newline,
        parse_throw_to,
        newline,
    ))(i)?;

    Ok((
        i,
        Monkey {
            id,
            items: VecDeque::from(items),
            operation,
            divisor,
            throw_to,
        },
    ))
}

pub fn parse_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(i)
}

pub fn load_all_monkeys(input: &str) -> Result<Vec<Monkey>> {
    let monkeys = all_consuming(parse_monkeys)(input)
        .map_err(|e| e.to_owned())?
        .1;
    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{
        parse_divisor, parse_monkey, parse_monkey_id, parse_monkeys, parse_operation,
        parse_starting_items, parse_throw_to, Item, Monkey, MonkeyId, Operation, ThrowTo,
    };
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[test]
    fn test_parse_monkey_id() {
        assert_eq!(parse_monkey_id("Monkey 7:"), Ok(("", MonkeyId(7))));
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(
            parse_starting_items("  Starting items: 54"),
            Ok(("", vec!(Item(54))))
        );
        assert_eq!(
            parse_starting_items("  Starting items: 54, 19, 33"),
            Ok(("", vec!(Item(54), Item(19), Item(33))))
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("  Operation: new = old * 19"),
            Ok(("", Operation::Multiply(19)))
        );
        assert_eq!(
            parse_operation("  Operation: new = old + 6"),
            Ok(("", Operation::Add(6)))
        );
        assert_eq!(
            parse_operation("  Operation: new = old * old"),
            Ok(("", Operation::Square))
        );
    }

    #[test]
    fn test_parse_divisor() {
        assert_eq!(parse_divisor("  Test: divisible by 19"), Ok(("", 19)));
    }

    #[test]
    fn test_parse_throw_to() {
        let input = "    If true: throw to monkey 42
    If false: throw to monkey 98";
        assert_eq!(
            parse_throw_to(input),
            Ok(("", ThrowTo(MonkeyId(42), MonkeyId(98))))
        );
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 42:
  Starting items: 65, 78
  Operation: new = old * 3
  Test: divisible by 5
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        assert_eq!(
            parse_monkey(input),
            Ok((
                "",
                Monkey {
                    id: MonkeyId(42),
                    items: VecDeque::from([Item(65), Item(78)]),
                    operation: Operation::Multiply(3),
                    divisor: 5,
                    throw_to: ThrowTo(MonkeyId(2), MonkeyId(3)),
                }
            ))
        );
    }

    #[fixture]
    fn input() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
    }

    #[rstest]
    fn test_parse_monkeys(input: &str) {
        assert_eq!(
            parse_monkeys(input),
            Ok((
                "",
                vec![
                    Monkey {
                        id: MonkeyId(0),
                        items: VecDeque::from([Item(79), Item(98)]),
                        operation: Operation::Multiply(19),
                        divisor: 23,
                        throw_to: ThrowTo(MonkeyId(2), MonkeyId(3)),
                    },
                    Monkey {
                        id: MonkeyId(1),
                        items: VecDeque::from([Item(54), Item(65), Item(75), Item(74)]),
                        operation: Operation::Add(6),
                        divisor: 19,
                        throw_to: ThrowTo(MonkeyId(2), MonkeyId(0)),
                    },
                    Monkey {
                        id: MonkeyId(2),
                        items: VecDeque::from([Item(79), Item(60), Item(97)]),
                        operation: Operation::Square,
                        divisor: 13,
                        throw_to: ThrowTo(MonkeyId(1), MonkeyId(3)),
                    },
                    Monkey {
                        id: MonkeyId(3),
                        items: VecDeque::from([Item(74)]),
                        operation: Operation::Add(3),
                        divisor: 17,
                        throw_to: ThrowTo(MonkeyId(0), MonkeyId(1)),
                    },
                ]
            ))
        )
    }
}
