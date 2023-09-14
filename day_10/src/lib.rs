// #![allow(unused_imports, dead_code, unused_variables, unused_mut)]

use std::collections::VecDeque;
use std::str::FromStr;

mod cpu;
use cpu::{Cpu, Instruction};

pub fn sum_of_signal_strengths(input: &str) -> i32 {
    // 20th, 60th, 100th, 140th, 180th, and 220th cycles
    const MULT_CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];
    let mut commands = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();
    let mut cpu = Cpu::default();
    let mut res: i32 = 0;

    loop {
        if MULT_CYCLES.contains(&cpu.cycle()) {
            let sig_str = cpu.signal_strength();
            res += sig_str;
        }
        // Reached the end of the cycles array
        if MULT_CYCLES
            .last()
            .is_some_and(|&cycle| cycle == cpu.cycle())
        {
            break;
        }
        // Reached the end of the commands deque
        if commands.is_empty() {
            break;
        }

        // Main CPU cycle
        cpu.tick();
        // Check whether command was completed
        if cpu.current_command().is_none() {
            cpu.set_command(commands.pop_front());
        }
    }

    res
}

fn sprite_value(pos: i32) -> u64 {
    const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;
    const SPRITE: u64 = 0b1110000000000000000000000000000000000000;

    let (shifted_sprite, _left) = if pos <= 0 {
        let shift = -(pos - 1);
        SPRITE.overflowing_shl(shift.try_into().unwrap())
    } else {
        let shift = pos - 1;
        SPRITE.overflowing_shr(shift.try_into().unwrap())
    };

    shifted_sprite & DISPLAY_MASK
}

pub fn draw_crt(input: &str) -> Vec<String> {
    const CRT_WIDTH: usize = 40;
    const CRT_HEIGHT: usize = 6;
    const CRT_PIXEL: char = '#';
    const CRT_DOT: char = '.';

    let mut crt_pixels: [[char; CRT_WIDTH]; CRT_HEIGHT] = [[CRT_DOT; CRT_WIDTH]; CRT_HEIGHT];
    let mut commands = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();
    let mut cpu = Cpu::default();

    loop {
        let crt_line_idx = cpu.cycle() as usize / CRT_WIDTH;
        let crt_pixel_idx = cpu.cycle() as usize - crt_line_idx * CRT_WIDTH;

        let crt_pixel = &mut crt_pixels[crt_line_idx][crt_pixel_idx];
        let sprite_position: [i32; 3] = [
            crt_pixel_idx as i32 - 1,
            crt_pixel_idx as i32,
            crt_pixel_idx as i32 + 1,
        ];
        // Main CPU cycle
        cpu.tick();
        if cpu.current_command().is_none() {
            cpu.set_command(commands.pop_front());
        }

        if sprite_position.contains(&cpu.reg_x()) {
            *crt_pixel = '#';
        }

        // Reached the end of commands list
        if commands.is_empty() {
            break;
        }
    }

    crt_pixels
        .into_iter() // iterator over arrays of char40
        .map(String::from_iter) // char40 array to string
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    mod main {
        use crate::tests::constants::CRT_LARGE;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn small_input_zero() {
            use constants::TEST_SMALL;
            let result = sum_of_signal_strengths(TEST_SMALL);
            assert_eq!(result, 0i32);
        }

        #[test]
        fn large_input_non_zero() {
            use constants::TEST_LARGE;
            let result = sum_of_signal_strengths(TEST_LARGE);
            assert_eq!(result, 13140i32);
        }

        #[test]
        fn large_crt() {
            use constants::{CRT_LARGE, TEST_LARGE};
            let result = draw_crt(TEST_LARGE);
            assert_eq!(result, CRT_LARGE.lines().collect::<Vec<_>>());
        }

        #[test]
        fn part_1() {
            let input = include_str!("../input.txt");
            let result = sum_of_signal_strengths(input);
            assert_eq!(result, 14820i32);
        }

        #[test]
        fn part_2() {
            let input = include_str!("../input.txt");
            let expected: Vec<String> = vec![];
            let result = draw_crt(input);
            assert_eq!(
                result,
                CRT_PART_2_ANSWER_RZEKEFHA
                    .lines()
                    .to_owned()
                    .collect::<Vec<_>>()
            );
        }
    }
    mod bitwise {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_sprite_value_minus_1() {
            assert_eq!(
                format!("{:040b}", sprite_value(-1)),
                "1000000000000000000000000000000000000000"
            );
        }

        #[test]
        fn test_sprite_value_0() {
            assert_eq!(
                format!("{:040b}", sprite_value(0)),
                "1100000000000000000000000000000000000000"
            );
        }

        #[test]
        fn test_sprite_value_1() {
            assert_eq!(
                format!("{:040b}", sprite_value(1)),
                "1110000000000000000000000000000000000000"
            );
        }

        #[test]
        fn test_sprite_value_38() {
            assert_eq!(
                format!("{:040b}", sprite_value(38)),
                "0000000000000000000000000000000000000111"
            );
        }

        #[test]
        fn test_sprite_value_39() {
            assert_eq!(
                format!("{:040b}", sprite_value(39)),
                "0000000000000000000000000000000000000011"
            );
        }

        #[test]
        fn test_sprite_value_40() {
            assert_eq!(
                format!("{:040b}", sprite_value(40)),
                "0000000000000000000000000000000000000001"
            );
        }
    }

    mod constants {
        pub const TEST_SMALL: &str = "noop
addx 3
addx -5
";

        pub const TEST_LARGE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        pub const CRT_LARGE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    }

    pub const CRT_PART_2_ANSWER_RZEKEFHA: &str = "###..####.####.#..#.####.####.#..#..##..
#..#....#.#....#.#..#....#....#..#.#..#.
#..#...#..###..##...###..###..####.#..#.
###...#...#....#.#..#....#....#..#.####.
#.#..#....#....#.#..#....#....#..#.#..#.
#..#.####.####.#..#.####.#....#..#.#..#.";
}
