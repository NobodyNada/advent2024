pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let mut c = chal()?;
    let mut input = String::new();
    c.input.read_to_string(&mut input)?;
    let regex = regex::Regex::new(r"(?:mul\((\d+),(\d+)\)|don't\(\))")?;
    let do_regex = regex::Regex::new(r"do\(\)")?;

    let mut idx = 0;
    let mut sum = 0;
    loop {
        let Some(next) = regex.captures_at(&input, idx) else {
            break;
        };
        idx = next.get(0).unwrap().range().end;

        if let (Some(a), Some(b)) = (next.get(1), next.get(2)) {
            // mul
            let a = a.as_str().parse::<u32>().unwrap();
            let b = b.as_str().parse::<u32>().unwrap();
            sum += a * b;
        } else if c.part2 {
            // don't, find do
            let Some(next) = do_regex.find_at(&input, idx) else {
                break;
            };
            idx = next.range().end;
        }
    }
    println!("{sum}");

    Ok(())
}
