type Seed = u32;

struct Almanac;

mod parser;

pub fn part_1(_input: &str) -> u32 {
    todo!("Part 1 implementation");
}

pub fn part_2(_input: &str) -> u32 {
    todo!("Part 2 implementation");
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
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
