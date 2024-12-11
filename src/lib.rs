use std::io::{BufReader, Read, Seek};

use anyhow::{Result, bail};

pub mod prelude {
    pub use std::collections::HashMap;
    pub use std::io::{BufRead, Read};

    pub use super::chal;
    pub use anyhow::{Context, Ok, Result, anyhow, bail, ensure};

    pub use itertools::Itertools;

    pub trait Collect2d: Iterator
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        fn collect_2d(self) -> ndarray::Array2<<Self::Item as IntoIterator>::Item> {
            let mut ncols = None;
            let mut nrows = 0;
            let mut elements = Vec::new();
            for row in self {
                let old_len = elements.len();
                elements.extend(row);
                let row_len = elements.len() - old_len;
                if let Some(ncols) = ncols {
                    assert_eq!(ncols, row_len);
                } else {
                    ncols = Some(row_len);
                }
                nrows += 1;
            }

            ndarray::Array2::from_shape_vec((ncols.unwrap_or(0), nrows), elements).unwrap()
        }
    }
    impl<T: Iterator<Item: IntoIterator> + Sized> Collect2d for T {}
}

pub struct Challenge {
    pub part1: bool,
    pub part2: bool,
    pub input: BufReader<Box<dyn Read>>,
}

pub fn chal() -> Result<Challenge> {
    let mut args = std::env::args().fuse();

    let (part2, infile) = match (
        args.next().as_deref(),
        args.next().as_deref(),
        args.next(),
        args.next().as_deref(),
    ) {
        (_, None | Some("1" | "part1"), infile, None) => (false, infile),
        (_, Some("2" | "part2"), infile, None) => (true, infile),
        (bin, _, _, _) => {
            eprintln!(
                "usage: {} [part1|part2 [infile]]",
                bin.unwrap_or("aoc_2024")
            );
            std::process::exit(1);
        }
    };

    let input: BufReader<Box<dyn Read>> = match infile.as_deref() {
        None => BufReader::new(Box::new(retrieve_input()?)),
        Some("-") => BufReader::new(Box::new(std::io::stdin().lock())),
        Some(file) => BufReader::new(Box::new(std::fs::File::open(file)?)),
    };

    Ok(Challenge {
        part1: !part2,
        part2,
        input,
    })
}

fn retrieve_input() -> Result<impl Read> {
    let bin = std::env::current_exe()?;
    let bin = bin.file_stem().unwrap().to_str().unwrap();
    assert!(bin.starts_with("day"));
    let path = format!("input/{bin}.txt");

    match std::fs::File::open(&path) {
        Ok(file) => return Ok(file),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    };

    // Input file does not exist, download it
    let mut session = match std::fs::read_to_string("session.txt") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            bail!("No input file and session.txt is not present")
        }
        Err(e) => return Err(e.into()),
    };

    if !session.starts_with("session=") {
        session.insert_str(0, "session=");
    }

    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(format!(
            "https://adventofcode.com/2024/day/{}/input",
            bin.strip_prefix("day").unwrap()
        ))
        .header("Cookie", session)
        .header("User-Agent", "https://github.com/NobodyNada/advent2024")
        .send()?
        .error_for_status()?;

    std::fs::create_dir_all("input")?;
    let mut file = std::fs::File::create_new(path)?;
    std::io::copy(&mut response, &mut file)?;

    file.rewind()?;
    Ok(file)
}
