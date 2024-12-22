use std::{cell::RefCell, iter::once, sync::LazyLock};

pub use aoc_2024::prelude::*;
use ndarray::{Array2, array};

struct Keypad<T> {
    buttons: Array2<Button<T>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Button<T> {
    Empty,
    Input(T),
    Accept,
}

impl<T: Eq> Keypad<T> {
    fn find(&self, button: &Button<T>) -> Option<(usize, usize)> {
        self.buttons
            .indexed_iter()
            .find(|(_, b)| *b == button)
            .map(|(xy, _)| xy)
    }
}

#[allow(unused)]
fn print_dirpresses(dirpresses: impl Iterator<Item = Button<Direction>>) {
    let mut dirpresses = dirpresses.map(|b| match b {
        Button::Empty => '.',
        Button::Input(Direction::North) => '^',
        Button::Input(Direction::South) => 'v',
        Button::Input(Direction::East) => '>',
        Button::Input(Direction::West) => '<',
        Button::Accept => 'A',
    });
    println!("{}", dirpresses.join(""));
}

#[derive(Clone)]
struct DirectionIterator {
    first: Option<(Direction, usize)>,
    second: Option<(Direction, usize)>,
}
impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((dir, n)) = &mut self.first {
            let dir = *dir;
            *n -= 1;
            if *n == 0 {
                self.first = None;
            }
            Some(dir)
        } else if let Some((dir, n)) = &mut self.second {
            let dir = *dir;
            *n -= 1;
            if *n == 0 {
                self.second = None;
            }
            Some(dir)
        } else {
            None
        }
    }
}

fn navigate<T: Eq>(
    from: &Button<T>,
    to: &Button<T>,
    keypad: &Keypad<T>,
) -> impl Iterator<Item = DirectionIterator> {
    let from = keypad.find(from).unwrap();
    let to = keypad.find(to).unwrap();
    let dx = to.0 as isize - from.0 as isize;
    let dy = to.1 as isize - from.1 as isize;

    let dir_x = match dx.cmp(&0) {
        std::cmp::Ordering::Less => Some(Direction::West),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Direction::East),
    };
    let dir_y = match dy.cmp(&0) {
        std::cmp::Ordering::Less => Some(Direction::North),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Direction::South),
    };

    match (dir_x, dir_y) {
        (None, None) => [
            Some(DirectionIterator {
                first: None,
                second: None,
            }),
            None,
        ],
        (Some(dir), None) => [
            Some(DirectionIterator {
                first: Some((dir, dx.unsigned_abs())),
                second: None,
            }),
            None,
        ],
        (None, Some(dir)) => [
            Some(DirectionIterator {
                first: Some((dir, dy.unsigned_abs())),
                second: None,
            }),
            None,
        ],
        (Some(dir_x), Some(dir_y)) => {
            let dirs = [
                DirectionIterator {
                    first: Some((dir_x, dx.unsigned_abs())),
                    second: Some((dir_y, dy.unsigned_abs())),
                },
                DirectionIterator {
                    second: Some((dir_x, dx.unsigned_abs())),
                    first: Some((dir_y, dy.unsigned_abs())),
                },
            ];

            dirs.map(|dirs @ DirectionIterator { first, .. }| {
                let (dir, n) = first.unwrap();
                let mut xy = from;
                for _ in 0..n {
                    xy = dir.apply(xy).unwrap();
                    if matches!(keypad.buttons[xy], Button::Empty) {
                        return None;
                    }
                }
                Some(dirs)
            })
        }
    }
    .into_iter()
    .flatten()
}

static NUMPAD: LazyLock<Keypad<u8>> = std::sync::LazyLock::new(|| {
    let mut numpad = array![
        [
            Button::Input(b'7'),
            Button::Input(b'8'),
            Button::Input(b'9')
        ],
        [
            Button::Input(b'4'),
            Button::Input(b'5'),
            Button::Input(b'6')
        ],
        [
            Button::Input(b'1'),
            Button::Input(b'2'),
            Button::Input(b'3')
        ],
        [Button::Empty, Button::Input(b'0'), Button::Accept],
    ];
    numpad.swap_axes(0, 1);
    Keypad { buttons: numpad }
});

static DIRPAD: LazyLock<Keypad<Direction>> = std::sync::LazyLock::new(|| {
    let mut dirpad = array![
        [
            Button::Empty,
            Button::Input(Direction::North),
            Button::Accept
        ],
        [
            Button::Input(Direction::West),
            Button::Input(Direction::South),
            Button::Input(Direction::East)
        ],
    ];
    dirpad.swap_axes(0, 1);
    Keypad { buttons: dirpad }
});

fn solve_depth<T: Eq>(from: &Button<T>, to: &Button<T>, keypad: &Keypad<T>, depth: usize) -> u64 {
    let seqs = navigate(from, to, keypad);
    seqs.map(|seq| {
        let seq = once(Button::Accept)
            .chain(seq.map(Button::Input))
            .chain(once(Button::Accept));

        if depth == 0 {
            seq.count() as u64 - 1
        } else {
            #[allow(unused)] // false-positive
            type Key = (Button<Direction>, Button<Direction>, usize);
            thread_local! {
                static MEMO: RefCell<HashMap<Key, u64>> = RefCell::new(HashMap::new());
            }
            seq.tuple_windows()
                .map(|(from, to)| {
                    if let Some(count) =
                        MEMO.with_borrow(|memo| memo.get(&(from, to, depth)).copied())
                    {
                        count
                    } else {
                        let count = solve_depth(&from, &to, &DIRPAD, depth - 1);
                        MEMO.with_borrow_mut(|memo| memo.insert((from, to, depth), count));
                        count
                    }
                })
                .sum()
        }
    })
    .min()
    .unwrap()
}

fn solve(input: &str, robots: usize) -> u64 {
    let input = input.strip_suffix('A').unwrap();

    let numpresses = input.bytes().map(Button::Input).chain(once(Button::Accept));
    let dirpresses = once(Button::Accept)
        .chain(numpresses)
        .tuple_windows()
        .map(|(from, to)| solve_depth(&from, &to, &NUMPAD, robots))
        .sum::<u64>();

    dirpresses * input.parse::<u64>().unwrap()
}

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let robots = if c.part1 { 2 } else { 25 };
    println!("{}", input.map(|line| solve(&line, robots)).sum::<u64>());

    Ok(())
}
