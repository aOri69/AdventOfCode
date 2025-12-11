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

    fn get_amount_from_ranges_only(&self) -> usize {
        // self.ranges
        //     .iter()
        //     .cloned()
        //     .flatten()
        //     .collect::<HashSet<Id>>()
        //     .len()
        self.ranges
            .iter()
            .fold(0usize, |acc, range| acc + *range.end() - *range.start() + 1)
    }

    fn normalize_ranges(&mut self) {
        let cmp_start = |a: &RangeInclusive<Id>, b: &RangeInclusive<Id>| a.start().cmp(b.start());
        self.ranges.sort_by(cmp_start);
        // dbg!(&self.ranges);
        let mut new_ranges: Vec<RangeInclusive<Id>> = vec![];
        for range in &self.ranges {
            let intersection = new_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(range.start()));
            // dbg!(&intersection);
            match &intersection {
                Some((idx, intersection_range)) => {
                    // dbg!(&intersection_range, &range);
                    if range.start() >= intersection_range.start()
                        && range.end() <= intersection_range.end()
                    {
                        continue;
                    } else if range.start() >= intersection_range.start() {
                        let start_i = *intersection_range.start();
                        // let end_i = *intersection_range.end();
                        new_ranges.remove(*idx);
                        new_ranges.push(start_i..=*range.start() - 1);
                        new_ranges.push(range.clone());
                    } else if range.start() < intersection_range.start() {
                        unreachable!("should not happen")
                    }
                    // dbg!(&new_ranges);
                }
                None => new_ranges.push(range.clone()),
            }
        }
        // dbg!(&new_ranges);
        self.ranges = new_ranges;
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.get_fresh_ids().len()
}

pub fn part2(input: &str) -> usize {
    let mut input = parse_input(input);
    input.normalize_ranges();
    input.get_amount_from_ranges_only()
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

    //     3 4 5 6
    //   2 3 4
    //         5 6 7
    const INTERSECTIONS: &str = "3-6
5-7
2-4
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 14);
    }

    #[test]
    fn test_part2_intersections() {
        let mut input = parse_input(INTERSECTIONS);
        input.normalize_ranges();
        let result = input.get_amount_from_ranges_only();
        assert_eq!(result, 6)
    }
}
