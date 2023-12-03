use day_02::{part1, Bag};

fn main() {
    const BAG: Bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part1(input, BAG));
}
