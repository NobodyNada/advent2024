pub use aoc_2024::prelude::*;

#[derive(Debug)]
struct Lock([u8; 5]);

#[derive(Debug)]
struct Key([u8; 5]);

impl Key {
    fn matches(&self, lock: &Lock) -> bool {
        self.0.iter().zip(lock.0.iter()).all(|(&a, &b)| a <= 5 - b)
    }
}

enum Input {
    Lock(Lock),
    Key(Key),
}

fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let key = input.next().unwrap() == ".....";
    let mut result = [0, 0, 0, 0, 0];
    for line in input {
        for (j, c) in line.bytes().enumerate() {
            result[j] += (c == b'#') as u8;
        }
    }
    if key {
        // keys have the extra row on the bottom
        Input::Key(Key(result.map(|n| n - 1)))
    } else {
        Input::Lock(Lock(result))
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for input in input
        .chunk_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| parse(group))
    {
        match input {
            Input::Lock(lock) => locks.push(lock),
            Input::Key(key) => keys.push(key),
        }
    }

    println!(
        "{}",
        locks
            .iter()
            .cartesian_product(keys.iter())
            .filter(|(lock, key)| key.matches(lock))
            .count()
    );

    Ok(())
}
