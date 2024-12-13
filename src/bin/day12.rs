pub use aoc_2024::prelude::*;
use ndarray::ArrayViewMut2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Done,
    Visited { borders: [bool; 4] },
    Unvisited(u8),
}

fn build_region(mut grid: ArrayViewMut2<Cell>, id: u8, xy: (usize, usize)) -> Option<usize> {
    match grid.view().get(xy).copied() {
        Some(Cell::Unvisited(i)) if i != id => None,
        None | Some(Cell::Done) => None,
        Some(Cell::Visited { .. }) => Some(0),
        Some(Cell::Unvisited(_)) => {
            *grid.get_mut(xy).unwrap() = Cell::Visited {
                borders: [false; 4],
            };
            let neighbors = Direction::all().into_iter().map(|dir| dir.apply(xy));
            let mut area = 1;
            let mut borders = [false; 4];
            for (neighbor, border) in neighbors.zip(borders.iter_mut()) {
                if let Some(neighbor) =
                    neighbor.and_then(|xy| build_region(grid.view_mut(), id, xy))
                {
                    area += neighbor;
                } else {
                    *border = true;
                }
            }
            *grid.get_mut(xy).unwrap() = Cell::Visited { borders };
            Some(area)
        }
    }
}

fn prune_borders(mut grid: ArrayViewMut2<Cell>, xy: (usize, usize), direction: Direction) {
    let Some(Cell::Visited { borders }) = grid.get_mut(xy) else {
        return;
    };
    if borders[direction as usize] {
        borders[direction as usize] = false;
        for xy in direction
            .perpendicular()
            .into_iter()
            .filter_map(|d| d.apply(xy))
        {
            prune_borders(grid.view_mut(), xy, direction)
        }
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c
        .input
        .lines()
        .map(Result::unwrap)
        .map(|s| s.into_bytes().into_iter().map(Cell::Unvisited))
        .collect_2d();

    let mut price = 0;
    for xy in (0..input.ncols()).cartesian_product(0..input.nrows()) {
        if let Cell::Unvisited(id) = input[xy] {
            let area = build_region(input.view_mut(), id, xy).unwrap();

            let mut perimeter = 0;
            for xy in (0..input.ncols()).cartesian_product(0..input.nrows()) {
                if let Cell::Visited { borders } = input[xy] {
                    for direction in Direction::all() {
                        if borders[direction as usize] {
                            perimeter += 1;
                        }
                        if c.part2 {
                            prune_borders(input.view_mut(), xy, direction);
                        }
                    }
                    input[xy] = Cell::Done;
                }
            }

            price += area * perimeter;
        }
    }

    println!("{price}");

    Ok(())
}
