mod hand;

use hand::parse_hands;

/// Now, you can determine the total winnings of this set of hands
/// by adding up the result of multiplying each hand's bid with its rank
pub fn part1(input: &str) -> usize {
    let mut hands = parse_hands(input).expect("expected successful parsing");

    hands.sort_by(|a, b| {
        // dbg!(a, b, a.compare(b));
        a.compare(b)
    });

    // dbg!(&hands);

    // use std::fs::OpenOptions;
    // use std::io::prelude::*;

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .create(true)
    //     .open("part1_debug.txt")
    //     .unwrap();

    let result = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        // dbg!(acc, hand, index + 1, hand.bid() * (index + 1));
        // dbg!(acc + (hand.bid() * (index + 1)));

        // let s = format!(
        //     "hand = {:?}, bid = {}, index = {}",
        //     hand,
        //     // acc,
        //     hand.bid(),
        //     index + 1
        // );
        // writeln!(file, "{}", s).unwrap();
        acc + (hand.bid() * (index + 1))
    });
    result
}

pub fn part2(_input: &str) -> usize {
    // let mut hands = parse_hands(input).expect("expected successful parsing");
    // hands.sort_by(|a, b| a.compare(b));

    // let result = hands
    //     .iter()
    //     .enumerate()
    //     .fold(0, |acc, (index, hand)| acc + (hand.bid() * (index + 1)));
    // result
    todo!("Part 2 not ready yet");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    // use rstest::rstest;

    #[allow(unused_imports)]
    use super::*;

    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn hand_type_test() {
        assert_eq!(part1(TEST), 6440);
    }

    #[test]
    fn hand_type_one_line() {
        assert_eq!(part1(TEST.lines().next().unwrap()), 765);
    }
}
