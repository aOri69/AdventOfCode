use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use crate::{Map, Seed, SeedRange};

pub struct ParseResult {
    pub seeds: Vec<Seed>,
    pub maps: Vec<Map>,
}

pub fn parse_input(input: &str) -> Result<ParseResult, nom::error::Error<String>> {
    let main_parser = tuple((parse_seeds, many1(parse_source_to_dest)));
    match all_consuming(main_parser)(input).finish() {
        Ok((_empty, (seeds, maps))) => Ok(ParseResult { seeds, maps }),
        Err(e) => Err(nom::error::Error {
            input: e.input.to_string(),
            code: e.code,
        }),
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    terminated(
        preceded(
            tag("seeds: "),
            separated_list1(space1, nom::character::complete::u64),
        ),
        line_ending,
    )(input)
}

fn parse_source_to_dest(input: &str) -> IResult<&str, Map> {
    use nom::character::complete::u64 as nom_u64;
    // \r\n or \n
    // seed-to-soil map:
    let (input, (source_name, destination_name)) = preceded(
        line_ending,
        separated_pair(
            alpha1,
            tag("-to-"),
            terminated(terminated(alpha1, tag(" map:")), line_ending),
        ),
    )(input)?;

    // 50 98 2\r\n
    let (input, mut ranges) = many1(map(
        tuple((nom_u64, space1, nom_u64, space1, nom_u64, opt(line_ending))),
        |(destination_start, _, source_start, _, length, _)| SeedRange {
            source_range: source_start..=source_start + length - 1,
            dest_range: destination_start..=destination_start + length - 1,
        },
    ))(input)?;

    ranges.sort();

    Ok((
        input,
        Map {
            source: source_name.to_string(),
            destination: destination_name.to_string(),
            ranges,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::carrige_return("seeds: 3 1\r\n",vec![3,1])]
    #[case::no_carrige_return("seeds: 2 1\n",vec![2,1])]
    fn test_parse_seeds(#[case] input: &str, #[case] expected: Vec<Seed>) {
        let result = parse_seeds(input);
        assert_eq!(result, Ok(("", expected)));
    }
}
