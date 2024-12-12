enum Direction {
    Increasing,
    Decreasing,
}

pub fn part1(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<u32>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for report in reports {
        if is_report_valid(&report) {
            count += 1;
            // dbg!(report);
        }
    }

    count
}

fn is_report_valid(report: &[u32]) -> bool {
    use Direction::*;

    let mut direction = None;
    let result = report.windows(2).all(|window| match window[0] < window[1] {
        true => match direction {
            Some(Increasing) => window[1] - window[0] <= 3 && window[1] - window[0] > 0,
            Some(Decreasing) => false,
            None => {
                direction = Some(Increasing);
                window[1] - window[0] <= 3 && window[1] - window[0] > 0
            }
        },
        false => match direction {
            Some(Increasing) => false,
            Some(Decreasing) => window[0] - window[1] <= 3 && window[0] - window[1] > 0,
            None => {
                direction = Some(Decreasing);
                window[0] - window[1] <= 3 && window[0] - window[1] > 0
            }
        },
    });

    if direction.is_none() {
        return false;
    }

    result
}

pub fn part2(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<u32>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for report in reports {
        // println!("rep - {report:?}");
        if is_report_valid(&report) {
            count += 1;
        } else {
            'inner_removal: for idx_to_remove in 0..report.len() {
                let slice_before_split = &report[..idx_to_remove];
                let slice_after_split = &report[idx_to_remove + 1..];
                let new_vec = [slice_before_split, slice_after_split].concat();
                // println!("bef - {slice_before_split:?}");
                // println!("aft - {slice_after_split:?}");
                // println!("new - {new_vec:?}");
                if is_report_valid(&new_vec) {
                    count += 1;
                    break 'inner_removal;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(INPUT), 2)
    }

    #[test]
    fn it_works2() {
        const INPUT: &str = "10 7 4 2 1";
        assert_eq!(part1(INPUT), 1)
    }

    #[test]
    fn it_works3() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(INPUT), 4)
    }
}
