pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap).next().unwrap();

    let stones = input.split_whitespace().map(|n| n.parse::<u64>().unwrap());
    let transform = |n: u64| match n {
        0 => vec![1u64],
        n if n.ilog10() % 2 == 1 => {
            let digits = n.ilog10() + 1;
            vec![n / 10u64.pow(digits / 2), n % 10u64.pow(digits / 2)]
        }
        n => vec![n * 2024],
    };

    let mut stones = stones.fold(HashMap::<u64, u64>::new(), |mut stones, stone| {
        *stones.entry(stone).or_insert(0) += 1;
        stones
    });

    for _ in 0..(if c.part1 { 25 } else { 75 }) {
        stones =
            stones
                .into_iter()
                .fold(HashMap::<u64, u64>::new(), |mut stones, (stone, count)| {
                    for stone in transform(stone) {
                        *stones.entry(stone).or_insert(0) += count;
                    }
                    stones
                });
    }

    println!("{}", stones.into_values().sum::<u64>());
    Ok(())
}
