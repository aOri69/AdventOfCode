pub trait Calculated {}

#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error("zero division is not possible")]
    ZeroDivision,
    #[error("unsupported operation \"{0:?}\"")]
    Unsupported(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Multiply(i32),
    Power(i32),
    Divide(i32),
    Add(i32),
    Subtract(i32),
}

impl Operation {
    pub fn new(operation: char, value: i32) -> Result<Self, OperationError> {
        match operation {
            '+' => Ok(Self::Add(value)),
            '-' => Ok(Self::Subtract(value)),
            '*' => Ok(Self::Multiply(value)),
            '^' => Ok(Self::Power(value)),
            '/' => {
                if value == 0 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, val) = s.split_at(1);
        todo!()
    }
}
