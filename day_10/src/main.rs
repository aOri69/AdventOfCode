use cathode_ray_tube::sum_of_signal_strengths;

fn main() {
    let input = include_str!("../input.txt");
    let result = sum_of_signal_strengths(input);
    println!("Part 1: {result}");
}
