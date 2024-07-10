use thiserror::Error;

pub type Surface = Vec<Vec<SurfaceType>>;

pub const HORIZONTAL: &str = "─";
pub const VERTICAL: &str = "│";
pub const DOWN_RIGHT: &str = "┌";
pub const DOWN_LEFT: &str = "┐";
pub const UP_RIGHT: &str = "└";
pub const UP_LEFT: &str = "┘";

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Coords {
    pub row: usize,
    pub col: usize,
}

impl std::fmt::Debug for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl std::ops::Add for Coords {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match self.row.checked_add(rhs.row) {
            Some(row_coord) => self.col.checked_add(rhs.col).map(|col_coord| Coords {
                row: row_coord,
                col: col_coord,
            }),
            None => None,
        }
    }
}

impl std::ops::Sub for Coords {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.row.checked_sub(rhs.row) {
            Some(row_coord) => self.col.checked_sub(rhs.col).map(|col_coord| Coords {
                row: row_coord,
                col: col_coord,
            }),
            None => None,
        }
    }
}

pub fn build_surface(input: &str) -> Result<(Coords, Surface), SurfaceError> {
    let mut starting_coords: Option<(usize, usize)> = None;

    let surface = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| {
                    let surface = SurfaceType::try_from(c);

                    if let Ok(SurfaceType::StartingPositon) = surface {
                        starting_coords = Some((row, col));
                    }

                    surface
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    match starting_coords {
        Some((row, col)) => Ok((Coords { row, col }, surface)),
        None => Err(SurfaceError::StartingPosNotFound),
    }
}

/// Pipe related errors
#[derive(Debug, Error)]
pub enum PipeError {
    #[error("Wrong argument given: {0}. 1 or 2 are possible entries")]
    WrongPipeType(char),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pipe(char);

impl TryFrom<char> for Pipe {
    type Error = PipeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' | '-' | 'L' | 'J' | '7' | 'F' => Ok(Self(value)),
            c => Err(PipeError::WrongPipeType(c)),
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            '|' => write!(f, "{}", VERTICAL),
            '-' => write!(f, "{}", HORIZONTAL),
            'L' => write!(f, "{}", UP_RIGHT),
            'J' => write!(f, "{}", UP_LEFT),
            '7' => write!(f, "{}", DOWN_LEFT),
            'F' => write!(f, "{}", DOWN_RIGHT),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SurfaceType {
    Pipe(Pipe),
    Ground,
    StartingPositon,
}

impl SurfaceType {
    pub fn is_pipe(&self) -> bool {
        match self {
            SurfaceType::Pipe(_) => true,
            SurfaceType::Ground => false,
            SurfaceType::StartingPositon => false,
        }
    }
}

/// Surface related errors
#[derive(Debug, Error)]
pub enum SurfaceError {
    // #[error("Wrong argument given: {0}. 1 or 2 are possible entries")]
    // WrongSurfaceType(char),
    #[error(transparent)]
    WrongPipeType(#[from] PipeError),
    #[error("Starting position not found")]
    StartingPosNotFound,
}

impl TryFrom<char> for SurfaceType {
    type Error = SurfaceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::StartingPositon),
            probably_pipe => Ok(Self::Pipe(Pipe::try_from(probably_pipe)?)),
        }
    }
}

impl std::fmt::Display for SurfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SurfaceType::Pipe(pipe) => write!(f, "{}", pipe),
            SurfaceType::Ground => write!(f, "."),
            SurfaceType::StartingPositon => write!(f, "S"),
        }
    }
}

pub fn get_directions_for_pipe(surface: &Surface, position: Coords) -> Vec<Coords> {
    use SurfaceType::*;

    let max_row = surface.len();
    let max_col = surface.first().unwrap().len();

    let lower = position + Coords { row: 1, col: 0 };
    let upper = position - Coords { row: 1, col: 0 };
    let right = position + Coords { row: 0, col: 1 };
    let left = position - Coords { row: 0, col: 1 };

    let result = match &surface[position.row][position.col] {
        Pipe(pipe) => match pipe.to_string().as_str() {
            HORIZONTAL => {
                vec![left, right]
            }
            VERTICAL => vec![upper, lower],
            DOWN_RIGHT => vec![right, lower],
            DOWN_LEFT => vec![lower, left],
            UP_RIGHT => vec![upper, right],
            UP_LEFT => vec![left, upper],
            _ => vec![],
        },
        Ground => vec![],
        StartingPositon => {
            let mut result: Vec<_> = Vec::new();

            match surface[left.unwrap().row][left.unwrap().col]
                .to_string()
                .as_str()
            {
                UP_LEFT | DOWN_LEFT | HORIZONTAL => result.push(left),
                _ => (),
            };
            match surface[upper.unwrap().row][upper.unwrap().col]
                .to_string()
                .as_str()
            {
                VERTICAL | DOWN_RIGHT | DOWN_LEFT => result.push(upper),
                _ => (),
            }
            match surface[right.unwrap().row][right.unwrap().col]
                .to_string()
                .as_str()
            {
                UP_LEFT | DOWN_LEFT | HORIZONTAL => result.push(right),
                _ => (),
            }
            match surface[lower.unwrap().row][lower.unwrap().col]
                .to_string()
                .as_str()
            {
                VERTICAL | UP_RIGHT | UP_LEFT => result.push(lower),
                _ => (),
            }
            if surface[lower.unwrap().row][lower.unwrap().col].is_pipe() {
                result.push(left);
            }
            result
        }
    };

    result
        .into_iter()
        .filter(|c| c.is_some_and(|c| c.row <= max_row && c.col <= max_col))
        .filter(|c| {
            let c = c.unwrap();
            surface[c.row][c.col].is_pipe()
        })
        .map(|c| c.unwrap())
        .collect()
}
