pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    Ok(())
}
