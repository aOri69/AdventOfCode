#![allow(unused)]

use std::{
    io::BufRead,
    ops::{AddAssign, RemAssign},
};

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Bag {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Bag {
    fn add_cube(&mut self, cube: Cube) {
        match cube {
            Cube::Red(n) => self.red += n,
            Cube::Green(n) => self.green += n,
            Cube::Blue(n) => self.blue += n,
        };
    }

    fn in_bounds(&self, other: &Bag) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
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
            set.split_terminator(',')
                .map(|cube| {
                    let mut it = cube.split_ascii_whitespace();
                    let count = it.next().unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match it.next().unwrap() {
                        "red" => Cube::Red(count),
                        "green" => Cube::Green(count),
                        "blue" => Cube::Blue(count),
                        _ => panic!("no such type of cube"),
                    }
                })
                .collect::<Vec<_>>()
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

pub fn part1(input: &str, bag_limit: Bag) -> u32 {
    let games = input.lines().map(parse_game).collect::<Vec<_>>();
    let mut result = 0;
    // dbg!(PrettyGames(&games));
    //
    for game in games {
        let cubes = game.sets.into_iter().flat_map(|s| s.into_iter());
        let mut current_bag = Bag::default();
        for cube in cubes {
            current_bag.add_cube(cube);
        }
        print!(
            "{:>3}: {:^2}-{:^2}-{:^2}",
            game.id, current_bag.red, current_bag.green, current_bag.blue
        );
        if bag_limit.in_bounds(&current_bag) {
            print!(" fits");
            result += game.id;
        }
        println!();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAG_LIMIT: Bag = Bag {
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
