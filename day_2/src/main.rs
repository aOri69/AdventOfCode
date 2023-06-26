use rock_paper_scissors::{build_strategy_map, build_strategy_map_from_result, calculate_points};

fn main() {
    let content = include_str!("../input.txt");

    // Part 1
    let strategy = build_strategy_map(content);
    let points = calculate_points(strategy);
    println!("Opponent shape/Your shape rounds result: {points}");
    // Part 2
    let strategy = build_strategy_map_from_result(content);
    let points = calculate_points(strategy);
    println!("Opponent shape/Round results result: {points}");
}
