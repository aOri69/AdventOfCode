#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_distance_variants(&self) -> Vec<usize> {
        (0..=self.time)
            .map(|ms| {
                let remaining_time = self.time - ms;
                let speed = ms;
                remaining_time * speed
            })
            .collect()
    }

    fn get_number_of_ways_to_beat(&self) -> usize {
        self.get_distance_variants()
            .into_iter()
            .filter(|&d| d > self.distance)
            .count()
    }
}

impl From<(usize, usize)> for Race {
    fn from((time, distance): (usize, usize)) -> Self {
        Self { time, distance }
    }
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines_it = input.lines().take(2);
    let times = lines_it
        .next()
        .expect("First line not found")
        .trim_start_matches("Time: ")
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .expect("expected successful parsing of time");
    let distances = lines_it
        .next()
        .expect("Second line not found")
        .trim_start_matches("Distance: ")
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .expect("expected successful parsing of distance");

    times.into_iter().zip(distances).map(Race::from).collect()
}

fn parse_as_single_race(input: &str) -> Race {
    let mut lines_it = input.lines().take(2);
    let time = lines_it
        .next()
        .expect("First line not found")
        .trim_start_matches("Time: ")
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse()
        .expect("expected successful parsing of time");

    let distance = lines_it
        .next()
        .expect("Second line not found")
        .trim_start_matches("Distance: ")
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse()
        .expect("expected successful parsing of distance");

    Race { time, distance }
}

fn main() {
    let input = include_str!("input.txt");
    let races = parse_races(input);
    let part1_answer: usize = races
        .iter()
        .map(|r| r.get_number_of_ways_to_beat())
        .product();
    println!("Part 1 answer: {part1_answer}");

    let race = parse_as_single_race(input);
    let part2_answer = race.get_number_of_ways_to_beat();
    println!("Part 2 answer: {part2_answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_part1() {
        let races = parse_races(TEST);
        let result: usize = races
            .iter()
            .map(|r| r.get_number_of_ways_to_beat())
            .product();
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let race = parse_as_single_race(TEST);
        let result: usize = race.get_number_of_ways_to_beat();
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_parse_races() {
        let input = "Time: 10 20 30\nDistance: 5 10 15";
        let races = parse_races(input);

        assert_eq!(
            races,
            vec![
                Race {
                    time: 10,
                    distance: 5
                },
                Race {
                    time: 20,
                    distance: 10
                },
                Race {
                    time: 30,
                    distance: 15
                },
            ]
        );
    }

    #[test]
    #[should_panic(expected = "expected successful parsing of time")]
    fn test_parse_races_missing_first_line() {
        let input = "Distance: 5 10 15";
        parse_races(input);
    }

    #[test]
    #[should_panic(expected = "Second line not found")]
    fn test_parse_races_missing_second_line() {
        let input = "Time: 10 20 30";
        parse_races(input);
    }

    #[test]
    #[should_panic(expected = "expected successful parsing of time")]
    fn test_parse_races_invalid_time() {
        let input = "Time: a b c\nDistance: 5 10 15";
        parse_races(input);
    }

    #[test]
    #[should_panic(expected = "expected successful parsing of distance")]
    fn test_parse_races_invalid_distance() {
        let input = "Time: 10 20 30\nDistance: x y z";
        parse_races(input);
    }
}
