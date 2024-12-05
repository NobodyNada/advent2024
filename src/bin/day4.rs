pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input: Vec<Vec<u8>> = c
        .input
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect();

    let mut result = 0;
    for (y, row) in input.iter().enumerate() {
        for x in 0..row.len() {
            if c.part2 {
                let s0 = [(-1, -1), (0, 0), (1, 1)];
                let s1 = [(-1, 1), (0, 0), (1, -1)];

                let mut r0 = s0;
                r0.reverse();
                let mut r1 = s1;
                r1.reverse();

                let seqs = [[s0, s1], [s0, r1], [r0, s1], [r0, r1]];

                let matches = seqs.into_iter().any(|seqs| {
                    seqs.into_iter().all(|seq| {
                        seq.into_iter().zip(b"MAS").all(|((dx, dy), c)| {
                            input
                                .get(y.wrapping_add_signed(dy))
                                .and_then(|row| row.get(x.wrapping_add_signed(dx)))
                                == Some(c)
                        })
                    })
                });
                if matches {
                    result += 1;
                }
            } else {
                let seqs: [[isize; 4]; 3] = [[0, 1, 2, 3], [0, -1, -2, -3], [0, 0, 0, 0]];
                // Take cartesian product
                let seqs = seqs
                    .iter()
                    .flat_map(|&xseq| seqs.iter().map(move |&yseq| xseq.into_iter().zip(yseq)));

                let matches = seqs.filter(|seq| {
                    seq.clone().zip(b"XMAS").all(|((dx, dy), c)| {
                        input
                            .get(y.wrapping_add_signed(dy))
                            .and_then(|row| row.get(x.wrapping_add_signed(dx)))
                            == Some(c)
                    })
                });
                result += matches.count();
            }
        }
    }

    println!("{result}");

    Ok(())
}
