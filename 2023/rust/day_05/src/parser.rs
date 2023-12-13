use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use crate::{Almanac, Seed};

#[derive(Debug)]
pub struct SeedRange {
    source_name: String,
    destination_name: String,
    destination_start: Seed,
    source_start: Seed,
    length: Seed,
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let main_parser = tuple((parse_seeds, many1(parse_source_to_dest)));
    let (_empty, (seeds, ranges)) = all_consuming(main_parser)(input)?;
    Ok(("", Almanac { seeds, ranges }))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    terminated(
        preceded(
            tag("seeds: "),
            separated_list1(space1, nom::character::complete::u32),
        ),
        line_ending,
    )(input)
}

fn parse_source_to_dest(input: &str) -> IResult<&str, Vec<SeedRange>> {
    use nom::character::complete::u32 as nom_u32;
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
    many1(map(
        tuple((nom_u32, space1, nom_u32, space1, nom_u32, opt(line_ending))),
        |(destination_start, _, source_start, _, length, _)| SeedRange {
            source_name: source_name.to_string(),
            destination_name: destination_name.to_string(),
            destination_start,
            source_start,
            length,
        },
    ))(input)
}
