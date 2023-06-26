use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take},
    combinator::{map, opt},
    sequence::delimited,
};

#[derive(Debug)]
pub struct Crate(char);

impl FromStr for Crate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct CrateStack {
    container: Vec<Crate>,
}
impl CrateStack {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }
    pub fn pop(&mut self) -> Option<Crate> {
        self.container.pop()
    }
    pub fn push(&mut self, item: Crate) {
        self.container.push(item)
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
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let mut parser = delimited(tag("["), take(1_usize), tag("]"));
    let result = map(parser, first_char)(s);
}
