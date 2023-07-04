use no_space_left_on_device::parse_line;
use nom::{combinator::all_consuming, Finish};

fn main() {
    let input = include_str!("../input.txt");

    let parsed_lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    parsed_lines.for_each(|line| println!("{line:?}"));
}
