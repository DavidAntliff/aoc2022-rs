use color_eyre::eyre::{eyre, Result};
use miette::GraphicalReportHandler;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, space0};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree};
use nom_supreme::final_parser::final_parser;
use std::collections::VecDeque;

pub type Span<'a> = LocatedSpan<&'a str>;

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

pub fn parse_monkey_id<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, MonkeyId, E> {
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

pub fn parse_starting_items<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<Item>, E> {
    let (i, _) = space0(i)?;
    let (i, _) = tag("Starting items: ")(i)?;
    let (i, v) = separated_list1(tag(", "), nom::character::complete::u32)(i)?;
    let starting_items: Vec<Item> = v.into_iter().map(Item).collect();
    Ok((i, starting_items))
}

fn parse_operation_add<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    map(preceded(tag("+ "), nom::character::complete::u32), |x| {
        Operation::Add(x)
    })(i)
}

fn parse_operation_multiply<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    map(preceded(tag("* "), nom::character::complete::u32), |x| {
        Operation::Multiply(x)
    })(i)
}

fn parse_operation_square<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    map(tag("* old"), |_| Operation::Square)(i)
}

pub fn parse_operation<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    let (i, _) = space0(i)?;
    let (i, _) = tag("Operation: new = old ")(i)?;
    alt((
        parse_operation_add,
        parse_operation_multiply,
        parse_operation_square,
    ))(i)
}

pub fn parse_divisor<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, u32, E> {
    preceded(tag("  Test: divisible by "), nom::character::complete::u32)(i)
}

pub fn parse_throw_to<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, ThrowTo, E> {
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

pub fn parse_monkey<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Monkey, E> {
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

pub fn parse_monkeys<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<Monkey>, E> {
    separated_list1(newline, parse_monkey)(i)
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    //src: &'static str,
    src: String,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

pub fn load_all_monkeys(input_static: &str) -> Result<Vec<Monkey>> {
    let input = Span::new(input_static);
    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_monkeys::<ErrorTree<Span>>)(input);
    let monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static.to_owned(),
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            return Err(eyre!(""));
        }
    };
    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{
        parse_divisor, parse_monkey, parse_monkey_id, parse_monkeys, parse_operation,
        parse_starting_items, parse_throw_to, Item, Monkey, MonkeyId, Operation, ThrowTo,
    };
    use nom_supreme::error::ErrorTree;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[test]
    fn test_parse_monkey_id() {
        assert_eq!(
            parse_monkey_id::<ErrorTree<Span>>("Monkey 7:".into())
                .unwrap()
                .1,
            MonkeyId(7)
        );
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(
            parse_starting_items::<ErrorTree<Span>>("  Starting items: 54".into())
                .unwrap()
                .1,
            vec!(Item(54))
        );
        assert_eq!(
            parse_starting_items::<ErrorTree<Span>>("  Starting items: 54, 19, 33".into())
                .unwrap()
                .1,
            vec!(Item(54), Item(19), Item(33))
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation::<ErrorTree<Span>>("  Operation: new = old * 19".into())
                .unwrap()
                .1,
            Operation::Multiply(19)
        );
        assert_eq!(
            parse_operation::<ErrorTree<Span>>("  Operation: new = old + 6".into())
                .unwrap()
                .1,
            Operation::Add(6)
        );
        assert_eq!(
            parse_operation::<ErrorTree<Span>>("  Operation: new = old * old".into())
                .unwrap()
                .1,
            Operation::Square
        );
    }

    #[test]
    fn test_parse_divisor() {
        assert_eq!(
            parse_divisor::<ErrorTree<Span>>("  Test: divisible by 19".into())
                .unwrap()
                .1,
            19
        );
    }

    #[test]
    fn test_parse_throw_to() {
        let input = "    If true: throw to monkey 42
    If false: throw to monkey 98";
        assert_eq!(
            parse_throw_to::<ErrorTree<Span>>(input.into()).unwrap().1,
            ThrowTo(MonkeyId(42), MonkeyId(98))
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
            parse_monkey::<ErrorTree<Span>>(input.into()).unwrap().1,
            Monkey {
                id: MonkeyId(42),
                items: VecDeque::from([Item(65), Item(78)]),
                operation: Operation::Multiply(3),
                divisor: 5,
                throw_to: ThrowTo(MonkeyId(2), MonkeyId(3)),
            }
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
            parse_monkeys::<ErrorTree<Span>>(input.into()).unwrap().1,
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
        )
    }

    #[rstest]
    fn test_load_all_monkeys(input: &str) {
        let monkeys = load_all_monkeys(input).unwrap();
        assert_eq!(monkeys.len(), 4);
    }
}
