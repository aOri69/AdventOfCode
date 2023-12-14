use std::ops::RangeInclusive;

use crate::parser::{parse_input, ParseResult};

type Seed = u64;

mod parser;

struct Map {
    source: String,
    destination: String,
    ranges: Vec<SeedRange>,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}-to-{} map:", self.source, self.destination)?;
        self.ranges.iter().try_for_each(|r| writeln!(f, "{r:?}"))?;
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
struct SeedRange {
    source_range: RangeInclusive<Seed>,
    dest_range: RangeInclusive<Seed>,
}

impl SeedRange {
    fn get_dest(&self, seed: Seed) -> Seed {
        todo!();
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SeedRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_range.start().cmp(other.source_range.start())
    }
}

impl core::hash::Hash for SeedRange {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source_range.start().hash(state);
    }
}

impl std::fmt::Debug for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "src: {:>11}..={:<11}, dst: {:>11}..={:<11}",
            self.source_range.start(),
            self.source_range.end(),
            self.dest_range.start(),
            self.dest_range.end()
        )?;
        Ok(())
    }
}

pub fn part_1(input: &str) -> u32 {
    let ParseResult { seeds, mut maps } = parse_input(input).unwrap();
    dbg!(maps);
    todo!("Part 1 implementation");
}

pub fn part_2(_input: &str) -> u32 {
    todo!("Part 2 implementation");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
    }

    #[test]
    fn test_part_2() {
        todo!("part2 test function");
    }
}
