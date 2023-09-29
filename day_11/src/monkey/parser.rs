// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_till, take_until},
//     character::complete::{alpha1, digit1, line_ending, one_of, space1},
//     combinator::{all_consuming, map, map_res, opt},
//     multi::{many0, separated_list0},
//     sequence::{delimited, preceded, separated_pair, terminated, tuple},
//     IResult,
// };

// use super::{Item, Items, Monkey, Monkeys, Operation, Test};

// pub fn parse_items(input: &str) -> IResult<&str, Items> {
//     let mut items_parser = preceded(
//         tag("  Starting items: "),
//         separated_list0(tag(", "), map(nom::character::complete::u32, Item::from)),
//     );

//     items_parser(input)
// }

// pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
//     // Operation: new = old * 2
//     let (remaining, _prefix) = take_until("old ")(input)?;
//     let (remaining, (op, rhs)) = separated_pair(one_of("+-*/"), space1, alpha1)(remaining)?;

//     // let mut parser = preceded(
//     //     tag("  Operation: new = old "),
//     //     map_res(operation_parser, Operation::try_from),
//     // );

//     // parser(input)
//     todo!()
// }

// pub fn parse_test(input: &str) -> IResult<&str, Test> {
//     let divisible_by = terminated(
//         preceded(tag("  Test: divisible by "), nom::character::complete::i32),
//         line_ending,
//     );
//     let if_true_throw_to = terminated(
//         preceded(
//             tag("    If true: throw to monkey "),
//             nom::character::complete::i32,
//         ),
//         line_ending,
//     );
//     let if_false_throw_to = preceded(
//         tag("    If false: throw to monkey "),
//         nom::character::complete::i32,
//     );

//     map_res(
//         tuple((divisible_by, if_true_throw_to, if_false_throw_to)),
//         |(divisible_by, if_true_throw_to, if_false_throw_to)| {
//             let operation = Operation::try_from(('/', divisible_by))?;
//             Ok::<Test, &str>(Test::new(operation, if_true_throw_to, if_false_throw_to))
//         },
//     )(input)
// }

// pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
//     let (remaining, _) = take_until("Monkey")(input)?;
//     let (remaining, _id) = delimited(
//         tag("Monkey "),
//         nom::character::complete::u32,
//         preceded(tag(":"), line_ending),
//     )(remaining)?;
//     let (remaining, items) = terminated(parse_items, line_ending)(remaining)?;
//     let (remaining, operation) = terminated(parse_operation, line_ending)(remaining)?;
//     let (remaining, test) = terminated(parse_test, opt(line_ending))(remaining)?;
//     let monkey = Monkey::new(items, operation, test);
//     Ok((remaining, monkey))
// }

// pub fn parse_monkeys(input: &str) -> IResult<&str, Monkeys> {
//     all_consuming(many0(parse_monkey))(input)
// }
