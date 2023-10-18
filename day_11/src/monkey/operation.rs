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

type WorryLevel = i32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Const(WorryLevel),
    Old,
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
}

impl std::str::FromStr for Operation {
    type Err = OperationError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
