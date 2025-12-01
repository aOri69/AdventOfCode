use std::collections::HashMap;

pub fn part1(input: &str) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    input.lines().for_each(|line| {
        let mut it = line.split_ascii_whitespace().take(2);
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        right.push(it.next().unwrap().parse::<i32>().unwrap());
    });
    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("Part 1 - {result}");
}
pub fn part2(input: &str) {
    let mut left = Vec::with_capacity(1000);
    let mut right = HashMap::new();

    input.lines().for_each(|line| {
        let mut it = line.split_ascii_whitespace().take(2);
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        *right
            .entry(it.next().unwrap().parse::<i32>().unwrap())
            .or_insert(0) += 1;
    });

    let result = left
        .iter()
        .map(|l| l * *right.get(l).unwrap_or(&0))
        .sum::<i32>();

    println!("Part 2 - {result}");
}
