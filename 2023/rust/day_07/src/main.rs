use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
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

#[derive(Debug, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
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

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        Self { cards, bid }
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

fn main() {
    let input = include_str!("input.txt");

    let hands = input
        .lines()
        .map(Hand::from_str)
        // .inspect(|r| {
        //     dbg!(&r);
        // })
        .collect::<Result<Vec<_>, _>>()
        .expect("expected valid input");
    println!("{hands:?}");

    println!("Hello, world!");
}
