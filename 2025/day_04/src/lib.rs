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

pub fn part2(_input: &str) -> usize {
    todo!("Part 2 implementation");
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
        todo!("Part 2 UT");
    }

    // #[rstest]
    // #[case("test", "test")]
    // fn test_part1_case(#[case] input: &str, #[case] expected: &str) {
    //     todo!()
    // }
}
