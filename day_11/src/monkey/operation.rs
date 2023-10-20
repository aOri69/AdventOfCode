use super::Item;

#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error("zero division is not possible")]
    ZeroDivision,
    #[error("unsupported operation \"{0:?}\"")]
    Unsupported(char),
}

#[derive(Debug, thiserror::Error)]
pub enum ValueError {
    #[error("unable to parse \"{0:?}\" as an arithmetic value")]
    ParsingFailed(String),
}

pub type WorryLevel = i32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Const(WorryLevel),
    Old,
}

impl Value {
    fn value_or_old(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Value::Const(c) => *c,
            Value::Old => old,
        }
    }
}

impl From<WorryLevel> for Value {
    fn from(value: WorryLevel) -> Self {
        Value::Const(value)
    }
}

impl std::str::FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<WorryLevel>() {
            return Ok(Self::Const(val));
        }
        if s == "old" {
            return Ok(Self::Old);
        }
        Err(ValueError::ParsingFailed(s.to_owned()))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Const(c) => write!(f, "{c}"),
            Value::Old => write!(f, "old value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Multiply(Value),
    Divide(Value),
    Add(Value),
    Subtract(Value),
}

impl Operation {
    pub fn new(operation: char, value: Value) -> Result<Self, OperationError> {
        match operation {
            '+' => Ok(Self::Add(value)),
            '-' => Ok(Self::Subtract(value)),
            '*' => Ok(Self::Multiply(value)),
            '/' => {
                if value == Value::Const(0) {
                    return Err(OperationError::ZeroDivision);
                }
                Ok(Self::Divide(value))
            }
            _ => Err(OperationError::Unsupported(operation)),
        }
    }

    pub fn evaluate(&self, other: Item) -> WorryLevel {
        let other: WorryLevel = other.into();
        match self {
            Operation::Multiply(v) => v.value_or_old(other) * other,
            Operation::Divide(v) => v.value_or_old(other) / other,
            Operation::Add(v) => v.value_or_old(other) + other,
            Operation::Subtract(v) => v.value_or_old(other) - other,
        }
    }
}

impl std::str::FromStr for Operation {
    type Err = OperationError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Multiply(v) => write!(f, "multiplied by {}", v),
            Operation::Divide(v) => write!(f, "divided by {}", v),
            Operation::Add(v) => write!(f, "increases by {}", v),
            Operation::Subtract(v) => write!(f, "subtracted by {}", v),
        }
    }
}
