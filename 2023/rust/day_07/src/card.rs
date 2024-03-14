use crate::{hand_type::Valuable, HandError};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(super) struct Card(pub(super) char);

impl Valuable for Card {
    /// Instead of using enum:
    /// enum CardValue {
    ///    Two = 2,
    ///    Three = 3,
    ///    Four = 4,
    ///    Five = 5,
    ///    Six = 6,
    ///    Seven = 7,
    ///    Eight = 8,
    ///    Nine = 9,
    ///    T = 10,
    ///    J = 11,
    ///    Q = 12,
    ///    K = 13,
    ///    cA = 14,
    ///}
    /// simple value getter was implemented
    fn value(&self) -> u8 {
        match **self {
            '2'..='9' => (**self).to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl TryFrom<char> for Card {
    type Error = HandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'K' | 'Q' | 'J' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                Ok(Self(value))
            }
            _ => Err(HandError::UnsupportedSymbol(value)),
        }
    }
}

impl std::ops::Deref for Card {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case('2', Ok(Card('2')))]
    #[case('3', Ok(Card('3')))]
    #[case('4', Ok(Card('4')))]
    #[case('5', Ok(Card('5')))]
    #[case('6', Ok(Card('6')))]
    #[case('7', Ok(Card('7')))]
    #[case('8', Ok(Card('8')))]
    #[case('9', Ok(Card('9')))]
    #[case('A', Ok(Card('A')))]
    #[case('K', Ok(Card('K')))]
    #[case('Q', Ok(Card('Q')))]
    #[case('J', Ok(Card('J')))]
    #[case('T', Ok(Card('T')))]
    fn parse_card(#[case] input: char, #[case] expected: Result<Card, HandError>) {
        assert_eq!(Card::try_from(input), expected);
    }

    #[rstest]
    fn unsupported_cards() {
        // lowercase plus uppercase minus card symbols
        let chars_it =
            ('a'..='z').chain(('A'..='Z').filter(|c| !matches!(c, 'A' | 'K' | 'Q' | 'J' | 'T')));
        for c in chars_it {
            dbg!(c);
            assert_eq!(Card::try_from(c), Err(HandError::UnsupportedSymbol(c)));
        }
    }
}
