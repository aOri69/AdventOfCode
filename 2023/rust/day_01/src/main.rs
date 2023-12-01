use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_until, take_while1};
use nom::character::complete::{alphanumeric0, anychar, digit1, not_line_ending, one_of};
use nom::character::is_alphanumeric;
use nom::combinator::{map, map_res};
use nom::sequence::preceded;
use nom::{Finish, IResult};

fn parse_digit(input: &str) -> IResult<&str, u32> {
    dbg!(input);
    map_res(
        alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
            tag("zero"),
            digit1,
        )),
        |s| match s {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            "zero" => Ok(0),
            s => s.parse::<u32>(),
        },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    dbg!(input);
    let res = take_while1(is_alphanumeric)(input);
    // preceded(not_line_ending, parse_digit)(input);
    dbg!(&res);
    res
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|d| d.first().unwrap() * 10 + d.last().unwrap())
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|d| d.first().unwrap() * 10 + d.last().unwrap())
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 answer: {}", part_1(input));
    println!("Part 2 answer: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGITS_ONLY: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const DIGITS_AND_WORDS: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DIGITS_ONLY), 142);
    }

    #[test]
    fn test_part_2() {
        DIGITS_AND_WORDS.lines().for_each(|line| {
            parse_number(line);
        });
        assert_eq!(part_2(DIGITS_AND_WORDS), 281);
    }
}
