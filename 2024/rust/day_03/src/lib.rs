use nom::bytes::complete::{take_until, take_while};
use nom::character::complete::char;
use nom::combinator::{map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, tuple};
use nom::Finish;
use nom::{bytes::complete::tag, IResult};

#[allow(unused)]
#[derive(Debug)]
struct Mul(u32, u32);

impl Mul {
    fn mul(&self) -> u32 {
        self.0 * self.1
    }
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (remaining, _) = take_until("mul(")(input)?;
    println!("{:?}", remaining);
    let (remaining, (a, _, b)) = delimited(
        tag("mul("),
        tuple((
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
        )),
        tag(")"),
    )(remaining)?;
    println!("{:?}", remaining);
    println!("{a}");
    println!("{b}");
    Ok((remaining, Mul(a, b)))
}

pub fn part1(input: &str) -> u32 {
    let (_, muls) = many1(opt(parse_mul))(input).unwrap();
    println!("{:#?}", muls);
    0
}

pub fn part2(input: &str) -> u32 {
    dbg!(input);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const TEST_AOC: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_NON_ALPHA: &str = "%&mul[3,7]!@^do_not_mul(5,5)";
    // const TEST2: &str = "mul(1,1)mul(2,2)mul(3,3)";
    // const TEST3: &str = "mul(10,20)mul(30,40)mul(50,60)";
    // const TEST4: &str = "mul(100,200)mul(300,400)mul(500,600)";
    // const TEST5: &str = "mul(0,0)mul(0,1)mul(1,0)";
    // const SIMPLE: &str = "mul(2,4)";
    // const MULTIPLE: &str = "mul(1,1)mul(2,2)mul(3,3)";

    #[rstest]
    // #[case(SIMPLE, 8)]
    // #[case(MULTIPLE, 14)]
    #[case(TEST_AOC, 161)]
    #[case(TEST_NON_ALPHA, 25)]
    // #[case(TEST2, 14)]
    // #[case(TEST3, 4400)]
    // #[case(TEST4, 260000)]
    // #[case(TEST5, 0)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1(input), expected);
    }

    // #[rstest]
    // #[case(TEST, 0)]
    // #[case(TEST2, 0)]
    // #[case(TEST3, 0)]
    // #[case(TEST4, 0)]
    // #[case(TEST5, 0)]
    // fn test_part2(#[case] input: &str, #[case] expected: u32) {
    //     assert_eq!(part2(input), expected);
    // }
}
