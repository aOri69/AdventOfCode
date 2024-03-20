use std::collections::HashSet;

type NodeName = &'static str;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";
const START_NODE_CHAR: char = 'A';
const END_NODE_CHAR: char = 'Z';

mod network;
mod node;

use network::*;
use node::*;

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
    network.iter().count() - 1
}

pub fn part2(input: &'static str) -> usize {
    let network = parse_network(input);
    let mut start_nodes = network.nodes.clone();
    start_nodes.retain(|n| n.name.ends_with(START_NODE_CHAR));
    dbg!(&start_nodes);

    let mut v_it = Vec::new();
    for Node { name, .. } in start_nodes {
        v_it.push(network.custom_iter(name));
    }

    let cycles = v_it.iter_mut().map(|it| it.count() - 1).collect::<Vec<_>>();
    dbg!(&cycles);

    lcm(&cycles)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    const TEST_INPUT_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

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
        assert_eq!(part1(TEST_INPUT1), 2)
    }

    #[test]
    fn part1_input2() {
        assert_eq!(part1(TEST_INPUT2), 6)
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(TEST_INPUT_PART2), 6)
    }
}
