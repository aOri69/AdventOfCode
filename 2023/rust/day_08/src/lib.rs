type NodeName = &'static str;

#[derive(PartialEq)]
struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} = ({},{})",
            self.name, self.left, self.right
        ))
    }
}

#[derive(PartialEq)]
struct Network {
    commands: &'static str,
    nodes: Vec<Node>,
}

impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{:#?}", self.commands, self.nodes))
    }
}

fn parse_network(input: &'static str) -> Network {
    let mut input_it = input.lines();

    Network {
        commands: input_it.next().expect("expected first row with commands"),
        nodes: input_it
            .skip(1)
            .map(|s| Node {
                name: &s[..3],
                left: &s[7..10],
                right: &s[12..15],
            })
            .collect(),
    }
}

pub fn part1(input: &'static str) -> usize {
    let _network = parse_network(input);
    todo!();
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
        let expected = Network {
            commands: "RL",
            nodes: vec![
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
            ],
        };
        assert_eq!(parse_network(TEST_INPUT1), expected);
    }

    #[test]
    fn parse_input2() {
        let expected = Network {
            commands: "LLR",
            nodes: vec![
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
            ],
        };
        assert_eq!(parse_network(TEST_INPUT2), expected);
    }
}
