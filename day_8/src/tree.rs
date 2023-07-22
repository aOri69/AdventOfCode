pub trait Visible {
    fn is_visible(&self) -> bool;
}

pub trait VerticalIterator: Iterator {}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Tree(u32);

impl From<char> for Tree {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap_or_default())
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

type TreeGrid = Vec<Vec<Tree>>;
pub type VisibilityGrid = Vec<Vec<bool>>;
pub type ScoreGrid = Vec<Vec<usize>>;

pub struct Forest {
    trees: TreeGrid,
}

impl Forest {
    pub fn build(s: &str) -> Forest {
        let mut grid: TreeGrid = vec![];

        for line in s.lines() {
            grid.push(vec![]);
            for tree in line.chars() {
                let tree = Tree::from(tree);
                grid.last_mut().unwrap().push(tree);
            }
        }
        Self { trees: grid }
    }

    pub fn get_visibility_grid(&self) -> VisibilityGrid {
        let mut visibility: VisibilityGrid = vec![];

        for (row_idx, row) in self.trees.iter().enumerate() {
            visibility.push(vec![]);
            for (col_idx, _) in row.iter().enumerate() {
                let visiblity_value = Forest::is_visible(&self.trees, row_idx, col_idx);
                visibility
                    .last_mut()
                    .unwrap()
                    .push(visiblity_value.is_some());
            }
        }

        visibility
    }

    pub fn get_score_grid(&self) -> ScoreGrid {
        let mut score_grid: ScoreGrid = vec![];

        for (row_idx, row) in self.trees.iter().enumerate() {
            score_grid.push(vec![]);
            for (col_idx, _tree) in row.iter().enumerate() {
                let score = self.get_score(row_idx, col_idx).unwrap_or_default();
                score_grid.last_mut().unwrap().push(score);
            }
        }
        // dbg!(PrettyScoreGrid(&score_grid));
        score_grid
    }

    fn get_score(&self, row: usize, col: usize) -> Option<usize> {
        let row_vec = self.trees.get(row)?.to_vec();
        let col_vec = TreeColIter::new(&self.trees, col)
            .cloned()
            .collect::<Vec<_>>();
        let current_tree = row_vec.get(col)?;

        if row == row_vec.len() - 1 || row == 0 {
            return Some(0);
        }
        if col == col_vec.len() - 1 || col == 0 {
            return Some(0);
        }

        // dbg!(&self);

        // println!("current tree: {}: {}-{}", current_tree, row, col);
        let left = &row_vec[..col];
        let right = &row_vec[col + 1..];
        // println!("horizontal: {:?}-{:?}", left, right);

        let top = &col_vec[..row];
        let bottom = &col_vec[row + 1..];
        // println!("vertical: {:?}-{:?}", top, bottom);

        let score_left = self.get_score_for_side(left.iter().rev().cloned(), current_tree);
        let score_right = self.get_score_for_side(right.iter().cloned(), current_tree);
        let score_top = self.get_score_for_side(top.iter().rev().cloned(), current_tree);
        let score_bottom = self.get_score_for_side(bottom.iter().cloned(), current_tree);

        // let score_left = left.iter().rev().take_while(|&t| t < current_tree).count();
        // let score_right = right.iter().take_while(|&t| t < current_tree).count();
        // let score_top = top.iter().rev().take_while(|&t| t < current_tree).count();
        // let score_bottom = bottom.iter().take_while(|&t| t < current_tree).count();

        // println!("horizontal scores: {}-{}", score_left, score_right);
        // println!("vertical scores: {}-{}", score_top, score_bottom);

        Some(score_left * score_right * score_top * score_bottom)
    }

    fn get_score_for_side(&self, tree_iter: impl Iterator<Item = Tree>, curr_tree: &Tree) -> usize {
        let mut score = 0;
        for tree in tree_iter {
            if tree >= *curr_tree {
                score += 1;
                break;
            }
            score += 1;
        }
        score
    }

    fn is_visible(grid: &TreeGrid, row: usize, col: usize) -> Option<bool> {
        let row_vec = grid.get(row)?.iter().collect::<Vec<_>>();
        let col_vec = TreeColIter::new(grid, col).collect::<Vec<_>>();
        let current_tree = row_vec.get(col)?;

        if row == row_vec.len() - 1 || row == 0 {
            return Some(true);
        }
        if col == col_vec.len() - 1 || col == 0 {
            return Some(true);
        }

        let left = &row_vec[..col];
        let right = &row_vec[col + 1..];

        let top = &col_vec[..row];
        let bottom = &col_vec[row + 1..];
        // println!("current tree: {}: {}-{}", current_tree, row, col);
        // println!("horizontal: {:?}-{:?}", left, right);
        // println!("vertical: {:?}-{:?}", top, bottom);

        let visible_left = Some(current_tree) > left.iter().max();
        let visible_right = Some(current_tree) > right.iter().max();
        let visible_top = Some(current_tree) > top.iter().max();
        let visible_bottom = Some(current_tree) > bottom.iter().max();

        if visible_left || visible_right || visible_top || visible_bottom {
            return Some(true);
        }

        // let (row_top, row_bottom) = row_vec.split_at(row);
        // println!("{:?}-{:?}", row_top, row_bottom);
        // let (col_left, col_right) = col_vec.split_at(col);
        // println!("{:?}-{:?}", col_left, col_right);
        None
    }
}

struct TreeColIter<'a> {
    trees: &'a TreeGrid,
    col: usize,
    current_row: usize,
}

impl TreeColIter<'_> {
    pub fn new(trees: &TreeGrid, col: usize) -> TreeColIter<'_> {
        TreeColIter {
            trees,
            col,
            current_row: 0,
        }
    }
}

impl<'a> Iterator for TreeColIter<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.trees.get(self.current_row).or(None)?;
        self.current_row += 1;
        row.get(self.col)
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

struct PrettyGrid<'a>(pub &'a TreeGrid);

impl std::fmt::Debug for PrettyGrid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        self.0.iter().for_each(|row| {
            writeln!(f, "{:?}", row).unwrap_or_default();
        });
        Ok(())
    }
}

pub struct PrettyVisibilityGrid<'a>(pub &'a VisibilityGrid);

impl std::fmt::Debug for PrettyVisibilityGrid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        self.0.iter().for_each(|row| {
            write!(f, "[ ").unwrap();
            row.iter().for_each(|b| {
                match b {
                    true => write!(f, "V "),
                    false => write!(f, "I "),
                }
                .unwrap();
            });
            write!(f, "]").unwrap();
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

pub struct PrettyScoreGrid<'a>(pub &'a ScoreGrid);

impl std::fmt::Debug for PrettyScoreGrid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        self.0.iter().for_each(|row| {
            writeln!(f, "{:?}", row).unwrap_or_default();
        });
        Ok(())
    }
}
