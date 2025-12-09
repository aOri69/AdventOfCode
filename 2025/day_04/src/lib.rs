const PAPER_ROLL: char = '@';

#[derive(Debug)]
struct Index {
    row: usize,
    col: usize,
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .fold(0usize, |acc_row, (row, line)| {
            acc_row
                + line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == PAPER_ROLL)
                    .fold(0usize, |acc_col, (col, _c)| match is_paper_roll_accessible(
                        input,
                        Index { row, col },
                        line.len() + 1, // because of \n
                    ) {
                        true => acc_col + 1,
                        false => acc_col,
                    })
        })
}

pub fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut changed_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    while let (map, Some(extracted_count)) = extract_accessible_rolls(changed_map) {
        changed_map = map;
        result += extracted_count;
    }

    result
}

fn extract_accessible_rolls(map: Vec<Vec<char>>) -> (Vec<Vec<char>>, Option<usize>) {
    let mut count = None;
    let mut new_map = map.clone();
    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == PAPER_ROLL && is_paper_roll_accessible_vec(&map, Index { row, col }) {
                new_map[row][col] = '.';
                count = match count {
                    Some(count) => Some(count + 1),
                    None => Some(1),
                };
            }
        }
    }
    (new_map, count)
}

fn is_paper_roll_accessible(paper_map: &str, index: Index, line_width: usize) -> bool {
    // row×width+col = index of the element in the &str
    let neighbour_indexes = [
        (index.row.checked_add(1)).map(|r| r * line_width + index.col), // down
        (index.row.checked_sub(1)).map(|r| r * line_width + index.col), // up
        index.col.checked_add(1).map(|c| index.row * line_width + c),   // right
        index.col.checked_sub(1).map(|c| index.row * line_width + c),   // left
        index
            .row
            .checked_add(1)
            .and_then(|r| index.col.checked_add(1).map(|c| r * line_width + c)), // down-right
        index
            .row
            .checked_add(1)
            .and_then(|r| index.col.checked_sub(1).map(|c| r * line_width + c)), // down-left
        index
            .row
            .checked_sub(1)
            .and_then(|r| index.col.checked_add(1).map(|c| r * line_width + c)), // up-right
        index
            .row
            .checked_sub(1)
            .and_then(|r| index.col.checked_sub(1).map(|c| r * line_width + c)), // up-left
    ];

    let paper_neighbours_count = neighbour_indexes
        .into_iter()
        .flatten()
        // .inspect(|idx| {
        //     if paper_map.chars().nth(*idx).is_some_and(|c| c == PAPER_ROLL) {
        //         dbg!(idx);
        //         dbg!(paper_map.chars().nth(*idx).unwrap());
        //     }
        // })
        .filter(|idx| paper_map.chars().nth(*idx).is_some_and(|c| c == PAPER_ROLL))
        .count();

    paper_neighbours_count < 4
}

fn is_paper_roll_accessible_vec(map: &[Vec<char>], index: Index) -> bool {
    let height = map.len();
    let width = map.first().map(|r| r.len()).unwrap_or(0);

    let neighbours = [
        (index.row.checked_add(1), Some(index.col)), // down
        (index.row.checked_sub(1), Some(index.col)), // up
        (Some(index.row), index.col.checked_add(1)), // right
        (Some(index.row), index.col.checked_sub(1)), // left
        (index.row.checked_add(1), index.col.checked_add(1)), // down-right
        (index.row.checked_add(1), index.col.checked_sub(1)), // down-left
        (index.row.checked_sub(1), index.col.checked_add(1)), // up-right
        (index.row.checked_sub(1), index.col.checked_sub(1)), // up-left
    ];

    let paper_neighbours_count = neighbours
        .into_iter()
        .filter_map(|(r, c)| {
            let (r, c) = (r?, c?);
            if r < height && c < width {
                Some((r, c))
            } else {
                None
            }
        })
        .filter(|(r, c)| map[*r][*c] == PAPER_ROLL)
        .count();

    paper_neighbours_count < 4
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 43);
    }

    // #[rstest]
    // #[case("test", "test")]
    // fn test_part1_case(#[case] input: &str, #[case] expected: &str) {
    //     todo!()
    // }
}
