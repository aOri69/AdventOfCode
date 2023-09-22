use nom::{
    bytes::complete::tag,
    character::complete::{one_of, space1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use super::{Item, Items, Operation};

pub fn parse_items(input: &str) -> IResult<&str, Items> {
    let mut items_parser = preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), map(nom::character::complete::u32, Item::from)),
    );

    items_parser(input)
}

pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
    // Operation: new = old * 2
    let operation_parser = separated_pair(one_of("+-*/"), space1, nom::character::complete::i32);

    let mut parser = preceded(
        tag("  Operation: new = old "),
        map_res(operation_parser, Operation::try_from),
    );

    parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod items {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn starting_items_empty() {
            const STARTING_ITEMS_EMPTY: &str = "  Starting items: ";
            let expected: Vec<Item> = vec![];
            let (_remaining, result) = parse_items(STARTING_ITEMS_EMPTY).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn starting_items_non_empty() {
            const STARTING_ITEMS: &str = "  Starting items: 54, 65, 75, 74";
            const STARTING_ITEMS_CRLF: &str = "  Starting items: 54, 65, 75, 74\r\n";
            let expected = vec![
                Item::from(54),
                Item::from(65),
                Item::from(75),
                Item::from(74),
            ];
            let (_remaining, result) = parse_items(STARTING_ITEMS).finish().unwrap();
            assert_eq!(expected, result);

            let (_remaining, result) = parse_items(STARTING_ITEMS_CRLF).finish().unwrap();
            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        use crate::monkey::{parser::parse_operation, Operation};

        #[test]
        fn add() {
            const OPERATION_OK: &str = "  Operation: new = old + 2";
            let expected: Operation = Operation::Add(2);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        #[should_panic]
        fn add_fail() {
            const OPERATION_INVALID: &str = "  Operation: new = old / 0";
            let result = parse_operation(OPERATION_INVALID).finish().unwrap();
        }
    }
}
