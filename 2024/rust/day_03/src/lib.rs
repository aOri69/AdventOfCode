use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};
use nom::{character::complete::digit1, multi::separated_list1, sequence::tuple};

#[derive(Debug)]
struct Mul(u32, u32);

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, (_, a, _, b, _)) = tuple((tag("mul("), digit1, tag(","), digit1, tag(")")))(input)?;
    let a: u32 = a.parse().unwrap();
    let b: u32 = b.parse().unwrap();
    Ok((input, Mul(a, b)))
}

pub fn part1(input: &str) -> u32 {
    dbg!(input);
    todo!();
}

pub fn part2(input: &str) {
    dbg!(input);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[rstest]
    #[case(TEST, 161)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1(TEST), 161);
    }

    #[rstest]
    #[case(TEST, 0)]
    fn test_part2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1(TEST), 161);
    }
}
