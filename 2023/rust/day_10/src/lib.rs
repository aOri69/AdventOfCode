//! --- Day 10: Pipe Maze ---
//!

pub fn part1(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SIMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_COMPLEX: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    /// Distance map for simple case
    /// .....
    /// .012.
    /// .1.3.
    /// .234.
    /// .....
    #[test]
    fn part1_simple_input() {
        let result = part1(TEST_SIMPLE);
        assert_eq!(result, 4);
    }

    /// Distance map for comlex case
    /// ..45.
    /// .236.
    /// 01.78
    /// 14567
    /// 23...
    #[test]
    fn part1_complex_input() {
        let result = part1(TEST_COMPLEX);
        assert_eq!(result, 8);
    }
}
