#![feature(iter_map_windows)]
pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);
    let input = input.map(|line| {
        line.split_whitespace()
            .map(|w| w.parse::<u32>().expect("parse error"))
            .collect::<Vec<_>>()
    });
    let safe = input.filter(|input| {
        let mut skippable = if c.part2 {
            0..=input.len()
        } else {
            input.len()..=input.len() // invalid index
        };

        skippable.any(|i| {
            let input = input
                .iter()
                .enumerate()
                .filter_map(|(j, x)| if j == i { None } else { Some(x) });

            (input.clone().is_sorted() || input.clone().rev().is_sorted())
                && input
                    .map_windows(|&x| x)
                    .all(|[&a, &b]| (1..=3).contains(&a.abs_diff(b)))
        })
    });

    println!("{}", safe.count());

    Ok(())
}
