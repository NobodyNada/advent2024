pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap);

    let towels = input.next().unwrap();
    let towels = towels.split(", ").collect_vec();
    assert_eq!(input.next().as_deref(), Some(""));

    let arrangements = input.map(|design| make_design(&design, 0, &mut Vec::new(), &towels));
    if c.part1 {
        println!("{}", arrangements.filter(|&n| n != 0).count());
    } else {
        println!("{}", arrangements.sum::<u64>());
    }

    Ok(())
}

fn make_design(design: &str, prefix: usize, memo: &mut Vec<Option<u64>>, towels: &[&str]) -> u64 {
    if prefix == design.len() {
        1
    } else if let Some(Some(designs)) = memo.get(prefix) {
        *designs
    } else {
        let designs = towels
            .iter()
            .filter(|towel| design[prefix..].starts_with(*towel))
            .map(|towel| make_design(design, prefix + towel.len(), memo, towels))
            .sum();

        memo.resize(memo.len().max(prefix + 1), None);
        memo[prefix] = Some(designs);
        designs
    }
}
