use rope_bridge::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    println!("How many positions does the tail of the rope visit at least once?");
    println!("Part 1 answer: {:?}", part_1(input));
    println!("Part 2 answer: {:?}", part_2(input));

    if let Err(e) = rope_bridge::run_gui() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
