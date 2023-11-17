use std::convert::Infallible;

#[derive(Debug, Clone)]
enum Node {
    Start,
    End,
    Path(u8),
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            'E' => Self::End,
            c => Self::Path(c as u8 - b'a'),
        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Node>>);

impl Grid {}

impl std::str::FromStr for Grid {
    type Err = Infallible; //Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!("{}", b'a');
        let grid = s
            .lines()
            .map(|s| s.chars().map(Node::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self(grid))
    }
}

fn main() {
    let _input = include_str!("../input.txt");
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn small_input() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        input.lines().for_each(|l| println!("{l}"));
        let grid = Grid::from_str(input);
        dbg!(grid.unwrap());
        todo!()
    }
}
