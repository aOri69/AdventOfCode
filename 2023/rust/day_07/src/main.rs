use day_07::part1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");

    let answer = part1(input);
    println!("Part 1 answer: {answer}");

    Ok(())
}
