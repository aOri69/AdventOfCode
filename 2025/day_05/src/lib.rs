use std::ops::RangeInclusive;

type Id = usize;

#[derive(Debug, Default)]
struct Input {
    ranges: Vec<RangeInclusive<Id>>,
    ids: Vec<Id>,
}

impl Input {
    fn get_fresh_ids(&self) -> Vec<Id> {
        self.ids
            .iter()
            .filter(|id| self.ranges.iter().any(|range| range.contains(id)))
            .cloned()
            .collect()
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.get_fresh_ids().len()
}

pub fn part2(_input: &str) -> usize {
    todo!("Part 2 implementation");
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let mut result = Input::default();
    // Ranges
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (l, r) = line
            .split_once('-')
            .expect("expected range with - in the middle");
        result.ranges.push(
            l.parse::<Id>().expect("expected valid ID")
                ..=r.parse::<Id>().expect("expected valid ID"),
        );
    }
    // Numbers
    for line in lines.skip_while(|l| l.is_empty()) {
        result
            .ids
            .push(line.parse::<Id>().expect("expected valid ID"));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 3);
    }

    #[test]
    fn test_part2() {
        todo!("Part 2 UT");
    }

    // #[rstest]
    // #[case("test", "test")]
    // fn test_part1_case(#[case] input: &str, #[case] expected: &str) {
    //     todo!()
    // }
}
