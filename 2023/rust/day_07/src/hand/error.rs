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
