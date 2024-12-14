use std::collections::{HashSet, hash_map::Entry};

pub use aoc_2024::prelude::*;
use flate2::{
    Compression,
    bufread::{GzDecoder, GzEncoder},
};
use regex::Regex;

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
//const WIDTH: i32 = 11;
//const HEIGHT: i32 = 7;

impl Robot {
    fn step(&mut self, n: i32) {
        self.p.0 = (self.p.0 + self.v.0 * n).rem_euclid(WIDTH);
        self.p.1 = (self.p.1 + self.v.1 * n).rem_euclid(HEIGHT);
    }

    #[expect(non_contiguous_range_endpoints)]
    fn quadrant(&self) -> Option<usize> {
        const L: i32 = WIDTH / 2;
        const R: i32 = WIDTH / 2 + 1;
        const T: i32 = HEIGHT / 2;
        const B: i32 = HEIGHT / 2 + 1;
        match (self.p.0, self.p.1) {
            (..L, ..T) => Some(0),
            (R.., ..T) => Some(1),
            (..L, B..) => Some(2),
            (R.., B..) => Some(3),
            _ => None,
        }
    }
}

fn stringify_grid(robots: &[Robot]) -> String {
    let mut s = String::new();
    let positions = robots.iter().map(|r| r.p).collect::<HashSet<_>>();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
    }
    s
}

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let regex = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$")?;
    let mut robots = input
        .map(|line| {
            let (_, [px, py, vx, vy]) = regex.captures(&line).unwrap().extract();
            Ok(Robot {
                p: (px.parse()?, py.parse()?),
                v: (vx.parse()?, vy.parse()?),
            })
        })
        .map(Result::unwrap)
        .collect_vec();

    if c.part1 {
        let mut quadrants = [0; 4];
        robots.into_iter().for_each(|mut r| {
            r.step(100);
            if let Some(q) = r.quadrant() {
                quadrants[q] += 1
            }
        });
        println!("{}", quadrants.into_iter().product::<i32>());
    } else {
        let mut grids: HashMap<Vec<u8>, i32> = HashMap::new();

        for time in 1.. {
            for robot in robots.iter_mut() {
                robot.step(1);
            }

            println!("{time}");
            let grid = stringify_grid(&robots);
            let mut encoder = GzEncoder::new(grid.as_bytes(), Compression::fast());
            let mut grid = Vec::new();
            encoder.read_to_end(&mut grid)?;

            if let Entry::Vacant(e) = grids.entry(grid) {
                e.insert(time);
            } else {
                break;
            }
        }

        let (data, time) = grids.iter().min_by_key(|e| e.0.len()).unwrap();

        let mut decoder = GzDecoder::new(data.as_slice());
        let mut grid = String::new();
        decoder.read_to_string(&mut grid)?;

        println!("{grid}");
        println!("{time}");
    }

    Ok(())
}
