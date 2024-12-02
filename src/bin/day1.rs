#![feature(iter_next_chunk)]

pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c
        .input
        .lines()
        .map(|l| {
            let l = l?;
            let [a, b] = l
                .split_whitespace()
                .next_chunk()
                .map_err(|_| anyhow!("parse error"))?;
            Ok((a.parse::<u32>()?, b.parse::<u32>()?))
        })
        .map(Result::unwrap);

    let (mut a, mut b): (Vec<_>, Vec<_>) = input.unzip();
    a.sort();
    b.sort();

    let result: u32 = std::iter::zip(a.iter(), b.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();
    println!("part 1: {result}");

    let mut histogram = HashMap::<u32, u32>::new();
    for x in b {
        *histogram.entry(x).or_default() += 1;
    }
    let result: u32 = a
        .iter()
        .map(|x| x * histogram.get(x).copied().unwrap_or_default())
        .sum();
    println!("part 2: {result}");

    Ok(())
}
