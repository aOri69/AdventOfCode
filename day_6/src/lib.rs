use std::collections::HashSet;

/// # My Solution
/// Subroutine that detects a start-of-packet marker in the datastream.
/// In the protocol being used by the Elves,
/// the start of a packet is indicated
/// by a sequence of four characters that are all different.
pub fn get_marker_index(s: &str, seq_size: usize) -> Option<usize> {
    let mut idx: usize = 0;
    while let Some(s) = s.get(idx..idx + seq_size) {
        // dbg!(s);
        if let Some(advance) = find_duplicate_index(s) {
            idx += advance + 1;
            continue;
        }
        return Some(idx + seq_size);
    }
    None
}

fn find_duplicate_index(s: &str) -> Option<usize> {
    let mut unique_chars = HashSet::new();
    s.char_indices()
        .find_map(|(i, c)| match unique_chars.insert(c).then_some(i) {
            Some(_) => None,
            None => s.chars().position(|i| i == c),
        })
}

/// Ring buffer based solution
pub fn marker_start(input: &str) -> usize {
    const PREV_SIZE: usize = 3;
    let mut prev = [' '; PREV_SIZE];
    prev.copy_from_slice(&input.chars().collect::<Vec<_>>()[..PREV_SIZE]);
    for (ix, c) in input.chars().skip(PREV_SIZE).enumerate() {
        if !prev.contains(&c) && is_uniq(&prev) {
            return ix + PREV_SIZE + 1;
        } else {
            prev[ix % PREV_SIZE] = c;
        }
    }

    unreachable!("Input contains no message marker")
}

/// Ring buffer based solution
pub fn message_start(input: &str) -> usize {
    const PREV_SIZE: usize = 13;
    let mut prev = [' '; PREV_SIZE];
    prev.copy_from_slice(&input.chars().collect::<Vec<_>>()[..PREV_SIZE]);
    for (ix, c) in input.chars().skip(PREV_SIZE).enumerate() {
        if !prev.contains(&c) && is_uniq(&prev) {
            return ix + PREV_SIZE + 1;
        } else {
            prev[ix % PREV_SIZE] = c;
        }
    }

    unreachable!("Input contains no message marker")
}

fn is_uniq(s: &[char]) -> bool {
    let set = s.iter().collect::<HashSet<_>>();
    matches!(set.len(), len if len == s.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    // Markers tests with 4
    #[test]
    fn test_marker_index_1() {
        assert_eq!(
            get_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
    }
    #[test]
    fn test_marker_index_2() {
        assert_eq!(get_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    }
    #[test]
    fn test_marker_index_3() {
        assert_eq!(get_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    }
    #[test]
    fn test_marker_index_4() {
        assert_eq!(
            get_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }
    #[test]
    fn test_marker_index_5() {
        assert_eq!(
            get_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }
    // Without markers
    #[test]
    fn test_without_markers() {
        assert_eq!(get_marker_index("aaaabbbbccccd", 4), None);
    }
    // Message tests with 14
    #[test]
    fn test_message_index_1() {
        assert_eq!(
            get_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
    }
    #[test]
    fn test_message_index_2() {
        assert_eq!(
            get_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
    }
    #[test]
    fn test_message_index_3() {
        assert_eq!(
            get_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
    }
    #[test]
    fn test_message_index_4() {
        assert_eq!(
            get_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
    }
    #[test]
    fn test_message_index_5() {
        assert_eq!(
            get_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
}
