pub mod card;
pub mod error;

use std::{cmp, mem, str::FromStr};

use self::{card::Card, error::ParseError};

/// Every hand is exactly one type. From strongest to weakest, they are:
/// Five of a kind, where all five cards have the same label: AAAAA
/// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
/// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
/// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
/// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
/// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
/// High card, where all cards' labels are distinct: 23456
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl HandType {
    #[allow(dead_code)]
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn discriminant_safe(&self) -> u8 {
        match self {
            HandType::HighCard(_) => 0,
            HandType::OnePair(_) => 1,
            HandType::TwoPair(_, _) => 2,
            HandType::ThreeOfAKind(_) => 3,
            HandType::FullHouse(_, _) => 4,
            HandType::FourOfAKind(_) => 5,
            HandType::FiveOfAKind(_) => 6,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self).eq(&mem::discriminant(other))
    }
}

impl Eq for HandType {}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.discriminant_safe().cmp(&other.discriminant_safe())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighCard(arg0) => write!(f, "HighCard({arg0:?})"),
            Self::OnePair(arg0) => write!(f, "OnePair({arg0:?})"),
            Self::TwoPair(arg0, arg1) => write!(f, "TwoPair({arg0:?},{arg1:?})"),
            Self::ThreeOfAKind(arg0) => write!(f, "ThreeOfAKind({arg0:?})"),
            Self::FullHouse(arg0, arg1) => write!(f, "FullHouse({arg0:?},{arg1:?})"),
            Self::FourOfAKind(arg0) => write!(f, "FourOfAKind({arg0:?})"),
            Self::FiveOfAKind(arg0) => write!(f, "FiveOfAKind({arg0:?})"),
        }
    }
}

#[derive(PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: usize) -> Self {
        Self { cards, bid }
    }

    pub fn hand_type(&self) -> HandType {
        let mut cards_to_count: Vec<(Card, i32)> = Vec::with_capacity(5);
        self.cards.iter().for_each(|c| {
            let entry = cards_to_count.iter_mut().find(|&&mut (card, _)| card.eq(c));
            match entry {
                Some(entry) => entry.1 += 1,
                None => cards_to_count.push((*c, 1)),
            }
        });

        let mut cards_to_count_sorted = cards_to_count
            .iter()
            .map(|(k, v)| (*k, v))
            .collect::<Vec<_>>();
        cards_to_count_sorted.sort_by(|a, b| b.1.cmp(a.1));

        let mut count_it_rev = cards_to_count_sorted.iter();
        let mut hand_type = HandType::HighCard(cards_to_count.first().unwrap().0);

        if let Some(&(card, &count)) = count_it_rev.next() {
            match count {
                5 => hand_type = HandType::FiveOfAKind(card),
                4 => hand_type = HandType::FourOfAKind(card),
                3 => {
                    hand_type = match count_it_rev.next().unwrap() {
                        &(next_card, 2) => HandType::FullHouse(card, next_card),
                        _ => HandType::ThreeOfAKind(card),
                    }
                }
                2 => {
                    hand_type = match count_it_rev.next().unwrap() {
                        &(next_card, 2) => HandType::TwoPair(card, next_card),
                        _ => HandType::OnePair(card),
                    }
                }
                1 => hand_type = HandType::HighCard(card),
                _ => unreachable!("amount of cards in HashMap should be between 1-5"),
            };
        }

        hand_type
    }

    fn compare_high_card(&self, other: &Hand) -> cmp::Ordering {
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
        cmp::Ordering::Equal
    }

    pub fn compare(&self, other: &Hand) -> cmp::Ordering {
        // let order = match self.hand_type().cmp(&other.hand_type()) {
        //     std::cmp::Ordering::Equal => self.compare_high_card(other),
        //     ord => ord,
        // };
        // // dbg!(self, self.hand_type(), other, other.hand_type(), &order);
        // order
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => self.compare_high_card(other),
            ord => ord,
        }
    }

    pub fn bid(&self) -> usize {
        self.bid
    }
}

impl std::str::FromStr for Hand {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut hand_it = line.split_ascii_whitespace().take(2);
        let mut cards_it = hand_it
            .next()
            .ok_or(ParseError::NoCards)?
            .chars()
            .take(5)
            .map(Card::try_from);
        let bid = hand_it.next().ok_or(ParseError::NoBid)?;
        Ok(Self::new(
            [
                cards_it.next().ok_or(ParseError::NotEnoughCards)??,
                cards_it.next().ok_or(ParseError::NotEnoughCards)??,
                cards_it.next().ok_or(ParseError::NotEnoughCards)??,
                cards_it.next().ok_or(ParseError::NotEnoughCards)??,
                cards_it.next().ok_or(ParseError::NotEnoughCards)??,
            ],
            str::parse::<usize>(bid).map_err(ParseError::BidNotNumber)?,
        ))
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for card in self.cards {
            write!(f, "{card:?}")?
        }
        write!(f, "\"")
        // write!(f, " - {}", self.bid)
    }
}

pub fn parse_hands(input: &str) -> Result<Vec<Hand>, ParseError> {
    input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {

    use crate::hand::{card::Card::*, error::ParseError, parse_hands, Hand, HandType::*};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[allow(unused_imports)]
    use super::*;

    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_test() {
        let hands = parse_hands(TEST);
        let expected: Result<Vec<Hand>, ParseError> = Ok(vec![
            Hand::new([Three, Two, T, Three, K], 765),
            Hand::new([T, Five, Five, J, Five], 684),
            Hand::new([K, K, Six, Seven, Seven], 28),
            Hand::new([K, T, J, J, T], 220),
            Hand::new([Q, Q, Q, J, A], 483),
        ]);

        assert_eq!(hands, expected);
    }
    #[test]
    fn hand_type_test() {
        let hands = parse_hands(TEST)
            .unwrap()
            .iter()
            .map(|h| h.hand_type())
            .collect::<Vec<_>>();

        let expected = vec![
            OnePair(Three),
            ThreeOfAKind(Five),
            TwoPair(K, Seven),
            TwoPair(T, J),
            ThreeOfAKind(Q),
        ];
        assert_eq!(hands, expected);
    }

    #[rstest]
    #[case(("32T3K","32T3K"), cmp::Ordering::Equal)]
    #[case(("KK677","KTJJT"), cmp::Ordering::Greater)]
    #[case(("T55J5","QQQJA"), cmp::Ordering::Less)]
    #[case(("2222A","2222J"), cmp::Ordering::Greater)]
    #[case(("25QJQ","279Q2"), cmp::Ordering::Less)]
    fn test_compare(#[case] input: (&str, &str), #[case] expected: cmp::Ordering) {
        let mut it = input.0.chars().map(|c| Card::try_from(c).unwrap());
        let lhs = Hand::new(
            [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ],
            0,
        );
        let mut it = input.1.chars().map(|c| Card::try_from(c).unwrap());
        let rhs = Hand::new(
            [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ],
            0,
        );

        assert_eq!(expected, lhs.compare(&rhs));
    }
}
