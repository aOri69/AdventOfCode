type Id = usize;
type IdRange = std::ops::RangeInclusive<Id>;

pub fn part1(input: &str) -> usize {
    let ranges = prepare_ranges(input);
    let mut invalid_ids_sum = 0usize;
    for range in ranges {
        let invalid_ids = get_invalid_ids(range);
        invalid_ids_sum += invalid_ids.iter().sum::<Id>();
    }
    invalid_ids_sum
}

pub fn part2(_input: &str) -> usize {
    // todo!("Part 2 implementation");
    0
}

fn prepare_ranges(input: &str) -> Vec<IdRange> {
    input
        .split(',')
        .map(|range_str| {
            let mut range_it = range_str.split('-');
            let left = range_it
                .next()
                .expect("Expected to get lower value of the range")
                .parse::<Id>()
                .expect("Expected number");
            let right = range_it
                .next()
                .expect("Expected to get higher value of the range")
                .trim_ascii()
                .parse::<Id>()
                .expect("Expected number");
            left..=right
        })
        .collect::<Vec<_>>()
}

fn get_invalid_ids(range: IdRange) -> Vec<Id> {
    range
        .filter(|value| match is_id_invalid(*value) {
            true => true,
            false => false,
        })
        .collect()
}

fn is_id_invalid(id: Id) -> bool {
    let id_str = id.to_string();
    if !id_str.len().is_multiple_of(2) {
        return false;
    }
    let (lhs, rhs) = id_str.split_at(id_str.len() / 2);
    lhs == rhs
}

/// 11-22 has two invalid IDs, 11 and 22.
/// 95-115 has one invalid ID, 99.
/// 998-1012 has one invalid ID, 1010.
/// 1188511880-1188511890 has one invalid ID, 1188511885.
/// 222220-222224 has one invalid ID, 222222.
/// 1698522-1698528 contains no invalid IDs.
/// 446443-446449 has one invalid ID, 446446.
/// 38593856-38593862 has one invalid ID, 38593859.
/// The rest of the ranges contain no invalid IDs.
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 1227775554)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 4174379265)
    }

    #[rstest]
    #[case("11-22", vec![11,22])]
    #[case("95-115", vec![99])]
    #[case("998-1012", vec![1010])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    #[case("1698522-1698528", vec![])]
    #[case("446443-446449",vec![446446])]
    #[case("38593856-38593862",vec![38593859])]
    fn test_part1_vec_of_invalid_ids(#[case] input: &str, #[case] expected: Vec<usize>) {
        let range = prepare_ranges(input);
        assert_eq!(expected, get_invalid_ids(range.first().unwrap().clone()));
    }

    #[rstest]
    #[case("11-22", vec![11,22])]
    #[case("95-115", vec![99,111])]
    #[case("998-1012", vec![999,1010])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    #[case("1698522-1698528", vec![])]
    #[case("446443-446449",vec![446446])]
    #[case("38593856-38593862",vec![38593859])]
    #[case("565653-565659",vec![565656])]
    #[case("824824821-824824827",vec![824824824])]
    #[case("2121212118-2121212124",vec![2121212121])]
    fn test_part2_vec_of_invalid_ids(#[case] input: &str, #[case] expected: Vec<usize>) {
        let range = prepare_ranges(input);
        assert_eq!(expected, get_invalid_ids(range.first().unwrap().clone()));
    }
}
