use treetop_tree_house::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    println!("How many trees are visible from outside the grid?");
    println!("Part 1 answer: {}", part_1(input));
    println!();
    println!("Part 2 answer: {}", part_2(input));
}
