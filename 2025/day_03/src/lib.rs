type Joltage = u64;

#[derive(Debug)]
struct Bank(Vec<Joltage>);

impl Bank {
    fn new(items: Vec<Joltage>) -> Self {
        Self(items)
    }

    fn get_max_joltage(&self) -> Joltage {
        let first_max;
        let second_max;
        let (max_idx, max_val) =
            Bank::get_max(self.0.as_slice()).expect("Expected to find the maximum");

        if max_idx == self.0.len() - 1 {
            second_max = max_val;
            let mut bank_copy = self.0.clone();
            let _ = bank_copy.remove(max_idx);
            first_max = *bank_copy.iter().max().expect("expected to find the max");
        } else {
            first_max = max_val;
            second_max = Bank::get_max(
                self.0
                    .get(max_idx + 1..)
                    .expect("expected to get the slice"),
            )
            .expect("Expected to get second max")
            .1; // Accessing second field in the tuple(we don't need index this time)
        };

        (first_max.to_string() + second_max.to_string().as_str())
            .parse::<Joltage>()
            .expect("expected to parse two digit number")
    }

    fn get_max(slice: &[Joltage]) -> Option<(usize, Joltage)> {
        slice
            .iter()
            .enumerate()
            .fold(None, |acc, (i, &j)| match acc {
                None => Some((i, j)),
                Some((_, next_j)) if j > next_j => Some((i, j)),
                Some(tuple) => Some(tuple),
            })
    }
}

pub fn part1(input: &str) -> Joltage {
    let banks = input
        .lines()
        .map(|line| {
            Bank::new(
                line.chars()
                    .map(|c| c.to_digit(10).expect("expected a digit") as Joltage)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    banks.iter().map(|b| b.get_max_joltage()).sum()
}

pub fn part2(_input: &str) -> Joltage {
    todo!("Part 2 implementation");
}

/// # Part 1
///
/// - In **98**7654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
/// - In **8**1111111111111**9**, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
/// - In 2342342342342**78**, you can make 78 by turning on the last two batteries (marked 7 and 8).
/// - In 818181**9**1111**2**111, the largest joltage you can produce is 92.
///
/// # Part 2
///
/// - In `_987654321111_111`, the largest joltage can be found by turning on everything except some `1`s at the end to produce `_987654321111_`.
/// - In the digit sequence `_81111111111_111_9_`, the largest joltage can be found by turning on everything except some `1`s, producing `_811111111119_`.
/// - In `23_4_2_34234234278_`, the largest joltage can be found by turning on everything except a `2` battery, a `3` battery, and another `2` battery near the start to produce `_434234234278_`.
/// - In `_8_1_8_1_8_1_911112111_`, the joltage `_888911112111_` is produced by turning on everything except some `1`s near the front.
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 357)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 3121910778619)
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("8992", 99)]
    fn test_part1_vec_of_invalid_ids(#[case] input: &str, #[case] expected: Joltage) {
        let max = Bank::new(
            input
                .chars()
                .map(|c| c.to_digit(10).expect("expected a digit") as Joltage)
                .collect::<Vec<_>>(),
        )
        .get_max_joltage();

        assert_eq!(max, expected);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    #[case("8992", 99)]
    fn test_part2_vec_of_invalid_ids(#[case] input: &str, #[case] expected: Joltage) {
        let max = Bank::new(
            input
                .chars()
                .map(|c| c.to_digit(10).expect("expected a digit") as Joltage)
                .collect::<Vec<_>>(),
        )
        .get_max_joltage();

        assert_eq!(max, expected);
    }
}
