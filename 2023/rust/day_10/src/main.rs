use anyhow::Result;
use pipe_maze::solve_parts;

fn main() -> Result<()> {
    solve_parts(include_str!("..\\input\\input.txt"))?;
    Ok(())
}
