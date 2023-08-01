#![allow(unused_variables, dead_code)]
// Old solution
mod movement;
mod rope_old;

// New solution
mod command;
mod guiapp;
mod rope;

pub use guiapp::run_gui;

pub fn part_1(input: &str) -> usize {
    use crate::movement::get_commands;
    use crate::rope_old::Rope;

    let mut rope = Rope::new();
    let commands = get_commands(input);

    commands
        .into_iter()
        .for_each(|cmd| rope.process_movement(cmd));
    // dbg!(&rope);
    rope.tail_visits_count()
    // todo!("part1")
}

pub fn part_1_new(input: &str) -> usize {
    use command::Command;
    use rope::Rope;
    let mut rope = Rope::new(1);
    dbg!(&rope);
    let commands = Command::get_commands(input).expect("expected all commands to be parsed");
    for cmd in commands {
        rope.process_command(cmd);
        dbg!(&rope);
    }
    rope.tail_visits_count()
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
    fn test_part_1_new() {
        assert_eq!(part_1_new(TEST_INPUT), 13);
    }
}
