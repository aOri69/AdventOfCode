pub mod directory;
mod parser;

use nom::{combinator::all_consuming, Finish};
pub use parser::{parse_line, Command, Entry, Line};

/// High level function to turn `&str` input
/// into the Iterator of `parser::Line` items
/// which could be either `Command` or `Entry`
pub fn get_parsed_lines(input: &str) -> impl Iterator<Item = Line> + '_ {
    input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
}
