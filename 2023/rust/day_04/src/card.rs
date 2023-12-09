const PIPE: char = '|';

#[derive(thiserror::Error, Debug)]
pub enum CardParseError {
    #[error("Should be only one pipe symbol")]
    Pipe(String),

    #[error("Card must start from the Card N: syntax, found {0}")]
    DoubleColon(String),

    // #[error(transparent)]
    #[error("Card number could not be parsed {0}")]
    ParseInt(#[source] std::num::ParseIntError, String),
}

#[derive(Clone)]
pub struct Card {
    id: u32,
    win: Vec<u32>,
    hand: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        self.win
            .iter()
            .filter(|w| self.hand.as_slice().binary_search(w).is_ok())
            .enumerate()
            .fold(0u32, |mut acc, (i, _)| {
                match i {
                    0 => acc += 1,
                    _ => acc *= 2,
                };
                acc
            })
    }

    pub fn matches_count(&self) -> usize {
        self.win
            .iter()
            .filter(|w| self.hand.as_slice().binary_search(w).is_ok())
            .count()
    }

    fn parse_one(card_line: &str) -> Result<Card, CardParseError> {
        let (id, numbers) = card_line
            .split_once(':')
            .ok_or(CardParseError::DoubleColon(card_line.to_owned()))?;

        let id = id
            .trim_start_matches("Card") // Remove Card word
            .trim_start() // Remove whitespaces before ID
            .parse::<u32>()
            .map_err(|e| CardParseError::ParseInt(e, id.to_owned()))?;

        let (win, hand) = numbers
            .split_once(PIPE)
            .ok_or(CardParseError::Pipe(numbers.to_owned()))?;

        let mut win = Card::to_vec(win)?;
        let mut hand = Card::to_vec(hand)?;
        win.sort();
        hand.sort();

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

    pub fn id(&self) -> u32 {
        self.id
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

pub fn parse_cards(input: &str) -> Result<Vec<Card>, CardParseError> {
    use std::str::FromStr;

    input.lines().map(Card::from_str).collect()
}
