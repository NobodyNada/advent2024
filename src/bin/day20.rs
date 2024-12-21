use std::collections::VecDeque;

pub use aoc_2024::prelude::*;

#[derive(Copy, Clone)]
enum Tile {
    Empty(Option<u16>, Option<u16>),
    Wall,
}

fn main() -> Result<()> {
    let c = chal()?;

    let mut maze = c
        .input
        .lines()
        .map(Result::unwrap)
        .map(|line| line.into_bytes().into_iter())
        .map(|line| {
            line.map(|b| match b {
                b'#' => Tile::Wall,
                b'.' => Tile::Empty(None, None),
                b'S' => Tile::Empty(Some(0), None),
                b'E' => Tile::Empty(None, Some(0)),
                _ => panic!("invalid input"),
            })
        })
        .collect_2d();

    let start = maze
        .indexed_iter()
        .filter(|(_, tile)| matches!(tile, Tile::Empty(Some(0), None)))
        .map(|(xy, _)| xy)
        .next()
        .unwrap();
    let end = maze
        .indexed_iter()
        .filter(|(_, tile)| matches!(tile, Tile::Empty(None, Some(0))))
        .map(|(xy, _)| xy)
        .next()
        .unwrap();

    // First, determine the distance from each point to the start & end
    let mut unvisited = VecDeque::new();

    unvisited.push_back(start);
    while let Some(xy) = unvisited.pop_front() {
        let Some(Tile::Empty(Some(cost), _)) = maze.get(xy) else {
            unreachable!();
        };

        let cost = cost + 1;
        for xy in Direction::all().into_iter().flat_map(|d| d.apply(xy)) {
            if let Some(Tile::Empty(c @ None, _)) = maze.get_mut(xy) {
                *c = Some(cost);
                unvisited.push_back(xy);
            }
        }
    }

    unvisited.push_back(end);
    while let Some(xy) = unvisited.pop_front() {
        let Some(Tile::Empty(_, Some(cost))) = maze.get(xy) else {
            unreachable!();
        };

        let cost = cost + 1;
        for xy in Direction::all().into_iter().flat_map(|d| d.apply(xy)) {
            if let Some(Tile::Empty(_, c @ None)) = maze.get_mut(xy) {
                *c = Some(cost);
                unvisited.push_back(xy);
            }
        }
    }

    let Tile::Empty(_, Some(best_path)) = maze[start] else {
        panic!("Maze is not navigable")
    };
    let cheat_length: isize = if c.part1 { 2 } else { 20 };
    let mut num_cheats = 0;

    // Now find every opportunity to cheat!
    for (y, x) in (0..maze.nrows()).cartesian_product(0..maze.ncols()) {
        for (dy, dx) in
            (-cheat_length..=cheat_length).cartesian_product(-cheat_length..=cheat_length)
        {
            let distance_used = dx.abs() + dy.abs();
            if distance_used > cheat_length {
                continue;
            }
            let (Some(ex), Some(ey)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) else {
                continue;
            };

            let start = (x, y);
            let end = (ex, ey);

            if let (Some(Tile::Empty(Some(d1), _)), Some(Tile::Empty(_, Some(d2)))) =
                (maze.get(start), maze.get(end))
            {
                let d = d1 + d2 + distance_used as u16;
                if d <= best_path.saturating_sub(100) {
                    num_cheats += 1;
                }
            }
        }
    }
    println!("{num_cheats}");

    Ok(())
}
