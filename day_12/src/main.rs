use std::{convert::Infallible, str::FromStr};

#[derive(Clone)]
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

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Start => "S".to_owned(),
            Self::End => "E".to_owned(),
            Self::Path(arg0) => arg0.to_string(),
        };
        write!(f, "{:^3}", c)?;
        Ok(())
    }
}

enum Algorithm {
    Dfs,
    Bfs,
}

struct Grid(Vec<Vec<Node>>);

impl Grid {
    fn shortest_path(&self, alg: Algorithm) -> Option<usize> {
        match alg {
            Algorithm::Dfs => self.dfs(),
            Algorithm::Bfs => self.bfs(),
        }
    }

    fn dfs(&self) -> Option<usize> {
        todo!("Depth-first search")
    }

    fn bfs(&self) -> Option<usize> {
        todo!("Breadth-first search")
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for row in self.0.iter() {
            for col in row.iter() {
                write!(f, "{col:?}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Grid {
    type Err = Infallible; //Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(b'a');
        let grid = s
            .lines()
            .map(|s| s.chars().map(Node::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self(grid))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = Grid::from_str(include_str!("../input.txt"))?;
    dbg!(&grid);
    println!(
        "Shortest path using BFS alg: {}",
        grid.shortest_path(Algorithm::Bfs).unwrap_or_default()
    );
    println!(
        "Shortest path using DFS alg: {}",
        grid.shortest_path(Algorithm::Dfs).unwrap_or_default()
    );
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn small_input() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        input.lines().for_each(|l| println!("{l}"));
        let grid = Grid::from_str(input).unwrap();
        dbg!(&grid);

        assert_eq!(grid.shortest_path(Algorithm::Bfs), Some(31));
        assert_eq!(grid.shortest_path(Algorithm::Dfs), Some(31));
    }
}
