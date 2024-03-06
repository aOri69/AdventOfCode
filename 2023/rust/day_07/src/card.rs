use super::error::ParseError;

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

pub mod jocker {
    use crate::error::ParseError;

    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        J = 2,
        Two = 3,
        Three = 4,
        Four = 5,
        Five = 6,
        Six = 7,
        Seven = 8,
        Eight = 9,
        Nine = 10,
        T = 11,
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

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use Card::*;

    #[rstest]
    #[case::two('2', Ok(Two))]
    #[case::three('3', Ok(Three))]
    #[case::four('4', Ok(Four))]
    #[case::five('5', Ok(Five))]
    #[case::six('6', Ok(Six))]
    #[case::seven('7', Ok(Seven))]
    #[case::eight('8', Ok(Eight))]
    #[case::nine('9', Ok(Nine))]
    #[case::t('T', Ok(T))]
    #[case::j('J', Ok(J))]
    #[case::q('Q', Ok(Q))]
    #[case::k('K', Ok(K))]
    #[case::a('A', Ok(A))]
    #[case::unsupported_symbol_z('Z', Err(ParseError::UnsupportedSymbol('Z')))]
    #[case::unsupported_symbol_y('y', Err(ParseError::UnsupportedSymbol('y')))]
    fn parse_card(#[case] input: char, #[case] expected: Result<Card, ParseError>) {
        assert_eq!(expected, Card::try_from(input));
    }
}
