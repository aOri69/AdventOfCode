use anyhow::{Ok, Result};
use pipe::build_surface;
use thiserror::Error;
use tui::run_tui;
pub mod pipe;
mod tui;

/// Main errors of the application
#[derive(Debug, Error)]
pub enum ArgumentError {
    #[error("No argument given. 1 or 2 are possible entries")]
    NoArg,
    #[error("Wrong argument given: {0}. 1 or 2 are possible entries")]
    WrongArg(usize),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Main executable function for process parts of the task
pub fn solve_parts(input: &str) -> Result<(usize, usize)> {
    let mut surface = build_surface(input)?;
    if cfg!(test) {
        while !surface.search().finished() {
            surface.update();
            // dbg!(surface.search().queue());
        }
        let result = surface
            .search()
            .distances()
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .max()
            .unwrap_or(&0);
        dbg!(&result);
        Ok((*result, 0))
    } else {
        run_tui(input)?;
        Ok((0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn square_loop_part1() {
        const PIPE: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solve_parts(PIPE).unwrap().0, 4);
    }

    #[test]
    fn complex_loop_part1() {
        const PIPE: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solve_parts(PIPE).unwrap().0, 8);
    }

    #[test]
    fn part2_1() {
        const PIPE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve_parts(PIPE).unwrap().1, 4);
    }

    #[test]
    fn part2_2() {
        const PIPE: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(solve_parts(PIPE).unwrap().1, 4);
    }

    #[test]
    fn part2_3() {
        const PIPE: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve_parts(PIPE).unwrap().1, 8);
    }

    #[test]
    fn part2_4() {
        const PIPE: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve_parts(PIPE).unwrap().1, 10);
    }
}
