mod card;
mod hand_type;
use std::str::FromStr;

use card::Card;
use hand_type::HandType;

#[derive(thiserror::Error, Debug, PartialEq)]
enum HandError {
    #[error("No such card: {0}")]
    UnsupportedSymbol(char),
    #[error("Not enough cards in hand")]
    NotEnoughCards,
    #[error("Faied to parse bid {0}")]
    BidNotNumber(#[source] std::num::ParseIntError),
    #[error("Failed to split at space: {0}")]
    SplitError(String),
}

const NO_JOCKER: bool = false;
const WITH_JOCKER: bool = true;

struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn mask_jocker(&self) -> [Card; 5] {
        if !self.cards.contains(&'J'.try_into().unwrap()) {
            return self.cards;
        }

        let replace_with = match self.hand_type(NO_JOCKER) {
            HandType::HighCard(c) => c,
            HandType::OnePair(c) => c,
            HandType::TwoPair(c1, c2) => match c1.cmp(&c2) {
                std::cmp::Ordering::Less => c2,
                std::cmp::Ordering::Equal => c1,
                std::cmp::Ordering::Greater => c1,
            },
            HandType::ThreeOfAKind(c) => c,
            HandType::FullHouse(c1, _) => c1,
            HandType::FourOfAKind(c) => c,
            HandType::FiveOfAKind(_) => return self.cards,
        };

        let mut result = self.cards;
        result
            .iter_mut()
            .filter(|c| *c == &Card('J'))
            .for_each(|c| *c = replace_with);
        result
    }
}

impl Hand {
    fn hand_type(&self, use_jocker: bool) -> HandType<Card> {
        let mut cards;
        cards = match use_jocker {
            true => self.mask_jocker(),
            false => self.cards,
        };
        cards.sort();

        // CHARS count in hand
        let occurences = [
            cards.iter().filter(|&&c| c == cards[0]).count(),
            cards.iter().filter(|&&c| c == cards[1]).count(),
            cards.iter().filter(|&&c| c == cards[2]).count(),
            cards.iter().filter(|&&c| c == cards[3]).count(),
            cards.iter().filter(|&&c| c == cards[4]).count(),
        ];
        // in descending order
        let mut sorted_occurences = occurences;
        sorted_occurences.sort_by(|a, b| b.cmp(a));

        match sorted_occurences {
            [5, 5, 5, 5, 5] => HandType::FiveOfAKind(cards[0]),
            [4, 4, 4, 4, 1] => {
                HandType::FourOfAKind(cards[occurences.iter().position(|p| *p == 4).unwrap()])
            }
            [3, 3, 3, 2, 2] => HandType::FullHouse(
                cards[occurences.iter().position(|p| *p == 3).unwrap()],
                cards[occurences.iter().position(|p| *p == 2).unwrap()],
            ),
            [3, 3, 3, 1, 1] => {
                HandType::ThreeOfAKind(cards[occurences.iter().position(|p| *p == 3).unwrap()])
            }
            [2, 2, 2, 2, 1] => {
                use std::collections::HashMap;
                let mut cards_map = HashMap::new();
                cards
                    .iter()
                    .for_each(|c| *cards_map.entry(c).or_insert(0) += 1);
                cards_map.retain(|_, v| *v == 2);
                let mut pairs_it = cards_map.into_keys().take(2);
                let (&first, &second) = (
                    pairs_it.next().expect("expected first pair"),
                    pairs_it.next().expect("expected second pair"),
                );
                HandType::TwoPair(first, second)
            }
            [2, 2, 1, 1, 1] => {
                HandType::OnePair(cards[occurences.iter().position(|p| *p == 2).unwrap()])
            }
            // High card should always be counted from the original hand
            // Furthermore J should have value 2 when USE_JOCKER and 12 when NO_JOCKER
            [1, 1, 1, 1, 1] => HandType::HighCard(
                *self
                    .cards
                    .iter()
                    .map(|c| match use_jocker {
                        true => match c {
                            Card(c) if *c == 'J' => &Card('2'),
                            c => c,
                        },
                        false => c,
                    })
                    .max()
                    .unwrap(),
            ),
            _ => unreachable!(),
        }
    }

    fn bid(&self) -> usize {
        self.bid
    }

    fn compare_no_jocker(&self, other: &Hand) -> std::cmp::Ordering {
        match self.hand_type(NO_JOCKER).cmp(&other.hand_type(NO_JOCKER)) {
            std::cmp::Ordering::Equal => self.compare_high_card(other),
            ord => ord,
        }
    }

    fn compare_with_jocker(&self, other: &Hand) -> std::cmp::Ordering {
        // For debug purposes
        // let result = match self.hand_type().cmp(&other.hand_type()) {
        //     std::cmp::Ordering::Equal => self.compare_high_card(other),
        //     ord => ord,
        // };
        // dbg!(self, other, &result);
        // result
        match self
            .hand_type(WITH_JOCKER)
            .cmp(&other.hand_type(WITH_JOCKER))
        {
            std::cmp::Ordering::Equal => self.compare_high_card(other),
            ord => ord,
        }
    }

    fn compare_high_card(&self, other: &Hand) -> std::cmp::Ordering {
        let mut other_it = other.cards.iter();
        for card in self.cards.iter() {
            let other_card = other_it
                .next()
                .expect("expected same amount of cards in hand");

            if card == other_card {
                continue;
            }

            return card.cmp(other_card);
        }
        std::cmp::Ordering::Equal
    }
}

impl std::str::FromStr for Hand {
    type Err = HandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_it, bid) = s
            .split_once(' ')
            .ok_or(HandError::SplitError(s.to_owned()))?;
        let mut hand_it = hand_it.chars();
        Ok(Self {
            cards: [
                hand_it
                    .next()
                    .ok_or(HandError::NotEnoughCards)
                    .and_then(Card::try_from)?,
                hand_it
                    .next()
                    .ok_or(HandError::NotEnoughCards)
                    .and_then(Card::try_from)?,
                hand_it
                    .next()
                    .ok_or(HandError::NotEnoughCards)
                    .and_then(Card::try_from)?,
                hand_it
                    .next()
                    .ok_or(HandError::NotEnoughCards)
                    .and_then(Card::try_from)?,
                hand_it
                    .next()
                    .ok_or(HandError::NotEnoughCards)
                    .and_then(Card::try_from)?,
            ],
            bid: bid.parse().map_err(HandError::BidNotNumber)?,
        })
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for card in self.cards {
            write!(f, "{}", *card)?
        }
        write!(f, "\"")?;
        write!(f, " - {}", self.bid)
    }
}

fn parse_hands(input: &str) -> Result<Vec<Hand>, HandError> {
    input.lines().map(Hand::from_str).collect()
}

pub fn part1(input: &str) -> usize {
    let mut hands = parse_hands(input).expect("expected succesfully parse input");
    // hands.sort_by_key(|h| h.hand_type());
    hands.sort_by(Hand::compare_no_jocker);
    // dbg!(&hands);
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (h.bid() * (i + 1)))
}

pub fn part2(input: &str) -> usize {
    let mut hands = parse_hands(input).expect("expected succesfully parse input");
    hands.sort_by(Hand::compare_with_jocker);
    // dbg!(&hands);
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (h.bid() * (i + 1)))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::cmp::Ordering;
    // use HandType::*;

    type HT = HandType<Card>;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_small() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn part2_small() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }

    #[test]
    fn one_line_only() {
        assert_eq!(part1(TEST_INPUT.lines().next().unwrap()), 765);
    }

    #[rstest]
    #[case::one_pair("KK234", HT::OnePair('K'.try_into().unwrap()))]
    #[case::one_pair("K234K", HT::OnePair('K'.try_into().unwrap()))]
    #[case::one_pair("K2K43", HT::OnePair('K'.try_into().unwrap()))]
    #[case::one_pair("23KK4", HT::OnePair('K'.try_into().unwrap()))]
    #[case::one_pair("234KK", HT::OnePair('K'.try_into().unwrap()))]
    #[case::two_pairs("KK677", HT::TwoPair('K'.try_into().unwrap(),'7'.try_into().unwrap()))]
    #[case::two_pairs("776KK", HT::TwoPair('K'.try_into().unwrap(),'7'.try_into().unwrap()))]
    #[case::two_pairs("767KK", HT::TwoPair('K'.try_into().unwrap(),'7'.try_into().unwrap()))]
    #[case::two_pairs("7K76K", HT::TwoPair('K'.try_into().unwrap(),'7'.try_into().unwrap()))]
    #[case::two_pairs("A2A32", HT::TwoPair('A'.try_into().unwrap(),'2'.try_into().unwrap()))]
    #[case::two_pairs("2A23A", HT::TwoPair('A'.try_into().unwrap(),'2'.try_into().unwrap()))]
    #[case::three_of_a_kind("T55J5", HT::ThreeOfAKind('5'.try_into().unwrap()))]
    #[case::three_of_a_kind("5T5J5", HT::ThreeOfAKind('5'.try_into().unwrap()))]
    #[case::three_of_a_kind("55TJ5", HT::ThreeOfAKind('5'.try_into().unwrap()))]
    #[case::three_of_a_kind("J5T55", HT::ThreeOfAKind('5'.try_into().unwrap()))]
    #[case::four_of_a_kind("AA2AA", HT::FourOfAKind('A'.try_into().unwrap()))]
    #[case::four_of_a_kind("2AAAA", HT::FourOfAKind('A'.try_into().unwrap()))]
    #[case::four_of_a_kind("AAAA2", HT::FourOfAKind('A'.try_into().unwrap()))]
    #[case::five_of_a_kind("AAAAA", HT::FiveOfAKind('A'.try_into().unwrap()))]
    #[case::five_of_a_kind("JJJJJ", HT::FiveOfAKind('J'.try_into().unwrap()))]
    fn hand_type_no_j(#[case] input: &str, #[case] expected: HandType<Card>) {
        let input = input.to_owned() + " 0";
        let hand = Hand::from_str(&input).unwrap();
        let hand_type = hand.hand_type(NO_JOCKER);
        assert_eq!(hand_type, expected);
    }

    #[rstest]
    #[case::four_of_a_kind("QJJQ2", HT::FourOfAKind('Q'.try_into().unwrap()))]
    #[case::four_of_a_kind("JKKK2", HT::FourOfAKind('K'.try_into().unwrap()))]
    fn hand_type_j(#[case] input: &str, #[case] expected: HandType<Card>) {
        let input = input.to_owned() + " 0";
        let hand = Hand::from_str(&input).unwrap();
        let hand_type = hand.hand_type(WITH_JOCKER);
        assert_eq!(hand_type, expected);
    }

    #[rstest]
    #[case("QJJQ2", "JKKK2", NO_JOCKER, Ordering::Less)]
    #[case("QJJQ2", "JKKK2", WITH_JOCKER, Ordering::Greater)]
    fn compare(
        #[case] lhs: &str,
        #[case] rhs: &str,
        #[case] use_jocker: bool,
        #[case] expected: std::cmp::Ordering,
    ) {
        let lhs = Hand::from_str(&(lhs.to_owned() + " 0")).unwrap();
        let rhs = Hand::from_str(&(rhs.to_owned() + " 0")).unwrap();
        let comparison_result = match use_jocker {
            true => lhs.compare_with_jocker(&rhs),
            false => lhs.compare_no_jocker(&rhs),
        };
        assert_eq!(comparison_result, expected);
    }
}
