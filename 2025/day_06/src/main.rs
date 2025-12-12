use day_06::{part1, part2};

fn main() {
    #[cfg(windows)]
    let input = include_str!("..\\input.txt");
    #[cfg(not(windows))]
    let input = include_str!("../input.txt");

    println!("Part 1: - {}", part1(input));
    println!("Part 2: - {}", part2(input));
}
