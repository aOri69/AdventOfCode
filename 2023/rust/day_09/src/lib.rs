pub fn part1(input: &str) -> i32 {
    let values = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("expected valid input");

    values.into_iter().map(|vh| predict_values(&vh).1).sum()
}

pub fn part2(input: &str) -> i32 {
    let values = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("expected valid input");

    values.into_iter().map(|vh| predict_values(&vh).0).sum()
}

fn predict_values(value_history: &[i32]) -> (i32, i32) {
    let deltas = get_deltas(value_history);
    let (mut prediction_left, mut prediction_right) = (0, 0);

    for delta in deltas.into_iter().rev().skip(1) {
        // println!("{delta:?}");
        prediction_left = delta.first().unwrap() - prediction_left;
        prediction_right += delta.last().unwrap();
    }
    // println!();

    (prediction_left, prediction_right)
}

fn get_deltas(value_history: &[i32]) -> Vec<Vec<i32>> {
    let result = vec![Vec::from(value_history)];
    append_delta(result)
}

fn append_delta(mut delta_container: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let next_delta = delta_container
        .last()
        .unwrap()
        .windows(2)
        .map(|p| p[1] - p[0])
        .collect::<Vec<_>>();

    delta_container.push(next_delta.clone());

    if !next_delta.iter().all(|&n| n == 0) {
        delta_container = append_delta(delta_container);
    }

    delta_container
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_test_input() {
        let result = part2(TEST_INPUT.lines().last().unwrap());
        assert_eq!(result, 5);
    }
}
