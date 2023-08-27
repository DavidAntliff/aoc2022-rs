mod parse;

use crate::parse::load_all_monkeys;
use color_eyre::eyre::Result;
use common::{load_file, select};

fn main() -> Result<()> {
    color_eyre::install()?;
    let name = env!("CARGO_PKG_NAME");
    select(
        format!("inputs/{name}.1").as_str(),
        part1_load,
        format!("inputs/{name}.2").as_str(),
        part2_load,
    )?;
    Ok(())
}

fn part1_load(filename: &str) -> Result<String> {
    part1(load_file(filename)?.as_str())
}

fn part1(input: &str) -> Result<String> {
    let mut monkeys = load_all_monkeys(input)?;

    let mut num_inspections = vec![0; monkeys.len()];
    let num_rounds = 20;

    for round in 1..=num_rounds {
        println!("Round {round}");
        //        for monkey in monkeys.iter_mut() {
        for i in 0..monkeys.len() {
            // we can't have multiple mutable references to monkeys,
            // so we will need to make a copy of the items list and then
            // clear the list later

            println!("Monkey {}:", monkeys[i].id.0);

            let mc;
            {
                let monkey = &mut monkeys[i];
                mc = monkey.clone();
                num_inspections[i] += mc.items.len() as u32;
            }

            for item in mc.items.iter().copied() {
                println!(
                    "  Monkey inspects an item with a worry level of {}.",
                    item.0
                );

                let new_item = item.do_operation(&mc.operation);
                let new_item = new_item.do_relief();

                let test_result = new_item.is_divisible_by(mc.divisor);

                let throw_to = match test_result {
                    true => &mc.throw_to.0,
                    false => &mc.throw_to.1,
                };

                println!(
                    "    Item with worry level {} is thrown to monkey {}.",
                    new_item.0, throw_to.0
                );
                monkeys[throw_to.0].items.push_back(new_item);
            }
            monkeys[i].items.clear();
        }

        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round
        );
        for i in 0..monkeys.len() {
            println!("Monkey {i}: {:?}", monkeys[i].items);
        }
    }

    // find the two largest numbers
    num_inspections.sort();
    let n = num_inspections.len();
    let solution = num_inspections[n - 1] * num_inspections[n - 2];

    Ok(solution.to_string())
}

fn part2_load(filename: &str) -> Result<String> {
    part2(load_file(filename)?.as_str())
}

fn part2(input: &str) -> Result<String> {
    let mut monkeys = load_all_monkeys(input)?;

    let mut num_inspections = vec![0; monkeys.len()];
    let num_rounds = 10000;

    let divisor_product: u64 = monkeys.iter().map(|m| m.divisor).product();
    println!("divisor product {divisor_product}");

    for round in 1..=num_rounds {
        println!("Round {round}");
        for i in 0..monkeys.len() {
            // we can't have multiple mutable references to monkeys,
            // so we will need to make a copy of the items list and then
            // clear the list later

            //println!("Monkey {}:", monkeys[i].id.0);

            let mc;
            {
                let monkey = &mut monkeys[i];
                mc = monkey.clone();
                num_inspections[i] += mc.items.len() as u32;
            }

            for item in mc.items.iter().copied() {
                // println!(
                //     "  Monkey inspects an item with a worry level of {}.",
                //     item.0
                // );

                let item = parse::Item(item.0 % divisor_product);
                let item = item.do_operation(&mc.operation);
                //let new_item = new_item.do_relief();

                let test_result = item.is_divisible_by(mc.divisor);

                let throw_to = match test_result {
                    true => &mc.throw_to.0,
                    false => &mc.throw_to.1,
                };

                // println!(
                //     "    Item with worry level {} is thrown to monkey {}.",
                //     new_item.0, throw_to.0
                // );
                monkeys[throw_to.0].items.push_back(item);
            }
            monkeys[i].items.clear();
        }

        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round
        );
        for i in 0..monkeys.len() {
            println!("Monkey {i}: {:?}", monkeys[i].items);
        }
    }

    // find the two largest numbers
    num_inspections.sort();
    let n = num_inspections.len();
    let solution: u64 = num_inspections[n - 1] as u64 * num_inspections[n - 2] as u64;

    Ok(solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

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
    fn test_part1(input: &str) {
        assert_eq!(part1(input).unwrap(), "10605");
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(part2(input).unwrap(), "2");
    }
}
