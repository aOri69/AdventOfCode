#![allow(unused)]

pub fn part1(input: &str) -> u32 {
    let line_length = input
        .lines()
        .next()
        .expect("should be at least one line in the input")
        .len();
    let line_count = input.lines().count();

    let mut part_numbers: Vec<u32> = vec![];

    '_rows: for (row, line) in input.lines().enumerate() {
        let mut cur_col = 0_usize;
        let mut number = String::new();
        let mut is_part_flag = false;

        let mut cols_it = get_ascii_digits_iter(line);

        while let Some((col, c)) = cols_it.next() {
            // '_cols: for (col, c) in cols_it {
            // print!("(({},{})-{}) ", row, col, c);

            if !is_part_flag {
                is_part_flag = is_part_number(input, row, col);
            }

            if col - cur_col == 1 || cur_col == 0 {
                number.push(c);
                cur_col = col;
            }
            // Next digit not exitst in the line
            // Or next digit is too far from current column(different number)
            if cols_it.peek().is_none() || cols_it.peek().is_some_and(|(c, _)| c.abs_diff(col) > 1)
            {
                if is_part_flag {
                    part_numbers.push(number.parse().unwrap());
                    is_part_flag = false;
                }
                number.clear();
                cur_col = 0;
            }
        }
    }

    // part_numbers.iter().for_each(|n| {
    //     println!("{n}");
    // });
    // let mut r = part_numbers
    //     .iter()
    //     .map(|n| n.to_string())
    //     .collect::<Vec<String>>();
    // r.sort();
    // std::fs::write("non_working.txt", r.join("\n"));
    part_numbers.iter().sum()
}

fn is_part_number(input: &str, row: usize, col: usize) -> bool {
    let is_part_number = get_neighbours(row, col).iter().any(|(row_n, col_n)| {
        let c = input
            .lines()
            .nth(*row_n)
            .unwrap_or("")
            .chars()
            .nth(*col_n)
            .unwrap_or('.');
        c != '.' && !c.is_ascii_digit()
    });
    is_part_number
}

fn get_ascii_digits_iter(
    line: &str,
) -> std::iter::Peekable<impl Iterator<Item = (usize, char)> + '_> {
    line.char_indices()
        .filter_map(|(j, c)| match c {
            c if c.is_ascii_digit() => Some((j, c)),
            _ => None,
        })
        .peekable()
}

fn get_neighbours(row: usize, col: usize) -> Vec<(usize, usize)> {
    // Neighbour coordinates
    [
        (row.checked_add(1), Some(col)),
        (row.checked_add(1), col.checked_add(1)),
        (row.checked_add(1), col.checked_sub(1)),
        (Some(row), col.checked_add(1)),
        (Some(row), col.checked_sub(1)),
        (row.checked_sub(1), Some(col)),
        (row.checked_sub(1), col.checked_add(1)),
        (row.checked_sub(1), col.checked_sub(1)),
    ]
    .iter()
    .filter(|(r, c)| r.is_some() && c.is_some())
    .map(|(r, c)| (r.unwrap(), c.unwrap()))
    .collect()
}

pub fn part2(_input: &str) -> u32 {
    todo!("Part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("input.txt");
        assert_eq!(part1(input), 525119);
    }

    #[test]
    fn test_obvious() {
        const CONT_INPUT: &str = "...&3..501.13..195......&.........
........./....*.........11........";
        assert_eq!(part1(CONT_INPUT), 710);
    }

    #[test]
    fn test_part2() {
        todo!("test for part 2");
        // assert_eq!(part2(INPUT), 4361);
    }
}
