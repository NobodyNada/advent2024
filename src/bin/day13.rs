pub use aoc_2024::prelude::*;
use regex::Regex;

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

fn det([a, b, c, d]: [i64; 4]) -> Option<i64> {
    (a * d).checked_sub(b * c)
}

impl Machine {
    fn solve(mut self, part2: bool) -> Option<i64> {
        if part2 {
            self.target.0 += 10000000000000;
            self.target.1 += 10000000000000;
        }
        let det_x = det([self.target.0, self.b.0, self.target.1, self.b.1])?;
        let det_y = det([self.a.0, self.target.0, self.a.1, self.target.1])?;
        let det = det([self.a.0, self.b.0, self.a.1, self.b.1])?;

        if det == 0 || det_x % det != 0 || det_y % det != 0 {
            None
        } else {
            let a = det_x / det;
            let b = det_y / det;
            if !part2 && (a > 100 || b > 100) {
                None
            } else {
                Some(a * 3 + b)
            }
        }
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap).peekable();

    let a_re = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$")?;
    let b_re = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$")?;
    let prize_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$")?;
    let mut cost = 0;
    loop {
        let (a, b, target) = &(
            input.next().ok_or_else(|| anyhow!("unexpected EOF"))?,
            input.next().ok_or_else(|| anyhow!("unexpected EOF"))?,
            input.next().ok_or_else(|| anyhow!("unexpected EOF"))?,
        );

        let [ax, ay] = a_re
            .captures(a)
            .ok_or_else(|| anyhow!("invalid input A: {a}"))?
            .extract()
            .1;
        let [bx, by] = b_re
            .captures(b)
            .ok_or_else(|| anyhow!("invalid input B: {b}"))?
            .extract()
            .1;
        let [tx, ty] = prize_re
            .captures(target)
            .ok_or_else(|| anyhow!("invalid input prize: {target}"))?
            .extract()
            .1;

        cost += Machine {
            a: (ax.parse()?, ay.parse()?),
            b: (bx.parse()?, by.parse()?),
            target: (tx.parse()?, ty.parse()?),
        }
        .solve(c.part2)
        .unwrap_or(0);

        if input.next().is_none() {
            break;
        }
    }

    println!("{cost}");

    Ok(())
}
