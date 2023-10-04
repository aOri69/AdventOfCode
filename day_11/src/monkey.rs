mod item;
mod operation;
mod parser;
mod test;

use std::str::FromStr;

pub use item::Item;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{alpha1, alphanumeric1, digit1, line_ending, one_of, space1},
        is_digit,
    },
    combinator::{all_consuming, map, map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
pub use operation::{Operation, OperationError, Value, ValueError};
pub use test::Test;

pub type Monkeys = Vec<Monkey>;
pub type Items = Vec<Item>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monkey {
    id: u32,
    items: Items,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn new(id: u32, items: Items, operation: Operation, test: Test) -> Self {
        Self {
            id,
            items,
            operation,
            test,
        }
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
        separated_list0(tag(", "), map(nom::character::complete::u32, Item::from)),
    );

    items_parser(input)
}

pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = preceded(space1, tag("Operation: new = old "))(input)?;
    let (input, (op, val)) = separated_pair(one_of("+-*/"), tag(" "), alphanumeric1)(input)?;
    dbg!(op, val);
    let val = Value::from_str(val)?;
    // Ok((input, Operation::Add(2.into())))
}

pub fn parse_test(input: &str) -> IResult<&str, Test> {
    let divisible_by = terminated(
        preceded(tag("  Test: divisible by "), nom::character::complete::i32),
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
            Ok::<Test, &str>(Test::new(operation, if_true_throw_to, if_false_throw_to))
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
