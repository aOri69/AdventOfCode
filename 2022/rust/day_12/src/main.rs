#![allow(unused)]

use std::{
    collections::{hash_map, HashMap, HashSet, VecDeque},
    convert::Infallible,
    ops::Add,
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Node {
    Start,
    End,
    Path(u8),
}

impl Node {
    fn elevation(self) -> u8 {
        match self {
            Node::Start => 0,
            Node::End => 25,
            Node::Path(h) => h,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.row).field(&self.col).finish()
    }
}

#[derive(Clone)]
struct Dimension {
    height: usize,
    width: usize,
}

#[derive(Clone)]
struct Grid {
    start: Coord,
    end: Coord,
    grid: Vec<Vec<Node>>,
    dim: Dimension,
}
// Logic impls
impl Grid {
    fn shortest_path(&self, alg: Algorithm) -> Option<usize> {
        match alg {
            Algorithm::Dfs => self.dfs(),
            Algorithm::Bfs => self.bfs(),
        }
    }

    fn dfs(&self) -> Option<usize> {
        None
    }

    fn bfs(&self) -> Option<usize> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut dst = vec![vec![0_usize; self.grid.first().unwrap().len()]; self.grid.len()];

        // initialization
        queue.push_back(self.start);
        let mut prev = None;

        while !queue.is_empty() {
            // safe because of while condition
            let current = queue.pop_front().unwrap();
            if let Coord { row: 25, col: 113 } = current {
                println!();
            }
            // set to "visited"
            visited.entry(current).or_insert(prev); //insert(current, prev);

            // break&return condition
            if current == self.end {
                return Some(dst[self.end.row][self.end.col]);
            }
            // all walkable neighbours
            for neighbour in self.walkable_neighbours(current) {
                // print!("{neighbour:?} ");
                // that have not yet been visited
                if let hash_map::Entry::Vacant(e) = visited.entry(neighbour) {
                    // add to queue and set as "visited"
                    queue.push_back(neighbour);
                    e.insert(Some(current));
                    dst[neighbour.row][neighbour.col] = dst[current.row][current.col] + 1;
                }
            }
            prev = Some(current);
        }
        None
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        self.dim.height > coord.row && self.dim.width > coord.col
    }

    fn walkable_neighbours(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let current_heigth = self
            .grid
            .get(coord.row)
            .unwrap() // should be just fine
            .get(coord.col)
            .unwrap() // should be just fine
            .elevation();
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .map(move |(dx, dy)| {
                Some(Coord {
                    row: coord.row.checked_add_signed(dx)?,
                    col: coord.col.checked_add_signed(dy)?,
                })
            })
            .into_iter()
            .flatten()
            .filter(move |c| {
                self.node(c)
                    .is_some_and(|n| n.elevation().abs_diff(current_heigth) <= 1)
            })
    }

    fn node(&self, coord: &Coord) -> Option<&Node> {
        self.grid.get(coord.row)?.get(coord.col)
    }
}
// Parsing impls
impl Grid {
    fn get_grid_match_coord(grid: &[Vec<Node>], node_to_find: Node) -> Coord {
        let width = grid.first().unwrap().len();
        let flatten_coord = match node_to_find {
            Node::Start => Grid::get_flatten_start(grid),
            Node::End => Grid::get_flatten_end(grid),
            Node::Path(_) => panic!("method supposed to be used only with Start/End"),
        };
        Coord {
            row: flatten_coord / width,
            col: flatten_coord - (flatten_coord / width) * width,
        }
    }

    fn get_flatten_start(grid: &[Vec<Node>]) -> usize {
        grid.iter()
            .flatten()
            .position(|node| matches!(node, Node::Start))
            .unwrap_or_default()
    }
    fn get_flatten_end(grid: &[Vec<Node>]) -> usize {
        grid.iter()
            .flatten()
            .position(|node| matches!(node, Node::End))
            .unwrap_or_default()
    }
}
// Trait impls
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)?;
        writeln!(f)?;
        for row in self.grid.iter() {
            for col in row.iter() {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Grid {
    type Err = Infallible; //Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // dbg!(b'a'); // 97
        let grid = s
            .lines()
            .map(|s| s.chars().map(Node::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = match grid.first() {
            Some(first_row) => first_row.len(),
            None => 0,
        };

        let start_pos = Grid::get_grid_match_coord(&grid, Node::Start);
        let end_pos = Grid::get_grid_match_coord(&grid, Node::End);
        Ok(Self {
            start: start_pos,
            end: end_pos,
            grid,
            dim: Dimension { height, width },
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = Grid::from_str(include_str!("../input.txt"))?;
    println!(
        "Shortest path using BFS alg: {:?}",
        grid.shortest_path(Algorithm::Bfs)
    );
    println!(
        "Shortest path using DFS alg: {:?}",
        grid.shortest_path(Algorithm::Dfs)
    );
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    fn test_grid() -> Grid {
        INPUT.lines().for_each(|l| {
            println!();
            l.chars().for_each(|c| print!("{c:^3}"));
        });
        println!();
        let grid = Grid::from_str(INPUT).unwrap();
        dbg!(&grid);
        grid
    }

    #[test]
    fn small_input_bfs() {
        assert_eq!(test_grid().shortest_path(Algorithm::Bfs), Some(31));
    }

    #[test]
    fn small_input_dfs() {
        assert_eq!(test_grid().shortest_path(Algorithm::Dfs), Some(31));
    }
}
