use tuning_trouble::{get_marker_index, marker_start, message_start};

fn main() {
    if let Ok(input) = std::fs::read_to_string("input.txt") {
        println!("-------------My solution-------------");
        println!("Signal marker index: {:?}", get_marker_index(&input, 4));
        println!("Message index: {:?}", get_marker_index(&input, 14));
        println!("-------------Ring buffer-------------");
        println!("Signal marker index: {:?}", marker_start(&input));
        println!("Message index: {:?}", message_start(&input));
    }
}
