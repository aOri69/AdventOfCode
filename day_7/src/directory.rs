//! Directory crate
//! # Purpose
//! Tree structure representation
//! of the file system

use std::rc::Weak;

/// Dummy struct to pretty print Weak pointers not like structs,
/// just inline.
struct PrettyWeakNode<'a>(&'a Weak<Node>);

impl<'a> std::fmt::Debug for PrettyWeakNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("not implemented")?;
        todo!()
    }
}

/// Node builder for construction either Files or Directories
pub struct NodeBuilder {
    name: String,
    size: usize,
    parent: Weak<Node>,
    children: Vec<Node>,
}

impl NodeBuilder {
    pub fn set_size(&mut self, size: usize) -> &mut Self {
        self.size = size;
        self
    }

    pub fn set_parent(&mut self, parent: Weak<Node>) -> &mut Self {
        self.parent = parent;
        self
    }

    pub fn set_children(&mut self, children: Vec<Node>) -> &mut Self {
        self.children = children;
        self
    }

    pub fn build(self) -> Node {
        Node {
            name: self.name,
            size: self.size,
            parent: self.parent,
            children: self.children,
        }
    }
}

/// Main node struct that represents either Directory,
/// or File
pub struct Node {
    name: String,
    size: usize,
    parent: Weak<Node>,
    children: Vec<Node>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("parent", &self.parent)
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
    pub fn new(name: &str, parent: Weak<Node>) -> NodeBuilder {
        NodeBuilder {
            name: name.to_owned(),
            parent,
            size: 0,
            children: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> &Weak<Node> {
        &self.parent
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}
