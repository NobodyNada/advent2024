pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let mut frequencies = HashMap::<u8, Vec<(isize, isize)>>::new();

    let mut width = 0;
    let mut height = 0;
    for (y, row) in input.enumerate() {
        width = row.len() as isize;
        height += 1;
        for (x, c) in row.bytes().enumerate() {
            if c != b'.' {
                frequencies
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let antinode = |x, y| {
        frequencies.values().any(|antennas| {
            let mut pairs = antennas.iter().tuple_combinations();
            pairs.any(|(&a1, &a2)| {
                let d1x = a1.0 - x;
                let d1y = a1.1 - y;
                let d2x = a2.0 - x;
                let d2y = a2.1 - y;

                a1 != a2
                    && if c.part1 {
                        a1 != a2 && d1x == d2x * 2 && d1y == d2y * 2
                    } else {
                        let slope1 = d1x as f32 / d1y as f32;
                        let slope2 = d2x as f32 / d2y as f32;

                        (x, y) == a1 || (x, y) == a2 || slope1 == slope2
                    }
            })
        })
    };

    println!(
        "{}",
        (0..height)
            .cartesian_product(0..width)
            .filter(|&(y, x)| { antinode(x, y) })
            .count()
    );

    // Print the grid for debugging
    for y in 0..height {
        for x in 0..width {
            if let Some((c, _)) = frequencies
                .iter()
                .find(|(_, antennas)| antennas.iter().any(|&a| (x, y) == a))
            {
                print!("{}", *c as char);
            } else if antinode(x, y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(())
}
