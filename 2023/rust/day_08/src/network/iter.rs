use crate::{node::Node, END_NODE, START_NODE};

use super::Network;

pub struct NetworkIter<'a> {
    pub(super) network: &'a Network,
    pub(super) command_it: std::iter::Cycle<std::str::Chars<'a>>,
    pub(super) current_node: Option<&'a Node>,
}

impl<'a> Iterator for NetworkIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node {
            Some(node) => match node.name {
                END_NODE => None,
                _ => match self.command_it.next() {
                    Some(c) => {
                        let result = match c {
                            'L' => self.network.nodes.get(&Node {
                                name: node.left,
                                left: "",
                                right: "",
                            }),
                            'R' => self.network.nodes.get(&Node {
                                name: node.right,
                                left: "",
                                right: "",
                            }),
                            _ => unreachable!(),
                        };
                        self.current_node = result;
                        result
                    }
                    None => unreachable!(),
                },
            },
            None => {
                self.current_node = self.network.nodes.get(&Node {
                    name: START_NODE,
                    left: "",
                    right: "",
                });
                self.current_node
            }
        }
    }
}
