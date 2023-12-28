use day_07::parse_hands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");

    let hands = parse_hands(input)?;
    println!("{hands:#?}");

    Ok(())
}
