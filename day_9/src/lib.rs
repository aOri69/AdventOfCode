// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(unused_variables, dead_code)]

// Old solution
mod movement;
mod rope_old;
// New solution
mod app;
mod command;
mod rope;

pub use app::run_gui;

pub fn part_1(input: &str) -> usize {
    let mut rope = rope::Rope::new(2);
    let commands =
        command::Command::get_commands(input).expect("expected all commands to be parsed");
    commands
        .into_iter()
        .for_each(|cmd| rope.process_command(cmd));
    rope.tail_visits_count()
}

pub fn part_2(input: &str) -> usize {
    let mut rope = rope::Rope::new(10);
    let commands =
        command::Command::get_commands(input).expect("expected all commands to be parsed");
    commands
        .into_iter()
        .for_each(|cmd| rope.process_command(cmd));
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

    const TEST_INPUT_LONG_MOVE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13);
        assert_eq!(part_1(TEST_INPUT_LONG_MOVE), 88);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 1);
        assert_eq!(part_2(TEST_INPUT_LONG_MOVE), 36);
    }
}
