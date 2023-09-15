#![allow(unused_variables, dead_code)]

use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Item(u32);

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Test {
    divide_by: i32,
    throw_if_true_to: u32,
    throw_if_false_to: u32,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Monkey {
    items: Vec<Item>,
    mult_operation: i32,
    test: Test,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err("not implemented".to_owned())
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    vec![Monkey::default()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_1_monkey() {
        let expected = Monkey {
            items: vec![Item(1), Item(2), Item(3), Item(4), Item(5)],
            mult_operation: 2,
            test: Test {
                divide_by: 2,
                throw_if_true_to: 1,
                throw_if_false_to: 2,
            },
        };
        let result = Monkey::from_str(GENERIC_MONKEY);

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn parse_4_monkeys() {
        let expected = vec![];
        let result = parse_monkeys(MONKEY_INPUT);
        assert_eq!(result, expected);
    }

    mod constants {
        pub const GENERIC_MONKEY: &str = "Monkey 0:
Starting items: 1, 2, 3, 4, 5
Operation: new = old * 2
Test: divisible by 2
  If true: throw to monkey 1
  If false: throw to monkey 2";
        pub const MONKEY_INPUT: &str = "Monkey 0:
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
    If false: throw to monkey 1";
    }
}
