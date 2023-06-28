use std::{fmt::Display, num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Clone, Copy)]
pub struct Crate(char);

impl Deref for Crate {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl std::fmt::Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{self}").as_str())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stack(Vec<Crate>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn pop(&mut self) -> Option<Crate> {
        self.0.pop()
    }
    pub fn push(&mut self, item: Crate) {
        self.0.push(item)
    }
    pub fn top(&self) -> Option<Crate> {
        self.0.last().copied()
    }
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn parse_into_crates(s: &str) -> Vec<Option<Crate>> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk_crate| {
            // println!("chunk_crate: {chunk_crate:?}");
            match chunk_crate.get(1) {
                Some(' ') => None,
                Some(c) => Some(Crate(*c)),
                None => panic!("Don't know how this could have happen"),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct MoveCommand {
    count: usize,
    from: usize,
    to: usize,
}

impl MoveCommand {
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn from(&self) -> usize {
        self.from
    }
    pub fn to(&self) -> usize {
        self.to
    }
}

impl FromStr for MoveCommand {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<_>>();

        if tokens.len() != 6 {
            return Err(CommandError::WrongLength);
        }
        if !tokens.first().is_some_and(|t| *t == "move") {
            return Err(CommandError::WrongCommand);
        }

        Ok(MoveCommand {
            count: tokens.get(1).unwrap().parse()?,
            from: tokens.get(3).unwrap().parse()?,
            to: tokens.get(5).unwrap().parse()?,
        })
    }
}

pub fn move_crate(stacks: &mut Vec<Stack>, command: MoveCommand) {
    for _ in 1..=command.count() {
        let poped_from = stacks.get_mut(command.from() - 1).unwrap().pop().unwrap();
        let to = stacks.get_mut(command.to() - 1).unwrap();
        to.push(poped_from);
    }
}

#[derive(Debug)]
pub enum CommandError {
    ParseInt(ParseIntError),
    WrongLength,
    WrongCommand,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::ParseInt(inner) => {
                f.write_str(format!("failed to parse integer: {inner}").as_str())
            }
            CommandError::WrongLength => {
                f.write_str("expected 6 tokens in command line separated by space")
            }
            CommandError::WrongCommand => f.write_str("Wrong command"),
        }
    }
}

impl From<ParseIntError> for CommandError {
    fn from(error: ParseIntError) -> Self {
        CommandError::ParseInt(error)
    }
}

impl std::error::Error for CommandError {}
