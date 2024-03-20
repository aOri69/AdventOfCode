use crate::NodeName;

#[derive(Clone)]
pub struct Node {
    pub(super) name: NodeName,
    pub(super) left: NodeName,
    pub(super) right: NodeName,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::cmp::Eq for Node {}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        // self.left.hash(state);
        // self.right.hash(state);
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} = ({},{})",
            self.name, self.left, self.right
        ))
    }
}
