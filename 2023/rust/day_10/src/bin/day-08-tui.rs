use std::{env, fs::read_to_string, path::PathBuf};

use anyhow::Result;
use pipe_maze::{execute_part, ArgumentError, Part};

fn main() -> Result<()> {
    let part = env::args()
        .nth(1)
        .ok_or(ArgumentError::NoArg)?
        .parse::<usize>()?;
    let path = env::args()
        .nth(2)
        .ok_or(ArgumentError::NoArg)?
        .parse::<PathBuf>()?;
    execute_part(&read_to_string(path)?, Part::try_from(part)?)?;
    Ok(())
}
