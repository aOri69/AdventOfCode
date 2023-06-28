use std::collections::HashSet;

/// # Purpose
/// Subroutine that detects a start-of-packet marker in the datastream.
/// In the protocol being used by the Elves,
/// the start of a packet is indicated
/// by a sequence of four characters that are all different.
pub fn get_marker_index(s: &str) -> usize {
    for (idx, _c) in s.char_indices() {
        let r = s.get(idx..idx + 4).unwrap();
        dbg!(&r);
        dbg!(is_different(r));
    }
    0
}

fn is_different(s: &str) -> bool {
    let mut set = HashSet::new();
    s.chars().for_each(|c| {
        set.insert(c);
    });
    matches!(set.len(), len if len == s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_6_examples() {
        assert_eq!(get_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(get_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(get_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(get_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
