//! Directory crate
//! # Purpose
//! Tree structure representation
//! of the file system

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Main node struct that represents either Directory,
/// or File
pub struct Node {
    name: String,
    size: u64,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "/".to_string(),
            size: 0,
            parent: Weak::new(),
            children: vec![],
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl Node {
    pub fn new(name: &str, parent: Weak<RefCell<Node>>) -> Self {
        Self {
            name: name.to_owned(),
            parent,
            size: 0,
            children: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> &Weak<RefCell<Node>> {
        &self.parent
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Node>>> {
        &mut self.children
    }
}

/// Dummy struct to pretty print Weak pointers not like structs,
/// just inline.
struct PrettyWeakNode<'a>(&'a Weak<Node>);

impl<'a> std::fmt::Debug for PrettyWeakNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("not implemented")?;
        todo!()
    }
}
