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

impl From<OperationError> for &str {
    fn from(value: OperationError) -> Self {
        value.into()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Multiply(i32),
    Divide(i32),
    Add(i32),
    Subtract(i32),
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

impl Test {
    pub fn new(operation: Operation, if_true_throw_to: i32, if_false_throw_to: i32) -> Self {
        Self {
            operation,
            if_true_throw_to,
            if_false_throw_to,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod items_parser {
        use crate::monkey::parser::parse_items;

        use super::*;
        use nom::Finish;
        use pretty_assertions::assert_eq;

        #[test]
        fn starting_items_empty() {
            const STARTING_ITEMS_EMPTY: &str = "  Starting items: ";
            let expected: Vec<Item> = vec![];
            let (_remaining, result) = parse_items(STARTING_ITEMS_EMPTY).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn starting_items_non_empty() {
            const STARTING_ITEMS: &str = "  Starting items: 54, 65, 75, 74";
            const STARTING_ITEMS_CRLF: &str = "  Starting items: 54, 65, 75, 74\r\n";
            let expected = vec![
                Item::from(54),
                Item::from(65),
                Item::from(75),
                Item::from(74),
            ];
            let (_remaining, result) = parse_items(STARTING_ITEMS).finish().unwrap();
            assert_eq!(expected, result);

            let (_remaining, result) = parse_items(STARTING_ITEMS_CRLF).finish().unwrap();
            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod operation_parser {
        use nom::Finish;
        use pretty_assertions::assert_eq;

        use crate::monkey::{parser::parse_operation, Operation};

        #[test]
        fn add() {
            const OPERATION_OK: &str = "  Operation: new = old + 2";
            let expected: Operation = Operation::Add(2);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn multiply() {
            const OPERATION_OK: &str = "  Operation: new = old * 3";
            let expected: Operation = Operation::Multiply(3);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn divide_ok() {
            const OPERATION_OK: &str = "  Operation: new = old / 2";
            let expected: Operation = Operation::Divide(2);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        #[should_panic]
        fn divide_fail() {
            const OPERATION_OK: &str = "  Operation: new = old / 0";
            let _result = parse_operation(OPERATION_OK).finish().unwrap();
        }

        #[test]
        #[should_panic]
        fn add_fail() {
            const OPERATION_INVALID: &str = "  Operation: new = old / 0";
            let _result = parse_operation(OPERATION_INVALID).finish().unwrap();
        }
    }

    #[cfg(test)]
    mod test_parser {
        use crate::monkey::parser::parse_test;

        use super::*;
        use nom::Finish;
        use pretty_assertions::assert_eq;

        #[test]
        fn simple() {
            const TEST_OK: &str = "  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 2
            ";
            let expected: Test = Test {
                operation: Operation::Divide(2),
                if_true_throw_to: 1,
                if_false_throw_to: 2,
            };
            let (_remaining, result) = parse_test(TEST_OK).finish().unwrap();
            assert_eq!(expected, result);
        }
    }

    mod monkey_parser {
        use nom::Finish;

        use crate::monkey::{
            parser::parse_monkey,
            utils::{Item, Test},
            Monkey,
        };

        #[test]
        fn one_monkey_ok() {
            pub const GENERIC_MONKEY: &str = "Monkey 0:
  Starting items: 1, 2, 3, 4, 5
  Operation: new = old * 2
  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 2
";

            let expected = Monkey::new(
                vec![Item(1), Item(2), Item(3), Item(4), Item(5)],
                crate::monkey::utils::Operation::Multiply(2),
                Test::new(crate::monkey::utils::Operation::Divide(2), 1, 2),
            );

            let (_remaining, result) = parse_monkey(GENERIC_MONKEY).finish().unwrap();
            assert_eq!(expected, result);
        }
    }
}
