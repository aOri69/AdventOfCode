use super::operation::Operation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test {
    operation: Operation,
    if_true_throw_to: u32,
    if_false_throw_to: u32,
}

impl Test {
    pub fn new(operation: Operation, if_true_throw_to: u32, if_false_throw_to: u32) -> Self {
        Self {
            operation,
            if_true_throw_to,
            if_false_throw_to,
        }
    }
}
