use std::{collections::HashMap, str::FromStr};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("No such card: {0}")]
    UnsupportedSymbol(char),
    #[error("Not enough cards in hand")]
    NotEnoughCards,
    #[error("No Bid")]
    NoBid,
    #[error("No Cards")]
    NoCards,
    #[error("Faied to parse bid {0}")]
    BidNotNumber(#[source] std::num::ParseIntError),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl TryFrom<char> for Card {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            c => Err(ParseError::UnsupportedSymbol(c)),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Two => write!(f, "2"),
            Card::Three => write!(f, "3"),
            Card::Four => write!(f, "4"),
            Card::Five => write!(f, "5"),
            Card::Six => write!(f, "6"),
            Card::Seven => write!(f, "7"),
            Card::Eight => write!(f, "8"),
            Card::Nine => write!(f, "9"),
            Card::T => write!(f, "T"),
            Card::J => write!(f, "J"),
            Card::Q => write!(f, "Q"),
            Card::K => write!(f, "K"),
            Card::A => write!(f, "A"),
        }
    }
}

/// Every hand is exactly one type. From strongest to weakest, they are:
/// Five of a kind, where all five cards have the same label: AAAAA
/// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
/// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
/// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
/// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
/// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
/// High card, where all cards' labels are distinct: 23456
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

#[derive(PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut cards_to_count = HashMap::with_capacity(5);
        self.cards.iter().for_each(|c| {
            *cards_to_count.entry(c).or_insert(0) += 1;
        });

        dbg!(&cards_to_count);

        let mut cards_to_count_sorted = cards_to_count
            .iter()
            .map(|(k, v)| (*k, v))
            .collect::<Vec<_>>();
        cards_to_count_sorted.sort_by(|a, b| b.1.cmp(a.1));

        dbg!(&cards_to_count_sorted);

        let mut count_it_rev = cards_to_count_sorted.iter();
        let mut hand_type = HandType::HighCard(**cards_to_count.iter().next().unwrap().0);

        if let Some(&(&card, &count)) = count_it_rev.next() {
            match count {
                5 => hand_type = HandType::FiveOfAKind(card),
                4 => hand_type = HandType::FourOfAKind(card),
                3 => {
                    hand_type = match count_it_rev.next().unwrap() {
                        &(next_card, 2) => HandType::FullHouse(card, *next_card),
                        _ => HandType::ThreeOfAKind(card),
                    }
                }
                2 => {
                    hand_type = match count_it_rev.next().unwrap() {
                        &(next_card, 2) => HandType::TwoPair(card, *next_card),
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
        for card in self.cards {
            write!(f, "{card:?}")?
        }
        write!(f, " - {}", self.bid)
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

    #[allow(unused_imports)]
    use super::*;

    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_test() {
        use Card::*;

        let hands = parse_hands(TEST);
        let expected: Result<Vec<Hand>, ParseError> = Ok(vec![
            Hand {
                cards: [Three, Two, T, Three, K],
                bid: 765,
            },
            Hand {
                cards: [T, Five, Five, J, Five],
                bid: 684,
            },
            Hand {
                cards: [K, K, Six, Seven, Seven],
                bid: 28,
            },
            Hand {
                cards: [K, T, J, J, T],
                bid: 220,
            },
            Hand {
                cards: [Q, Q, Q, J, A],
                bid: 483,
            },
        ]);

        assert_eq!(hands, expected);
    }
    #[test]
    fn hand_type_test() {
        use Card::*;
        use HandType::*;

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
}
