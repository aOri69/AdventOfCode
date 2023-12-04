use day_02::{part1, part2, Set};

fn main() {
    const BAG: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part1(input, BAG));
    println!("Part 2 answer: {}", part2(input));
}
