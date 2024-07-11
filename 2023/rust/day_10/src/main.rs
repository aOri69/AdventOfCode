use std::{env, fs::read_to_string, path::PathBuf};

use anyhow::Result;
use pipe_maze::{execute_part, ArgumentError};

fn main() -> Result<()> {
    let path = env::args()
        .nth(1)
        .ok_or(ArgumentError::NoArg)?
        .parse::<PathBuf>()?;
    execute_part(&read_to_string(path)?)?;
    Ok(())
}
