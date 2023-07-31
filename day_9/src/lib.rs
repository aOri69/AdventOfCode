mod movement;
mod rope;

mod instruction;

use crate::movement::get_commands;
use crate::rope::Rope;

pub fn part_1(input: &str) -> usize {
    let mut rope = Rope::new();
    let commands = get_commands(input);

    commands
        .into_iter()
        .for_each(|cmd| rope.process_movement(cmd));
    // dbg!(&rope);
    rope.tail_visits_count()
    // todo!("part1")
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13);
    }

    #[test]
    fn test_new_command_parser() {
        use instruction::Command;
        let commands = Command::get_commands(TEST_INPUT).unwrap();
        println!("{commands:?}");
        assert_eq!(commands.len(), 8)
    }
}
