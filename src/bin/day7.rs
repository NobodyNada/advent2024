use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let ok = input
        .map(|line| {
            let (first, rest) = line.split_once(": ").unwrap();
            (
                first.parse::<u64>().unwrap(),
                rest.split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .filter(|(result, nums)| {
            check(
                if c.part2 {
                    &[Operator::Add, Operator::Mul, Operator::Concat]
                } else {
                    &[Operator::Add, Operator::Mul]
                },
                *result,
                nums[0],
                &nums[1..],
            )
        });

    println!("{}", ok.map(|(result, _)| result).sum::<u64>());

    Ok(())
}

enum Operator {
    Add,
    Mul,
    Concat,
}
impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> Option<u64> {
        match self {
            Operator::Add => lhs.checked_add(rhs),
            Operator::Mul => lhs.checked_mul(rhs),
            Operator::Concat => lhs
                .checked_mul(10u64.checked_pow(rhs.checked_ilog10().unwrap_or(0) + 1)?)?
                .checked_add(rhs),
        }
    }
}
fn check(ops: &[Operator], target: u64, accum: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        target == accum
    } else {
        for op in ops {
            if let Some(accum) = op.apply(accum, nums[0]) {
                if accum <= target && check(ops, target, accum, &nums[1..]) {
                    return true;
                }
            }
        }
        false
    }
}
