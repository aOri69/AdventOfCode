use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use super::{Item, Items, Operation, Test};

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

pub fn parse_test(input: &str) -> IResult<&str, Test> {
    let divisible_by = terminated(
        preceded(tag("  Test: divisible by "), nom::character::complete::i32),
        line_ending,
    );
    let if_true_throw_to = terminated(
        preceded(
            tag("    If true: throw to monkey "),
            nom::character::complete::i32,
        ),
        line_ending,
    );
    let if_false_throw_to = terminated(
        preceded(
            tag("    If false: throw to monkey "),
            nom::character::complete::i32,
        ),
        line_ending,
    );

    map_res(
        tuple((divisible_by, if_true_throw_to, if_false_throw_to)),
        |(divisible_by, if_true_throw_to, if_false_throw_to)| {
            let operation = Operation::try_from(('/', divisible_by))?;
            // Shit here. better to implement in another way
            Test::try_from((operation, if_true_throw_to, if_false_throw_to))
        },
    )(input)
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
    mod operation_parser {
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
        fn multiply() {
            const OPERATION_OK: &str = "  Operation: new = old * 3";
            let expected: Operation = Operation::Multiply(3);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn divide_ok() {
            const OPERATION_OK: &str = "  Operation: new = old / 2";
            let expected: Operation = Operation::Divide(2);
            let (_remaining, result) = parse_operation(OPERATION_OK).finish().unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        #[should_panic]
        fn divide_fail() {
            const OPERATION_OK: &str = "  Operation: new = old / 0";
            let result = parse_operation(OPERATION_OK).finish().unwrap();
        }

        #[test]
        #[should_panic]
        fn add_fail() {
            const OPERATION_INVALID: &str = "  Operation: new = old / 0";
            let result = parse_operation(OPERATION_INVALID).finish().unwrap();
        }
    }

    #[cfg(test)]
    mod test_parser {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn simple() {
            const TEST_OK: &str = "  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 2
            ";
            let expected: Test = Test {
                operation: Operation::Divide(2),
                if_true_throw_to: 1,
                if_false_throw_to: 2,
            };
            let (_remaining, result) = parse_test(TEST_OK).finish().unwrap();
            assert_eq!(expected, result);
        }
    }
}
