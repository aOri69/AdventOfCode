use std::str::FromStr;

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
    #[error("unsupported operation")]
    UnsupportedOperation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl TryFrom<(char, i32)> for Operation {
    type Error = OperationError;

    fn try_from(value: (char, i32)) -> Result<Self, Self::Error> {
        let result = match value.0 {
            '+' => Self::Add(value.1),
            '-' => Self::Subtract(value.1),
            '*' => Self::Multiply(value.1),
            '/' => {
                if value.1 == 0 {
                    return Err(OperationError::ZeroDivision);
                }
                Self::Divide(value.1)
            }
            _ => return Err(OperationError::UnsupportedOperation),
        };
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test {
    operation: Operation,
    if_true_throw_to: i32,
    if_false_throw_to: i32,
}

impl TryFrom<(Operation, i32, i32)> for Test {
    type Error = OperationError;

    fn try_from(value: (Operation, i32, i32)) -> Result<Self, Self::Error> {
        Ok(Test {
            operation: value.0,
            if_true_throw_to: value.1,
            if_false_throw_to: value.2,
        })
    }
}
