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

pub fn part2(_input: &str) -> usize {
    todo!("Part 2 implementation");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 4277556);
    }

    #[test]
    fn test_part2() {
        todo!("Part 2 UT");
    }
}
