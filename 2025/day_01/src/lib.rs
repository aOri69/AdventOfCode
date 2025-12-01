const DIAL_START: u8 = 50;

struct Dial {
    value: u8,
    points_to_zero_times: usize,
}

impl Dial {
    pub fn new(start_pos: u8) -> Dial {
        Self {
            value: start_pos,
            points_to_zero_times: 0,
        }
    }

    fn right(&mut self) -> u8 {
        self.value = (self.value + 1) % 100;
        self.value
    }

    fn right_by(&mut self, steps: usize) -> u8 {
        // advance
        self.value = ((self.value as usize + steps) % 100) as u8;
        // check 0
        if self.value == 0 {
            self.points_to_zero_times += 1;
        }
        // return
        self.value
    }

    fn left(&mut self) -> u8 {
        // advance
        self.value = if self.value == 0 { 99 } else { self.value - 1 };
        // check 0
        if self.value == 0 {
            self.points_to_zero_times += 1;
        }
        // return
        self.value
    }

    fn left_by(&mut self, steps: usize) -> u8 {
        for _ in 0..steps {
            self.value = if self.value == 0 { 99 } else { self.value - 1 };
        }
        if self.value == 0 {
            self.points_to_zero_times += 1;
        }
        self.value
    }

    fn points_to_zero_times(&self) -> usize {
        self.points_to_zero_times
    }
}

impl DoubleEndedIterator for Dial {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.left())
    }
}

impl Iterator for Dial {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.right())
    }
}

pub fn part1(input: &str) -> usize {
    let mut dial = Dial::new(DIAL_START);

    for line in input.lines() {
        let (direction, steps) = line.split_at(1);
        let steps = steps.parse::<usize>().expect("Expected to parse integer");

        let _current_dial_value = match direction {
            "L" => dial.left_by(steps),
            "R" => dial.right_by(steps),
            _ => panic!("Expected to parse the input"),
        };
    }

    dial.points_to_zero_times()
}

pub fn part2(input: &str) -> usize {
    let mut dial = Dial::new(DIAL_START);

    for line in input.lines() {
        let (direction, steps) = line.split_at(1);
        let steps = steps.parse::<usize>().expect("Expected to parse integer");

        for _step in 0..steps {
            let _current_dial_value = match direction {
                "L" => dial.left_by(1),
                "R" => dial.right_by(1),
                _ => panic!("Expected to parse the input"),
            };
        }
    }

    dial.points_to_zero_times()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 6);
    }
}
