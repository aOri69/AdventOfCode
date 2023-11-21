use std::convert::Infallible;

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

struct Grid(Vec<Vec<Node>>);

impl Grid {}

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
