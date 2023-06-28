pub fn process_signals(_s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_6_examples() {
        assert_eq!(process_signals("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(process_signals("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_signals("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_signals("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
