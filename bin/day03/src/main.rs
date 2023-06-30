use std::collections::HashSet;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    common::select_and_solve("inputs/day03.1", part1, "inputs/day03.2", part2)
}

struct Backpack<'a>(&'a str, &'a str);

impl<'a> TryFrom<&'a str> for Backpack<'a> {
    type Error = color_eyre::Report;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let len = value.len();
        match len % 2 {
            0 => {
                let (a, b) = value.split_at(len / 2);
                Ok(Backpack(a, b))
            }
            _ => Err(color_eyre::eyre::eyre!("not even")),
        }
    }
}

struct Backpack2(HashSet<char>, HashSet<char>);

// TODO: this would be better...
// impl TryFrom<&str> for Backpack2 {
//     type Error = color_eyre::Report;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let len = value.len();
//         match len % 2 {
//             0 => {
//                 let (a, b) = value.split_at(len / 2);
//                 Ok(Backpack2(a.chars().collect(), b.chars().collect()))
//             }
//             _ => Err(color_eyre::eyre::eyre!("not even")),
//         }
//     }
// }

impl From<&str> for Backpack2 {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_at(value.len() / 2);
        Backpack2(a.chars().collect(), b.chars().collect())
    }
}

impl Backpack2 {
    fn foo(value: &str) -> Backpack2 {
        let (a, b) = value.split_at(value.len() / 2);
        Backpack2(a.chars().collect(), b.chars().collect())
    }
}

// TODO: why can't we implement From<&str> and have it work below?
fn part1(input: Vec<String>) -> String {
    //let backpacks: Vec<Backpack2> = input.iter().map(|x| Backpack2::from(x)).collect();
    let backpacks: Vec<Backpack2> = input.iter().map(|x| Backpack2::foo(x)).collect();
    "1".to_owned()
}

fn part2(_input: Vec<String>) -> String {
    "2".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> Vec<String> {
        "
"
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    #[rstest]
    fn test_part1(input: Vec<String>) {}

    #[rstest]
    fn test_backpack_tryfrom() {
        let b = Backpack::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(b.0, "abc");
        assert_eq!(b.1, "DEF");
    }

    #[rstest]
    fn test_backpack2_tryfrom() {
        let b = Backpack2::try_from("abcDEF");
        let b = b.expect("not error");
        assert_eq!(b.0, "abc".chars().collect());
        assert_eq!(b.1, "DEF".chars().collect());
    }
}
