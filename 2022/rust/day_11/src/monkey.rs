mod item;
mod operation;
mod test;

use std::{collections::VecDeque, str::FromStr};

pub use item::Item;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, line_ending, one_of, space1},
    combinator::{all_consuming, map, map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
pub use operation::{Operation, OperationError, Value};
pub use test::Test;

pub type WorryLevel = u64;
pub type Monkeys = Vec<Monkey>;
pub type Items = VecDeque<Item>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Monkey {
    id: u32,
    items: Items,
    operation: Operation,
    test: Test,
    evaluations_count: WorryLevel,
}

impl Monkey {
    pub fn new(id: u32, items: Items, operation: Operation, test: Test) -> Self {
        Self {
            id,
            items,
            operation,
            test,
            evaluations_count: Default::default(),
        }
    }

    pub fn test(&self) -> &Test {
        &self.test
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn items_mut(&mut self) -> &mut Items {
        &mut self.items
    }

    pub fn items(&self) -> &Items {
        &self.items
    }

    pub fn evaluations_count_mut(&mut self) -> &mut WorryLevel {
        &mut self.evaluations_count
    }

    pub fn evaluations_count(&self) -> WorryLevel {
        self.evaluations_count
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MonkeyError {
    #[error(transparent)]
    InvalidOperation(#[from] OperationError),
}

pub fn parse_items(input: &str) -> IResult<&str, Items> {
    let mut items_parser = preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), map(nom::character::complete::u64, Item::from)),
    );

    let (remaining, items_vec) = items_parser(input)?;
    Ok((remaining, items_vec.into()))
}

pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = preceded(space1, tag("Operation: new = old "))(input)?;
    let (input, (operation, value)) =
        separated_pair(one_of("+-*/"), tag(" "), alphanumeric1)(input)?;
    // remove unwraps
    let value = Value::from_str(value).unwrap();
    let operation = Operation::new(operation, value).unwrap();
    Ok((input, operation))
}

pub fn parse_test(input: &str) -> IResult<&str, Test> {
    let divisible_by = terminated(
        preceded(tag("  Test: divisible by "), nom::character::complete::u64),
        line_ending,
    );
    let if_true_throw_to = terminated(
        preceded(
            tag("    If true: throw to monkey "),
            nom::character::complete::u32,
        ),
        line_ending,
    );
    let if_false_throw_to = preceded(
        tag("    If false: throw to monkey "),
        nom::character::complete::u32,
    );

    map_res(
        tuple((divisible_by, if_true_throw_to, if_false_throw_to)),
        |(divisible_by, if_true_throw_to, if_false_throw_to)| {
            let operation = Operation::new('/', divisible_by.into()).unwrap();
            Ok::<Test, &str>(Test::new(
                operation,
                if_true_throw_to.try_into().unwrap(),
                if_false_throw_to.try_into().unwrap(),
            ))
        },
    )(input)
}

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (remaining, _) = take_until("Monkey")(input)?;
    let (remaining, id) = delimited(
        tag("Monkey "),
        nom::character::complete::u32,
        preceded(tag(":"), line_ending),
    )(remaining)?;
    let (remaining, items) = terminated(parse_items, line_ending)(remaining)?;
    let (remaining, operation) = terminated(parse_operation, line_ending)(remaining)?;
    let (remaining, test) = terminated(parse_test, opt(line_ending))(remaining)?;
    let monkey = Monkey::new(id, items, operation, test);
    Ok((remaining, monkey))
}

pub fn parse_monkeys(input: &str) -> IResult<&str, Monkeys> {
    all_consuming(many0(parse_monkey))(input)
}

pub struct PrettyMonkeysItems<'a>(pub &'a [Monkey]);

impl<'a> std::fmt::Debug for PrettyMonkeysItems<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0.iter() {
            write!(f, "Monkey {}: ", m.id())?;
            writeln!(f, "{:?}", m.items())?;
        }
        Ok(())
    }
}

pub struct PrettyMonkeysEvalCount<'a>(pub &'a [Monkey]);

impl<'a> std::fmt::Debug for PrettyMonkeysEvalCount<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0.iter() {
            write!(f, "Monkey {}: ", m.id())?;
            writeln!(f, "{:?}", m.evaluations_count())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::monkey::{parse_monkeys, Item, Monkey, Operation, Value};

    use super::*;
    use constants::*;
    use nom::Finish;
    use pretty_assertions::assert_eq;

    #[test]
    fn monkeys_to_vec() {
        let (_r, monkeys) = parse_monkeys(MONKEY_INPUT).finish().unwrap();
        // dbg!(monkeys);
        // todo!();
        let monkey_0 = Monkey {
            id: 0,
            items: vec![Item::from(79), Item::from(98)].into(),
            operation: Operation::Multiply(Value::Const(19)),
            test: Test::new(Operation::Divide(Value::Const(23)), 2, 3),
            evaluations_count: 0,
        };
        let monkey_1 = Monkey {
            id: 1,
            items: vec![
                Item::from(54),
                Item::from(65),
                Item::from(75),
                Item::from(74),
            ]
            .into(),
            operation: Operation::Add(Value::Const(6)),
            test: Test::new(Operation::Divide(Value::Const(19)), 2, 0),
            evaluations_count: 0,
        };
        let monkey_2 = Monkey {
            id: 2,
            items: vec![Item::from(79), Item::from(60), Item::from(97)].into(),
            operation: Operation::Multiply(Value::Old),
            test: Test::new(Operation::Divide(Value::Const(13)), 1, 3),
            evaluations_count: 0,
        };
        let monkey_3 = Monkey {
            id: 3,
            items: vec![Item::from(74)].into(),
            operation: Operation::Add(Value::Const(3)),
            test: Test::new(Operation::Divide(Value::Const(17)), 0, 1),
            evaluations_count: 0,
        };
        assert_eq!(monkeys, vec![monkey_0, monkey_1, monkey_2, monkey_3]);
    }

    mod constants {
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
    If false: throw to monkey 1
";
    }
}
