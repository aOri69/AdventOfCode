use std::{rc::Rc, str::FromStr};
use thiserror::Error;

pub trait Appliable {
    type ApplyResult;
    type ApplyError;

    fn is_appliable() -> bool;
    fn apply() -> Result<Self::ApplyResult, Self::ApplyError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item(u32);

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Item(value)
    }
}

#[derive(Debug, Error, PartialEq, PartialOrd)]
pub enum OperationError {
    #[error("zero division is not possible")]
    ZeroDivision,
    #[error("unknown operation error")]
    Other,
}

pub enum Operation {
    Multiply(i32),
    Divide(i32),
    Add(i32),
    Subtract(i32),
}

impl FromStr for Operation {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub struct Test {
    operation: Operation,
    if_true_throw_to: Rc<Monkey>,
    if_false_throw_to: Rc<Monkey>,
}

pub struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
        // match parse_monkey(s).finish() {
        //     Ok((_remaining, monkey)) => Ok(monkey),
        //     Err(e) => Err("Error"),
        // }
    }
}

pub type Monkeys = Vec<Monkey>;
pub type Items = Vec<Item>;
