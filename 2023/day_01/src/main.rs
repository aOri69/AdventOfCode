fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|d| d.first().unwrap() * 10 + d.last().unwrap())
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    // let mut v = vec![];
    // one,two,three,four,five,six,seven,eight,nine;
    let mut result = 0u32;
    for line in input.lines() {
        let l = get_left_digit(line);
        let r = get_right_digit(line);
        // dbg!(l + r);
        result += l + r;
    }
    result
}

fn get_right_digit(line: &str) -> u32 {
    for (mut i, _) in line.char_indices().rev() {
        // dbg!(line[..i + 1].to_string());
        i += 1;
        if let Some(d) = line.chars().nth(i - 1).unwrap().to_digit(10) {
            return d;
        }
        if line[..i].ends_with("one") {
            return 1;
        }
        if line[..i].ends_with("two") {
            return 2;
        }
        if line[..i].ends_with("three") {
            return 3;
        }
        if line[..i].ends_with("four") {
            return 4;
        }
        if line[..i].ends_with("five") {
            return 5;
        }
        if line[..i].ends_with("six") {
            return 6;
        }
        if line[..i].ends_with("seven") {
            return 7;
        }
        if line[..i].ends_with("eight") {
            return 8;
        }
        if line[..i].ends_with("nine") {
            return 9;
        }
    }
    0
}

fn get_left_digit(line: &str) -> u32 {
    for (i, _) in line.char_indices() {
        if line[i..].starts_with(|c| matches!(c, 'o' | 't' | 'f' | 's' | 'e' | 'n' | '1'..='9')) {
            if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
                return d * 10;
            }
            if line[i..].starts_with("one") {
                return 10;
            }
            if line[i..].starts_with("two") {
                return 20;
            }
            if line[i..].starts_with("three") {
                return 30;
            }
            if line[i..].starts_with("four") {
                return 40;
            }
            if line[i..].starts_with("five") {
                return 50;
            }
            if line[i..].starts_with("six") {
                return 60;
            }
            if line[i..].starts_with("seven") {
                return 70;
            }
            if line[i..].starts_with("eight") {
                return 80;
            }
            if line[i..].starts_with("nine") {
                return 90;
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 answer: {}", part_1(input));
    println!("Part 2 answer: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGITS_ONLY: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const DIGITS_AND_WORDS: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DIGITS_ONLY), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(DIGITS_AND_WORDS), 281);
    }
}
