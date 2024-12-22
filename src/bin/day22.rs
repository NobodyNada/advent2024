pub use aoc_2024::prelude::*;

fn rng(mut seed: u64) -> u64 {
    seed = (seed ^ (seed << 6)) & 0xFFFFFF;
    seed = (seed ^ (seed >> 5)) & 0xFFFFFF;
    seed = (seed ^ (seed << 11)) & 0xFFFFFF;
    seed
}

fn main() -> Result<()> {
    let c = chal()?;

    let input = c.input.lines().map(Result::unwrap);

    let seeds = input.map(|line| line.parse::<u64>().unwrap());

    if c.part1 {
        println!(
            "{}",
            seeds
                .map(|seed| (0..2000).fold(seed, |seed, _| rng(seed)))
                .sum::<u64>()
        );
    } else {
        let prices = seeds.map(|seed| {
            (0..2000).scan(seed, |seed, _| {
                *seed = rng(*seed);
                Some(*seed % 10)
            })
        });

        let mut results = HashMap::<[i32; 4], u64>::new();
        for monkey in prices {
            let mut my_results = HashMap::<[i32; 4], u64>::new();
            for (a, b, c, d, e) in monkey.map(|i| i as i32).tuple_windows() {
                let changes = [b - a, c - b, d - c, e - d];
                my_results.entry(changes).or_insert(e as u64);
            }
            my_results
                .into_iter()
                .for_each(|(k, v)| *results.entry(k).or_default() += v);
        }
        println!("{:?}", results.iter().max_by_key(|(_, n)| *n).unwrap());
    }

    Ok(())
}
