use std::cmp;

pub trait Valuable {
    fn value(&self) -> u8;
}

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
pub enum HandType<T>
where
    T: Valuable,
{
    HighCard(T) = 0,
    OnePair(T) = 50,
    TwoPair(T, T) = 100,
    ThreeOfAKind(T) = 150,
    FullHouse(T, T),
    FourOfAKind(T),
    FiveOfAKind(T) = 241,
}

impl<T> HandType<T>
where
    T: Valuable,
{
    #[allow(dead_code)]
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// A2345 -> A = 14
    /// AA345 -> AA = 1000 + 14 = 1014
    /// AA3KK -> AAKK = 2000 + 14 + 13 = 2027
    /// etc...
    #[allow(clippy::identity_op)]
    fn discriminant_safe(&self) -> u16 {
        match self {
            // HandType::HighCard(c) => (0 + c.value()) as u16,
            // HandType::OnePair(c) => 1000u16 + c.value() as u16,
            // HandType::TwoPair(c1, c2) => 2000u16 + c1.value() as u16 + c2.value() as u16,
            // HandType::ThreeOfAKind(c) => 3000u16 + c.value() as u16,
            // HandType::FullHouse(c1, c2) => 4000u16 + c1.value() as u16 + c2.value() as u16,
            // HandType::FourOfAKind(c) => 5000u16 + c.value() as u16,
            // HandType::FiveOfAKind(c) => 6000u16 + c.value() as u16,
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

impl<T> PartialEq for HandType<T>
where
    T: Valuable,
{
    fn eq(&self, other: &Self) -> bool {
        // Not working correctly.
        // in case of different cards inside TwoPair(T,U).cmp(TwoPair(Z,K))
        // will return true
        // std::mem::discriminant(self).eq(&std::mem::discriminant(other))
        self.discriminant_safe().eq(&other.discriminant_safe())
    }
}

impl<T> Eq for HandType<T> where T: Valuable {}

impl<T> Ord for HandType<T>
where
    T: Valuable,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.discriminant_safe().cmp(&other.discriminant_safe())
    }
}

impl<T> PartialOrd for HandType<T>
where
    T: Valuable,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::fmt::Debug for HandType<T>
where
    T: std::fmt::Debug + Valuable,
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
