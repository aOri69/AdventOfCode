use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut splits_count = 0;
    let mut beams = HashSet::new();
    for (line_idx, line) in input.lines().enumerate() {
        let mut debug_line = Vec::from_iter(line.chars());
        if line_idx == 0 {
            let starting_point_idx = line.find('S').expect("Expected to find starting point");
            beams.insert(starting_point_idx);
            continue;
        }
        for beam in beams.clone() {
            match line.chars().nth(beam) {
                Some('.') => {
                    beams.insert(beam);
                    debug_line[beam] = '|';
                }
                Some('^') => {
                    beams.remove(&beam);
                    splits_count += 1;
                    if beam != 0 && beams.insert(beam - 1) {
                        debug_line[beam - 1] = '|';
                    }
                    if beam != line.len() - 1 && beams.insert(beam + 1) {
                        debug_line[beam + 1] = '|';
                    }
                }
                Some(_) => unreachable!(),
                None => unreachable!(),
            }
        }
        println!("{}", String::from_iter(debug_line.iter()));
    }
    splits_count
}

pub fn part2(_input: &str) -> usize {
    todo!("Part 2 implementation");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 21);
    }

    #[test]
    fn test_part2() {
        todo!("Part 2 UT");
    }
}
