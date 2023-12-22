use day_05::{part_1, part_2_threaded};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part_1(input));
    println!("Part 2 answer: {}", part_2_threaded(input));
}
