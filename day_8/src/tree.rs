pub trait Visible {
    fn is_visible(&self) -> bool;
}

pub trait VerticalIterator: Iterator {}

#[derive(Default)]
pub struct Tree(u8);

impl From<char> for Tree {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap_or_default() as u8)
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

type Grid = Vec<Vec<Tree>>;

pub struct Forest {
    trees: Grid,
}

impl Forest {
    pub fn build(s: &str) -> Forest {
        let mut grid: Grid = vec![];

        for line in s.lines() {
            grid.push(vec![]);
            let last_row = grid.len();
            println!("line {}", last_row);
            for tree in line.chars() {
                grid.last_mut().unwrap().push(Tree::from(tree));
                let last_col = grid.last().unwrap().len();
                println!("col {}", last_col);
            }
        }
        Self { trees: grid }
    }
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", PrettyGrid(&self.trees))
    }
}

impl std::fmt::Debug for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", PrettyGrid(&self.trees))
    }
}

struct PrettyGrid<'a>(pub &'a Grid);

impl std::fmt::Debug for PrettyGrid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        self.0.iter().for_each(|row| {
            // writeln!(f).unwrap();
            // row.iter().for_each(|tree| {
            //     write!(f, "{:?}", tree).unwrap();
            // })
            writeln!(f, "{:?}", row).unwrap_or_default();
        });
        Ok(())
    }
}
