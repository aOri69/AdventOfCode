use log::debug;

use super::operation::{Operation, WorryLevel};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    pub fn apply(&self, current_worry: WorryLevel) -> usize {
        debug!(
            "    Current worry level {} is not divisible by {}",
            current_worry,
            self.operation.value()
        );

        if let crate::monkey::Value::Const(op_value) = self.operation.value() {
            match current_worry % op_value {
                0 => self.if_true_throw_to as usize,
                _ => self.if_false_throw_to as usize,
            }
        } else {
            panic!("wrong operation in test. should be only division");
        }
    }
}
