use std::{cmp, mem};

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
pub enum HandType<T> {
    HighCard(T),
    OnePair(T),
    TwoPair(T, T),
    ThreeOfAKind(T),
    FullHouse(T, T),
    FourOfAKind(T),
    FiveOfAKind(T),
}

impl<T> HandType<T> {
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

impl<T> PartialEq for HandType<T> {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self).eq(&mem::discriminant(other))
    }
}

impl<T> Eq for HandType<T> {}

impl<T> Ord for HandType<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.discriminant_safe().cmp(&other.discriminant_safe())
    }
}

impl<T> PartialOrd for HandType<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::fmt::Debug for HandType<T>
where
    T: std::fmt::Debug,
{
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
