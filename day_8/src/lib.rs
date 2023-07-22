use crate::tree::Forest;

mod tree;

pub fn part_1(input: &str) -> usize {
    let grid = Forest::build(input);
    // dbg!(&grid);
    // dbg!(PrettyVisibilityGrid(&grid.get_visibility_grid()));
    let result = grid
        .get_visibility_grid()
        .iter()
        .flatten()
        .filter(|&b| b == &true)
        .count();
    // println!("Total number of visible trees on the map: {}", result);
    result
}

pub fn part_2(_input: &str) -> usize {
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
        assert_eq!(part_1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 8);
    }
}
