pub use aoc_2024::prelude::*;
use ndarray::{ArcArray2, prelude::*};

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let topo = input
        .map(|line| {
            line.into_bytes().into_iter().map(|c| match c {
                b'0'..=b'9' => c - b'0',
                _ => panic!(),
            })
        })
        .collect_2d();

    let mut reachable = topo.map(|_| 0);
    for (xy, &elevation) in topo.indexed_iter() {
        if elevation == 9 {
            // This is a viable endpoint for a trail
            fn search(
                xy: (usize, usize),
                topo: &Array2<u8>,
                reachable: &mut Array2<u32>,
                visited: &mut ArcArray2<bool>,
                part2: bool,
            ) -> bool {
                let elevation = topo[xy];
                if elevation == 0 {
                    reachable[xy] += 1;
                    return true;
                }

                let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                let mut any = false;
                for (dx, dy) in dirs {
                    let (nx, ny) = (xy.0 as isize - dx, xy.1 as isize - dy);
                    if nx < 0
                        || ny < 0
                        || nx as usize >= topo.ncols()
                        || ny as usize >= topo.nrows()
                    {
                        continue;
                    }
                    let nxy = (nx as usize, ny as usize);
                    if visited[nxy] {
                        continue;
                    }
                    if topo[nxy] == elevation - 1 {
                        if part2 {
                            let mut visited = visited.clone();
                            any |= search(nxy, topo, reachable, &mut visited, part2);
                        } else {
                            any |= search(nxy, topo, reachable, visited, part2);
                        }
                        visited[nxy] = true;
                    }
                }
                if any {
                    reachable[xy] += 1;
                }
                any
            }
            search(
                xy,
                &topo,
                &mut reachable,
                &mut topo.map(|_| false).into_shared(),
                c.part2,
            );
        }
    }

    let result = azip!(&reachable, &topo).fold(0, |count, &reachable, &elevation| {
        if elevation == 0 {
            count + reachable
        } else {
            count
        }
    });
    println!("{result}");

    Ok(())
}
