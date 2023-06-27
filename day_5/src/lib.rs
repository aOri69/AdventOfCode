use std::{fmt::Display, ops::Deref};

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
