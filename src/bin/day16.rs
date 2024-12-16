use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque, hash_map::Entry},
};

pub use aoc_2024::prelude::*;

enum Tile {
    Empty,
    Start,
    Wall,
    OnGoalPath,
    Goal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct State {
    xy: (usize, usize),
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Unvisited {
    cost: isize,
    state: State,
    predecessor: Option<State>,
}

struct Visited {
    cost: isize,
    predecessors: Vec<State>,
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
                b'.' => Tile::Empty,
                b'S' => Tile::Start,
                b'E' => Tile::Goal,
                _ => panic!("invalid input"),
            })
        })
        .collect_2d();

    let start = maze
        .indexed_iter()
        .filter(|(_, tile)| matches!(tile, Tile::Start))
        .map(|(xy, _)| xy)
        .next()
        .unwrap();

    let mut unvisited = BinaryHeap::new();
    let mut visited = HashMap::new();
    unvisited.push(Unvisited {
        cost: 0,
        state: State {
            xy: start,
            direction: Direction::East,
        },
        predecessor: None,
    });

    let mut goal_cost = None;
    while let Some(cur) = unvisited.pop() {
        let state = cur.state;

        match visited.entry(state) {
            Entry::Vacant(e) => {
                e.insert(Visited {
                    cost: cur.cost,
                    predecessors: cur.predecessor.into_iter().collect(),
                });
            }
            Entry::Occupied(mut e) => match e.get().cost.cmp(&cur.cost) {
                Ordering::Greater => continue, // The entry already there is better.
                Ordering::Equal => {
                    e.get_mut().predecessors.extend(cur.predecessor);
                    continue;
                }
                Ordering::Less => {
                    e.insert(Visited {
                        cost: cur.cost,
                        predecessors: cur.predecessor.into_iter().collect(),
                    });
                }
            },
        };

        let neighbors = [
            Some(Unvisited {
                state: State {
                    direction: state.direction.cw(1),
                    ..state
                },
                cost: cur.cost - 1000,
                predecessor: Some(state),
            }),
            Some(Unvisited {
                state: State {
                    direction: state.direction.ccw(1),
                    ..state
                },
                cost: cur.cost - 1000,
                predecessor: Some(state),
            }),
            {
                let xy = state.direction.apply(state.xy).unwrap();
                let cost = cur.cost - 1;
                match maze[xy] {
                    Tile::Wall => None,
                    Tile::Start => None,
                    Tile::Goal => {
                        if goal_cost.is_none_or(|c| c == -cost) {
                            goal_cost = Some(-cost);

                            // Mark all (transitive) predecessors as on the goal path.
                            let mut predecessors = VecDeque::from([state]);
                            while let Some(pred) = predecessors.pop_front() {
                                maze[pred.xy] = Tile::OnGoalPath;
                                predecessors.extend(&visited[&pred].predecessors);
                            }
                        }
                        None
                    }
                    Tile::Empty | Tile::OnGoalPath => Some(Unvisited {
                        cost,
                        state: State { xy, ..state },
                        predecessor: Some(state),
                    }),
                }
            },
        ]
        .into_iter()
        .flatten();

        unvisited.extend(neighbors);
    }

    println!("part 1: {}", goal_cost.expect("no path found"));
    println!(
        "part 2: {}",
        maze.iter()
            .filter(|tile| matches!(tile, Tile::OnGoalPath))
            .count()
            + 1 // the goal counts too!
    );

    Ok(())
}
