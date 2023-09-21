use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, newline},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, preceded, terminated},
    Finish, IResult,
};

use crate::monkey::{Item, Items, Monkey};

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    todo!()
}

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    todo!()
}

pub fn parse_items(input: &str) -> IResult<&str, Items> {
    let mut items_parser = terminated(
        preceded(
            tag("  Starting items: "),
            separated_list0(tag(", "), map(nom::character::complete::u32, Item::from)),
        ),
        line_ending,
    );

    items_parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn starting_items_empty() {
        const STARTING_ITEMS_EMPTY: &str = "  Starting items: \r\n";
        let expected: Vec<Item> = vec![];
        let (_remaining, result) = parse_items(STARTING_ITEMS_EMPTY).finish().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn starting_items_non_empty() {
        const STARTING_ITEMS: &str = "  Starting items: 54, 65, 75, 74\r\n";
        let expected = vec![
            Item::from(54),
            Item::from(65),
            Item::from(75),
            Item::from(74),
        ];
        let (_remaining, result) = parse_items(STARTING_ITEMS).finish().unwrap();
        assert_eq!(expected, result);
    }
}
