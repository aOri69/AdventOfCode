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

    pub fn children(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.children
    }

    pub fn is_dir(&self) -> bool {
        self.size != 0 && self.children.is_empty()
    }

    pub fn size(&self) -> u64 {
        let children_size = self
            .children
            .iter()
            .map(|child| child.borrow().size())
            .sum::<u64>();
        self.size + children_size
    }
}

pub struct PrettyNode<'a>(pub &'a Rc<RefCell<Node>>);

impl<'a> std::fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file size={})", this.size)?;
        }
        for child in &this.children {
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{} {}", child.borrow().name, line)?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}
