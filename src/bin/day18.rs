use std::collections::BinaryHeap;

pub use aoc_2024::prelude::*;
use ndarray::Array2;

enum Tile {
    Empty,
    Wall,
    Visited,
}

fn main() -> Result<()> {
    let c = chal()?;

    let mut grid = Array2::from_shape_fn((71, 71), |_| Tile::Empty);

    let input = c.input.lines().map(Result::unwrap);

    'next_input: for (i, line) in input.enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

        grid[(x, y)] = Tile::Wall;

        grid.iter_mut().for_each(|t| {
            if matches!(t, Tile::Visited) {
                *t = Tile::Empty
            }
        });

        let goal = (70, 70);
        let mut unvisited = BinaryHeap::new();
        unvisited.push((0, (0, 0)));
        while let Some((cost, xy)) = unvisited.pop() {
            if matches!(grid[xy], Tile::Visited) {
                continue;
            }

            grid[xy] = Tile::Visited;
            for neighbor in Direction::all().iter().flat_map(|d| d.apply(xy)) {
                if neighbor == goal {
                    if i == 1024 {
                        println!("Part 1: {}", -cost);
                    }
                    continue 'next_input;
                } else if matches!(grid.get(neighbor), Some(Tile::Empty)) {
                    unvisited.push((cost - 1, neighbor));
                };
            }
        }

        // No path found.
        println!("Part 2: {},{}", x, y);
        break;
    }

    Ok(())
}
