use cathode_ray_tube::{draw_crt, sum_of_signal_strengths};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", sum_of_signal_strengths(&input));
    println!("Part 2:\n{}", draw_crt(&input).join("\n"));
}
