use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum RoundResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
impl RoundResult {
    pub fn value(&self) -> isize {
        match self {
            RoundResult::Loss => RoundResult::Loss as isize,
            RoundResult::Draw => RoundResult::Draw as isize,
            RoundResult::Win => RoundResult::Win as isize,
        }
    }
    pub fn get_shape_for_opponent(&self, op: &Shape) -> Shape {
        match self {
            RoundResult::Loss => match op {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            RoundResult::Draw => op.clone(),
            RoundResult::Win => match op {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
}
impl std::fmt::Display for RoundResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RoundResult::Loss => "Loss",
            RoundResult::Draw => "Draw",
            RoundResult::Win => "Win",
        })
    }
}
impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err("Wrong round result".to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn value(&self) -> isize {
        match self {
            Shape::Rock => Shape::Rock as isize,
            Shape::Paper => Shape::Paper as isize,
            Shape::Scissors => Shape::Scissors as isize,
        }
    }
    pub fn get_round_result(&self, op: &Self) -> RoundResult {
        match self {
            Shape::Rock => match op {
                Shape::Rock => RoundResult::Draw,
                Shape::Paper => RoundResult::Loss,
                Shape::Scissors => RoundResult::Win,
            },
            Shape::Paper => match op {
                Shape::Rock => RoundResult::Win,
                Shape::Paper => RoundResult::Draw,
                Shape::Scissors => RoundResult::Loss,
            },
            Shape::Scissors => match op {
                Shape::Rock => RoundResult::Loss,
                Shape::Paper => RoundResult::Win,
                Shape::Scissors => RoundResult::Draw,
            },
        }
    }
}
impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Wrong shape".to_string()),
        }
    }
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Shape::Rock => "Rock",
            Shape::Paper => "Paper",
            Shape::Scissors => "Scissors",
        })
    }
}

pub fn build_strategy_map(s: &str) -> Vec<(Shape, Shape)> {
    s.lines()
        .map(|l| {
            let mut it = l.split_whitespace().take(2);
            let left = Shape::from_str(it.next().unwrap()).unwrap();
            let right = Shape::from_str(it.next().unwrap()).unwrap();
            (left, right)
        })
        .collect()
}

pub fn build_strategy_map_from_result(s: &str) -> Vec<(Shape, Shape)> {
    s.lines()
        .map(|l| {
            let mut it = l.split_whitespace().take(2);
            let left = Shape::from_str(it.next().unwrap()).unwrap();
            // Get result of the round
            let result = RoundResult::from_str(it.next().unwrap()).unwrap();
            // Get Shape based on the round result
            let right = result.get_shape_for_opponent(&left);
            (left, right)
        })
        .collect()
}

pub fn calculate_points(strategy: Vec<(Shape, Shape)>) -> u32 {
    strategy
        .iter()
        .map(|(op, you)| {
            // println!("{you} vs {op} -> {}", you.get_round_result(op));
            // dbg!(you.value() + you.get_round_result(op).value());
            you.get_round_result(op).value() as u32 + you.value() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapes() {
        assert_eq!(Shape::from_str("A").unwrap(), Shape::Rock);
        assert_eq!(Shape::from_str("X").unwrap(), Shape::Rock);
        assert_eq!(Shape::Rock.value(), 1);
    }

    #[test]
    #[should_panic]
    fn test_undefined_shapes() {
        Shape::from_str("D").unwrap();
    }

    #[test]
    fn test_simple_strategy_part1() {
        let strategy = build_strategy_map(
            "A Y
        B X
        C Z",
        );
        let points = calculate_points(strategy);
        assert_eq!(points, 15);
    }

    #[test]
    fn test_simple_strategy_part2() {
        let strategy = build_strategy_map_from_result(
            "A Y
        B X
        C Z",
        );
        let points = calculate_points(strategy);
        assert_eq!(points, 12);
    }
}
