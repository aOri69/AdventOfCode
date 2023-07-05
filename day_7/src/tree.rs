use std::{ops::Deref, rc::Weak};

pub struct Root;
struct Child;

#[derive(Default)]
pub struct Dir<Relation = Root> {
    name: String,
    parent: Option<Weak<Dir>>,
    child_dirs: Vec<Dir<Child>>,
    files: Vec<(u64, String)>,

    relation: std::marker::PhantomData<Relation>,
}

impl Dir {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn parent(&self) -> Option<&Dir> {
        match &self.parent {
            Some(parent) => todo!(),
            None => None,
        }
    }
}

impl Dir<Root> {
    pub fn new() -> Dir<Root> {
        Dir {
            name: "/".to_string(),
            parent: None,
            child_dirs: Vec::new(),
            files: Vec::new(),
            relation: std::marker::PhantomData::<Root>,
        }
    }
}

impl Dir<Child> {
    pub fn new(name: &str, parent: Weak<Dir>) -> Dir<Child> {
        Dir {
            name: name.to_string(),
            parent: Some(parent),
            child_dirs: Vec::new(),
            files: Vec::new(),
            relation: std::marker::PhantomData::<Child>,
        }
    }
}

impl<Relation> Deref for Dir<Relation> {
    type Target = Dir<Relation>;
    fn deref(&self) -> &Self::Target {
        &self
    }
}
