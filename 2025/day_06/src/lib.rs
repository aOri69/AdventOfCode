use std::str::FromStr;

#[derive(Debug)]
struct Operator(char);

impl Operator {
    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        // dbg!(self.0, lhs, rhs);
        match self.0 {
            '+' => lhs + rhs,
            '*' => lhs * rhs,
            _ => unreachable!("Not implemented for anything but + and *"),
        }
    }

    fn apply_from_vec(&self, numbers: &[usize]) -> usize {
        numbers.iter().fold(0usize, |acc, x| match x {
            x if acc == 0 => *x,
            x => self.apply(acc, *x),
            // _ => panic!("should not be operator"),
        })
    }
}

#[derive(Debug)]
enum InputValue {
    Number(usize),
    Operator(Operator),
}

impl FromStr for InputValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(number) => Ok(Self::Number(number)),
            Err(_) => match s {
                "+" => Ok(Self::Operator(Operator('+'))),
                "*" => Ok(Self::Operator(Operator('*'))),
                s => Err(format!("Error value: {s}")),
            },
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|number| number.parse::<InputValue>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // dbg!(&input);

    let mut result = 0usize;
    for (idx, op) in input
        .last()
        .expect("exptectd non empty vector")
        .iter()
        .enumerate()
    {
        if let InputValue::Operator(operator) = op {
            let line_result = input
                .iter()
                .take(input.len() - 1)
                .fold(0usize, |acc, num| match num.get(idx) {
                    Some(value) => match value {
                        InputValue::Number(number) if acc == 0 => *number,
                        InputValue::Number(number) => operator.apply(acc, *number),
                        InputValue::Operator(_) => panic!("should not be operator"),
                    },
                    None => todo!(),
                });
            result += line_result;
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
pub fn part2(input: &str) -> usize {
    let input = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows = input.len();
    let cols = input[0].len();

    let mut result = 0;
    let mut numbers = vec![];

    for col in (0..cols).rev() {
        let mut number = String::with_capacity(rows - 1);
        for row in 0..rows - 1 {
            // dbg!(&input[row][col]);
            number.push(input[row][col]);
        }
        let number = number.trim();
        if !number.is_empty() {
            numbers.push(usize::from_str(number).expect("expected to parse the number"));
            // dbg!(&number);
        }
        result += match input[rows - 1][col] {
            '+' => {
                let result = Operator('+').apply_from_vec(&numbers);
                numbers.clear();
                result
            }
            '*' => {
                let result = Operator('*').apply_from_vec(&numbers);
                numbers.clear();
                result
            }
            _ => continue,
        };
        // dbg!(&result);
    }
    // dbg!(&result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    const TEST_RIGHT_TO_LEFT_SHORT: &str = "4373\n3141\n858 \n78  \n+   \n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Operator('+').apply_from_vec(&[10, 10, 10]), 30);
        assert_eq!(part2(TEST_RIGHT_TO_LEFT_SHORT), 8324);
        assert_eq!(part2(TEST), 3263827);
    }
}
