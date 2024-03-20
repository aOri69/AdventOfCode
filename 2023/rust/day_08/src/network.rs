mod custom_iter;
mod endless_iter;
mod iter;

use std::collections::HashSet;

use crate::node::Node;

#[derive(PartialEq, Clone)]
pub struct Network {
    pub(super) commands: &'static str,
    pub(super) nodes: HashSet<Node>,
}

impl Network {
    pub fn iter(&self) -> iter::NetworkIter<'_> {
        iter::NetworkIter {
            network: self,
            command_it: self.commands.chars().cycle(),
            current_node: None,
        }
    }

    #[allow(unused)]
    pub fn endless_iter(&self, start_node_name: &'static str) -> endless_iter::NetworkIter<'_> {
        endless_iter::NetworkIter {
            network: self,
            command_it: self.commands.chars().cycle(),
            current_node: None,
            start_node_name,
        }
    }
    pub fn custom_iter(&self, start_node_name: &'static str) -> custom_iter::NetworkIter<'_> {
        custom_iter::NetworkIter {
            network: self,
            command_it: self.commands.chars().cycle(),
            current_node: None,
            start_node_name,
        }
    }
}

impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{:#?}", self.commands, self.nodes))
    }
}
