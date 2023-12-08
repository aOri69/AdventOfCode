#![allow(unused)]

const PIPE: char = '|';

#[derive(thiserror::Error, Debug)]
enum CardParseError {
    #[error("Should be only one pipe symbol")]
    Pipe(String),

    #[error("Card must start from the Card N: syntax, found {0}")]
    DoubleColon(String),

    // #[error(transparent)]
    #[error("Card number could not be parsed {0}")]
    ParseInt(#[source] std::num::ParseIntError, String),
}

struct Card {
    id: u32,
    win: Vec<u32>,
    hand: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        todo!("Main logic of counting the points based on win and hand numbers");
    }

    fn parse_one(card_line: &str) -> Result<Card, CardParseError> {
        let (id, numbers) = card_line
            .split_once(':')
            .ok_or(CardParseError::DoubleColon(card_line.to_owned()))?;

        let id = id
            .trim_start_matches("Card ")
            .parse::<u32>()
            .map_err(|e| CardParseError::ParseInt(e, id.to_owned()))?;

        let (win, hand) = numbers
            .split_once(PIPE)
            .ok_or(CardParseError::Pipe(numbers.to_owned()))?;

        let win = Card::to_vec(win)?;
        let hand = Card::to_vec(hand)?;

        Ok(Card { id, win, hand })
    }

    fn to_vec(input: &str) -> Result<Vec<u32>, CardParseError> {
        input
            .split_ascii_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|e| CardParseError::ParseInt(e, n.to_owned()))
            })
            .collect()
    }
}

impl std::str::FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Card::parse_one(s)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {:?} | {:?}", self.id, self.win, self.hand)
    }
}

fn parse_cards(input: &str) -> Result<Vec<Card>, CardParseError> {
    use std::str::FromStr;

    input.lines().map(Card::from_str).collect()
}

fn part1(input: &str) -> u32 {
    let cards = parse_cards(input).expect("Can't parse all cards");
    let result = cards.iter().map(|c| c.points()).sum::<u32>();
    result
}

fn part2(input: &str) -> u32 {
    todo!("part 2 implementation");
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part1(input));
    println!("Part 2 answer: {}", part2(input));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 13);
    }

    #[test]
    fn test_part2() {
        todo!("part2 test");
    }
}
