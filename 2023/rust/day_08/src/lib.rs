use std::collections::HashSet;

type NodeName = &'static str;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
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
struct NetworkIter<'a> {
    network: &'a Network,
    command_it: std::iter::Cycle<std::str::Chars<'a>>,
    current_node: Option<&'a Node>,
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

#[derive(PartialEq)]
struct Network {
    commands: &'static str,
    nodes: HashSet<Node>,
}

impl Network {
    fn iter(&self) -> NetworkIter<'_> {
        NetworkIter {
            network: self,
            command_it: self.commands.chars().cycle(),
            current_node: None,
        }
    }
}

impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{:#?}", self.commands, self.nodes))
    }
}

fn parse_network(input: &'static str) -> Network {
    let mut input_it = input.lines();

    let commands = input_it.next().expect("expected first row with commands");
    let mut nodes = HashSet::new();
    input_it
        .skip(1)
        .map(|s| Node {
            name: &s[..3],
            left: &s[7..10],
            right: &s[12..15],
        })
        .for_each(|n| {
            if !nodes.insert(n) {
                panic!("Expected valid input");
            }
        });

    Network { commands, nodes }
}

pub fn part1(input: &'static str) -> usize {
    let network = parse_network(input);
    dbg!(&network);
    network.iter().count() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn parse_input1() {
        let mut nodes = HashSet::new();

        vec![
            Node {
                name: "AAA",
                left: "BBB",
                right: "CCC",
            },
            Node {
                name: "BBB",
                left: "DDD",
                right: "EEE",
            },
            Node {
                name: "CCC",
                left: "ZZZ",
                right: "GGG",
            },
            Node {
                name: "DDD",
                left: "DDD",
                right: "DDD",
            },
            Node {
                name: "EEE",
                left: "EEE",
                right: "EEE",
            },
            Node {
                name: "GGG",
                left: "GGG",
                right: "GGG",
            },
            Node {
                name: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            },
        ]
        .into_iter()
        .for_each(|n| {
            nodes.insert(n);
        });

        let expected = Network {
            commands: "RL",
            nodes,
        };
        assert_eq!(parse_network(TEST_INPUT1), expected);
    }

    #[test]
    fn parse_input2() {
        let mut nodes = HashSet::new();

        vec![
            Node {
                name: "AAA",
                left: "BBB",
                right: "BBB",
            },
            Node {
                name: "BBB",
                left: "AAA",
                right: "ZZZ",
            },
            Node {
                name: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            },
        ]
        .into_iter()
        .for_each(|n| {
            nodes.insert(n);
        });

        let expected = Network {
            commands: "LLR",
            nodes,
        };
        assert_eq!(parse_network(TEST_INPUT2), expected);
    }

    #[test]
    fn part1_input1() {
        for element in parse_network(TEST_INPUT1).iter() {
            dbg!(element);
        }
        assert_eq!(parse_network(TEST_INPUT1).iter().count() - 1, 2)
    }

    #[test]
    fn part1_input2() {
        for element in parse_network(TEST_INPUT2).iter() {
            dbg!(element);
        }
        assert_eq!(parse_network(TEST_INPUT2).iter().count() - 1, 6)
    }
}
