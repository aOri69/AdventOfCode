use crate::tree::Forest;

mod tree;

pub fn part_1(input: &str) -> Result<u32, String> {
    let grid = Forest::build(input);
    dbg!(grid);
    todo!("part1");
}

pub fn part_2(_input: &str) -> Result<u32, String> {
    todo!("part2");
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), Ok(21));
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(TEST_INPUT), );
    }
}
