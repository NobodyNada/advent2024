use std::{cell::RefCell, rc::Rc};

pub use aoc_2024::prelude::*;
use ndarray::ArrayViewMut2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Box(bool),
}

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap);

    let robot = Rc::new(RefCell::new(None));

    let mut grid = if c.part1 {
        input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                let robot = robot.clone();
                line.into_bytes()
                    .into_iter()
                    .enumerate()
                    .map(move |(x, b)| match b {
                        b'.' => Tile::Empty,
                        b'#' => Tile::Wall,
                        b'O' => Tile::Box(false),
                        b'@' => {
                            robot.replace(Some((x, y)));
                            Tile::Empty
                        }
                        _ => panic!("invalid character"),
                    })
            })
            .collect_2d()
    } else {
        input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                let robot = robot.clone();
                line.into_bytes()
                    .into_iter()
                    .enumerate()
                    .flat_map(move |(x, b)| match b {
                        b'.' => [Tile::Empty, Tile::Empty],
                        b'#' => [Tile::Wall, Tile::Wall],
                        b'O' => [Tile::Box(false), Tile::Box(true)],
                        b'@' => {
                            robot.replace(Some((x * 2, y)));
                            [Tile::Empty, Tile::Empty]
                        }
                        _ => panic!("invalid character"),
                    })
            })
            .collect_2d()
    };
    //dbg!(&grid);

    let boxes_before = grid
        .iter()
        .filter(|&tile| matches!(tile, Tile::Box(_)))
        .count();

    let mut robot = robot.borrow().unwrap();

    'fail: for b in input.flat_map(|line| line.into_bytes()) {
        let direction = match b {
            b'^' => Direction::North,
            b'v' => Direction::South,
            b'>' => Direction::East,
            b'<' => Direction::West,
            _ => panic!("invalid character"),
        };
        if c.part1 {
            // First, see if we can move
            let target_pos = direction.apply(robot).unwrap();
            let mut current_pos = target_pos;
            loop {
                match grid[current_pos] {
                    Tile::Empty => break,
                    Tile::Wall => continue 'fail,
                    Tile::Box(_) => current_pos = direction.apply(current_pos).unwrap(),
                }
            }

            grid[current_pos] = Tile::Box(false);
            grid[target_pos] = Tile::Empty;
            robot = target_pos;
        } else {
            fn move_boxes(
                mut grid: ArrayViewMut2<Tile>,
                pos: (usize, usize),
                dir: Direction,
                is_paired_move: bool,
                commit: bool,
            ) -> bool {
                match grid[pos] {
                    Tile::Empty => true,
                    Tile::Wall => false,
                    Tile::Box(r) => {
                        let new_pos = dir.apply(pos).unwrap();
                        let success = if dir.is_horizontal() || is_paired_move {
                            move_boxes(grid.view_mut(), new_pos, dir, false, commit)
                        } else {
                            move_boxes(grid.view_mut(), new_pos, dir, false, commit)
                                && move_boxes(
                                    grid.view_mut(),
                                    if r { Direction::West } else { Direction::East }
                                        .apply(pos)
                                        .unwrap(),
                                    dir,
                                    true,
                                    commit,
                                )
                        };
                        if commit {
                            assert!(success);
                            //println!("move {pos:?} {dir:?}");
                            grid[new_pos] = grid[pos];
                            grid[pos] = Tile::Empty;
                        }
                        success
                    }
                }
            }

            let target_pos = direction.apply(robot).unwrap();
            if move_boxes(grid.view_mut(), target_pos, direction, false, false) {
                move_boxes(grid.view_mut(), target_pos, direction, false, true);
                robot = target_pos;
            }
        }
    }

    let result: usize = grid
        .indexed_iter()
        .filter_map(|((x, y), tile)| {
            if let Tile::Box(false) = tile {
                Some(y * 100 + x)
            } else {
                None
            }
        })
        .sum();
    let boxes_after = grid
        .iter()
        .filter(|&tile| matches!(tile, Tile::Box(_)))
        .count();
    assert_eq!(boxes_before, boxes_after);
    println!("{result}");

    Ok(())
}
