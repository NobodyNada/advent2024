use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Write},
};

pub use aoc_2024::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(&self, in1: bool, in2: bool) -> bool {
        match self {
            Op::And => in1 & in2,
            Op::Or => in1 | in2,
            Op::Xor => in1 ^ in2,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Label {
    chars: [u8; 3],
}

impl Label {
    fn from_str(s: &str) -> Label {
        assert_eq!(s.len(), 3);
        Label {
            chars: [s.as_bytes()[0], s.as_bytes()[1], s.as_bytes()[2]],
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.chars
            .iter().try_for_each(|c| f.write_char(*c as char))
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone)]
enum Wire {
    Value(bool),
    Gate(Op, Label, Label),
}

impl Wire {
    fn parse_value(input: &str) -> (Label, Wire) {
        let (gate, val) = input.split_once(": ").unwrap();
        (
            Label::from_str(gate),
            Wire::Value(val.parse::<u8>().unwrap() != 0),
        )
    }

    fn parse_gate(input: &str) -> (Label, Wire) {
        let mut words = input.split(" ");
        let in1 = words.next().unwrap();
        let op = match words.next().unwrap() {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("invalid input"),
        };
        let in2 = words.next().unwrap();
        assert_eq!(words.next().unwrap(), "->");
        let out = words.next().unwrap();

        (
            Label::from_str(out),
            Wire::Gate(op, Label::from_str(in1), Label::from_str(in2)),
        )
    }
}

struct Wires { 
    wires: BTreeMap<Label, Wire>,
    swaps: Vec<[Label; 2]>
}

impl Wires {
    fn read(mut input: impl Iterator<Item = String>) -> Wires {
        let mut wires: BTreeMap<Label, Wire> = input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| Wire::parse_value(&line))
            .collect();

        wires.extend(input.map(|line| Wire::parse_gate(&line)));
        Wires { wires, swaps: Vec::new() }
    }

    fn evaluate(&self, wire: Label) -> bool {
        match &self.wires[&wire] {
            Wire::Value(b) => *b,
            Wire::Gate(op, in1, in2) => {
                let op = *op;
                op.apply(self.evaluate(*in1), self.evaluate(*in2))
            }
        }
    }

    fn find_gate(&mut self, op: Op, in1: Label, in2: Label) -> Label {
        self.wires
            .iter()
            .find(|(_, wire)| matches!(wire, Wire::Gate(o, i1, i2) if *o == op && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1))))
            .map(|(l, _)| *l)
            .unwrap_or_else(|| {
                for (label, &wire) in self.wires.iter() {
                    if let 
                        Wire::Gate(o, i1, i2) = wire {
                            if o != op { continue; }
                            let swaps = if i1 == in1 {
                                Some([i2, in2])
                            } else if i1 == in2 {
                                Some([i2, in1])
                            } else if i2 == in1 {
                                Some([i1, in2])
                            } else if i2 == in2 {
                                Some([i1, in1])
                            } else {
                                None
                            };
                            if let Some(swap) = swaps {
                                self.swaps.push(swap);
                                return *label
                            }
                        }
                    }
                panic!("could not resolve swap");
            })
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);
    let mut wires = Wires::read(input);

    let zs = wires.wires.keys().filter(|w| w.chars[0] == (b'z'));
    let mut result: u64 = 0;
    let mut mask = 1;
    for z in zs {
        if wires.evaluate(*z) {
            result |= mask;
        }
        mask <<= 1;
    }
    println!("{result}");

    let xs = wires
        .wires
        .keys()
        .filter(|w| w.chars[0] == (b'x'))
        .copied()
        .collect_vec();
    let ys = wires
        .wires
        .keys()
        .filter(|w| w.chars[0] == (b'y'))
        .copied()
        .collect_vec();

    let mut carry_in = None;
    for (x, y) in xs.into_iter().zip(ys.into_iter()) {
        let xor = wires.find_gate(Op::Xor, x, y);
        let (_result, carry_half0) = if let Some(carry) = carry_in {
            (
                wires.find_gate(Op::Xor, xor, carry),
                Some(wires.find_gate(Op::And, xor, carry)),
            )
        } else {
            (xor, None)
        };

        let carry_half1 = wires.find_gate(Op::And, x, y);
        let carry_out = if carry_in.is_some() {
            wires.find_gate(Op::Or, carry_half0.unwrap(), carry_half1)
        } else {
            carry_half1
        };

        carry_in = Some(carry_out);
    }
    
    println!("{}", wires.swaps.into_iter().flatten().sorted().dedup().join(","));

    Ok(())
}
