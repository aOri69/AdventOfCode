use day_08::part2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input.txt");

    let answer = part2(input);
    println!("Part 2 answer: {answer}");

    Ok(())
}
