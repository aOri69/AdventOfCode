enum MoveDirection {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

struct Position {
    x: u32,
    y: u32,
}

struct Rope {
    head: Position,
    tail: Position,
}

pub fn part_1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13);
    }
}
