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
        '_cols: for (col, c) in line.char_indices().filter_map(|(j, c)| match c {
            c if c.is_ascii_digit() => Some((j, c)),
            _ => None,
        }) {
            println!("(({},{})-{}) ", row, col, c);

            if col - cur_col == 1 || cur_col == 0 {
                number.push(c);
                cur_col = col;
            } else {
                let is_part_number = get_neighbours(row, col)
                    .iter()
                    .inspect(|n| println!("({},{})", n.0, n.1))
                    .any(|(row_n, col_n)| {
                        // Actual coord in the stream of chars
                        let coord = row_n * line_length + col_n;
                        // PLACEHOLDER
                        // TO-DO
                        true
                    });
                if is_part_number {
                    part_numbers.push(number.parse().unwrap());
                }
                dbg!(&number);
                number.clear();
                cur_col = col;
                number.push(c);
            }
        }
    }

    part_numbers.iter().sum()
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
    fn test_part1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        todo!("test for part 2");
        // assert_eq!(part2(INPUT), 4361);
    }
}
