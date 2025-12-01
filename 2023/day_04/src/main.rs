#![allow(unused)]
use card::parse_cards;

use crate::card::Card;

mod card;

fn part1(input: &str) -> u32 {
    let cards = parse_cards(input).expect("Can't parse all cards");
    let result = cards.iter().map(|c| c.points()).sum::<u32>();
    result
}

fn part2(input: &str) -> u32 {
    let cards = parse_cards(input).expect("Can't parse all cards");
    let mut cards_amount: Vec<usize> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let matches = card.matches_count();
        let multiplier = cards_amount.get(i).copied().unwrap();
        for id in (card.id() as usize..card.id() as usize + matches) {
            if let Some(amount) = cards_amount.get_mut(id) {
                *amount += 1;
                (1..multiplier).for_each(|_| *amount += 1);
            }
        }
    }
    // dbg!(cards_amount);
    cards_amount.into_iter().sum::<usize>() as u32
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part1(input));
    println!("Part 2 answer: {}", part2(input));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 13);
    }

    #[test]
    fn test_multiple_spaces() {
        const MULT_SPACE: &str = "Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(part1(MULT_SPACE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 30);
    }
}
