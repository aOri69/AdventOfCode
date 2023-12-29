mod hand;

use hand::parse_hands;

/// Now, you can determine the total winnings of this set of hands
/// by adding up the result of multiplying each hand's bid with its rank
pub fn part1(input: &str) -> usize {
    let mut hands = parse_hands(input).expect("expected successful parsing");

    hands.sort_by(|a, b| {
        dbg!(a, b, a.compare(b));
        a.compare(b)
    });

    dbg!(&hands);

    let result = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        dbg!(acc, hand, index + 1, hand.bid() * (index + 1));
        dbg!(acc + (hand.bid() * (index + 1)))
    });
    result
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
