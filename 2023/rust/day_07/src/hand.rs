use std::{cmp, str::FromStr};

use crate::{
    card::{jocker, Card},
    error::ParseError,
    hand_type::HandType,
};

#[derive(PartialEq)]
pub struct Hand<CardType> {
    cards: [CardType; 5],
    bid: usize,
}

impl<T> Hand<T>
where
    T: Copy + std::cmp::Eq + std::cmp::Ord,
{
    pub fn hand_type(&self) -> HandType<T> {
        let mut cards_to_count: Vec<(T, i32)> = Vec::with_capacity(5);
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
}

impl<T> Hand<T>
where
    T: Copy + std::cmp::Eq + std::cmp::Ord,
{
    pub fn new(cards: [T; 5], bid: usize) -> Self {
        Self { cards, bid }
    }

    pub fn compare(&self, other: &Hand<T>) -> cmp::Ordering {
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

    fn compare_high_card(&self, other: &Hand<T>) -> cmp::Ordering {
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
}

impl<T> std::str::FromStr for Hand<T>
where
    T: std::convert::TryFrom<char> + Copy + std::cmp::Eq + std::cmp::Ord,
    crate::error::ParseError: std::convert::From<<T as std::convert::TryFrom<char>>::Error>,
{
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut hand_it = line.split_ascii_whitespace().take(2);
        let mut cards_it = hand_it
            .next()
            .ok_or(ParseError::NoCards)?
            .chars()
            .take(5)
            .map(T::try_from);
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

impl<T: std::fmt::Debug + Copy> std::fmt::Debug for Hand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for card in self.cards {
            write!(f, "{card:?}")?
        }
        write!(f, "\"")
        // write!(f, " - {}", self.bid)
    }
}

pub fn parse_hands(input: &str) -> Result<Vec<Hand<Card>>, ParseError> {
    input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse_hands_with_jockers(input: &str) -> Result<Vec<Hand<jocker::Card>>, ParseError> {
    input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {

    use crate::{card::Card::*, error::ParseError, hand::HandType::*};
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
        let expected: Result<Vec<Hand<Card>>, ParseError> = Ok(vec![
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
        let lhs: Hand<Card> = Hand::new(
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
