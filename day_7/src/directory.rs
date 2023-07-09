use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type ParentPtr = RefCell<Weak<Node>>;
pub type ChildrenPtr = RefCell<Vec<Rc<Node>>>;

pub struct Node {
    pub name: String,
    pub parent: ParentPtr,
    pub children: ChildrenPtr,
}

pub struct File(u64, String);
