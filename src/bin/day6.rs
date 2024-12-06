pub use aoc_2024::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy)]
enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

struct DirectionSet(u8);
impl DirectionSet {
    fn new() -> DirectionSet {
        DirectionSet(0)
    }
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
    fn contains(&self, dir: Direction) -> bool {
        self.0 & dir as u8 != 0
    }
    fn insert(&mut self, dir: Direction) -> bool {
        let inserted = !self.contains(dir);
        self.0 |= dir as u8;
        inserted
    }
}

struct Tile {
    solid: bool,
    visited: DirectionSet,
}
struct Grid {
    tiles: Vec<Tile>,
    dim: usize,
}
#[derive(Clone)]
struct Guard {
    direction: Direction,
    position: (isize, isize),
}

impl Grid {
    fn idx(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.dim as isize {
            None
        } else {
            let idx = self.dim * y as usize + x as usize;
            if idx < self.tiles.len() {
                Some(idx)
            } else {
                None
            }
        }
    }
    fn get(&self, x: isize, y: isize) -> Option<&Tile> {
        Some(&self.tiles[self.idx(x, y)?])
    }
    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Tile> {
        let idx = self.idx(x, y)?;
        Some(&mut self.tiles[idx])
    }

    fn new(dim: usize) -> Grid {
        Grid {
            tiles: Vec::new(),
            dim,
        }
    }

    fn push(&mut self, row: &str) -> Option<Guard> {
        assert_eq!(row.len(), self.dim);
        let mut guard = None;
        for c in row.chars() {
            self.tiles.push(match c {
                '.' => Tile {
                    solid: false,
                    visited: DirectionSet::new(),
                },
                '#' => Tile {
                    solid: true,
                    visited: DirectionSet::new(),
                },
                c => {
                    let direction = match c {
                        '^' => Direction::North,
                        '>' => Direction::East,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        c => panic!("unexpected character '{c:?}'"),
                    };
                    assert!(guard.is_none(), "two guards");
                    guard = Some(Guard {
                        direction,
                        position: (
                            (self.tiles.len() % self.dim) as isize,
                            (self.tiles.len() / self.dim) as isize,
                        ),
                    });
                    Tile {
                        solid: false,
                        visited: DirectionSet::new(),
                    }
                }
            });
        }
        guard
    }
}

impl Guard {
    // Walks the grid, returning Ok if the guard leaves the maze or Err if he gets stuck in a loop.
    fn walk(&self, grid: &mut Grid) -> Result<usize, usize> {
        let mut guard = self.clone();

        grid.tiles
            .iter_mut()
            .for_each(|tile| tile.visited = DirectionSet::new());

        let mut unique = 0;
        let mut loop_length = 0;
        loop {
            let (x, y) = guard.position;
            let tile = grid.get_mut(x, y).unwrap();
            if tile.visited.is_empty() {
                unique += 1;
            }

            if tile.visited.insert(guard.direction) {
                loop_length += 1;
            } else {
                return Err(loop_length);
            }

            let (dx, dy) = match guard.direction {
                Direction::North => (0, -1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
                Direction::South => (0, 1),
            };

            match grid.get(x + dx, y + dy) {
                None => break,
                Some(Tile { solid: true, .. }) => {
                    guard.direction = match guard.direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    }
                }
                Some(Tile { solid: false, .. }) => {
                    guard.position = (x + dx, y + dy);
                }
            }
        }

        Result::Ok(unique)
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap).peekable();

    let mut guard = None;
    let mut grid = Grid::new(input.peek().unwrap().len());
    for row in input {
        if let Some(g) = grid.push(&row) {
            assert!(guard.is_none(), "two guards");
            guard = Some(g);
        }
    }

    let guard = guard.expect("no guard");
    if !c.part2 {
        println!("{}", guard.walk(&mut grid).expect("loop"));
    } else {
        let mut loops = 0;
        for i in 0..grid.tiles.len() {
            if grid.tiles[i].solid {
                continue;
            }

            grid.tiles[i].solid = true;
            if guard.walk(&mut grid).is_err() {
                loops += 1;
            }
            grid.tiles[i].solid = false;
        }
        println!("{loops}");
    }

    Ok(())
}
