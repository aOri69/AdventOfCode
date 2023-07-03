use no_space_left_on_device::parse_command;

fn main() {
    let input = include_str!("../input.txt");
    // println!("{:?}", input);
    let res = input
        .lines()
        .filter_map(|l| parse_command(l).ok())
        .collect::<Vec<_>>();
    println!("{res:?}");
}
