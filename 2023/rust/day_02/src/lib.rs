#![allow(unused)]

use std::{
    io::BufRead,
    ops::{AddAssign, RemAssign},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    fn in_bounds(&self, other: &Set) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_all_sets_in_bounds(&self, compare_set: &Set) -> bool {
        self.sets.iter().all(|s| s.in_bounds(compare_set))
    }
}

fn parse_game(input: &str) -> Game {
    let idx = input.find(':').unwrap();
    let id = input[..idx]
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let sets = input[idx + 1..]
        .split_terminator(';')
        .map(|set| {
            let mut current_set = Set::default();
            set.split_terminator(',').for_each(|cube| {
                let mut it = cube.split_ascii_whitespace();
                let count = it.next().unwrap();
                let count = count.parse::<u32>().unwrap();
                match it.next().unwrap() {
                    "red" => current_set.red += count,
                    "green" => current_set.green += count,
                    "blue" => current_set.blue += count,
                    _ => panic!("no such type of cube"),
                }
            });
            current_set
        })
        .collect();

    Game { id, sets }
}

struct PrettyGames<'a>(&'a Vec<Game>);

impl std::fmt::Debug for PrettyGames<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|g| {
            write!(f, "Game {}: ", g.id);
            write!(f, "{:?}", g.sets);
            writeln!(f);
        });
        Ok(())
    }
}

pub fn part1(input: &str, bag_limit: Set) -> u32 {
    let games = input.lines().map(parse_game).collect::<Vec<_>>();
    dbg!(PrettyGames(&games));

    games
        .iter()
        .filter_map(|g| match g.is_all_sets_in_bounds(&bag_limit) {
            true => Some(g.id),
            false => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAG_LIMIT: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    const PART1_SMALL: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART1_SMALL, BAG_LIMIT), 8);
    }
}
