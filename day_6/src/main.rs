use tuning_trouble::get_marker_index;

fn main() {
    if let Ok(input) = std::fs::read_to_string("input.txt") {
        println!("Signal marker index: {}", get_marker_index(&input));
    }
}
