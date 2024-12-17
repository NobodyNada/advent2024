use std::sync::Arc;

pub use aoc_2024::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy)]
enum Instruction {
    Adv(Combo),
    Bxl(u8),
    Bst(Combo),
    Jnz(u8),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Clone, Copy)]
struct Combo(u8);

impl Combo {
    fn apply(&self, vm: &Vm) -> u64 {
        match self.0 {
            n @ 0..4 => n as u64,
            4 => vm.a,
            5 => vm.b,
            6 => vm.c,
            _ => panic!("invalid operand"),
        }
    }
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Instruction {
        use Instruction::*;
        match opcode {
            0 => Adv(Combo(operand)),
            1 => Bxl(operand),
            2 => Bst(Combo(operand)),
            3 => Jnz(operand),
            4 => Bxc,
            5 => Out(Combo(operand)),
            6 => Bdv(Combo(operand)),
            7 => Cdv(Combo(operand)),
            _ => panic!("invalid opcode"),
        }
    }
}

#[derive(Clone)]
struct Vm {
    a: u64,
    b: u64,
    c: u64,

    pc: usize,
    program: Arc<Vec<u8>>,
}

enum Status {
    Step(Option<u64>),
    Done,
}

impl Vm {
    fn new(mut input: impl Iterator<Item = String>) -> Vm {
        let a = input
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let b = input
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let c = input
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();

        assert!(input.next().unwrap().is_empty());

        let program = Arc::new(
            input
                .next()
                .unwrap()
                .strip_prefix("Program: ")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect(),
        );

        Vm {
            a,
            b,
            c,
            program,
            pc: 0,
        }
    }

    fn step(&mut self) -> Status {
        let (Some(&opcode), Some(&operand)) =
            (self.program.get(self.pc), self.program.get(self.pc + 1))
        else {
            return Status::Done;
        };
        let inst = Instruction::new(opcode, operand);

        self.pc += 2;
        match inst {
            Instruction::Adv(combo) => self.a >>= combo.apply(self),
            Instruction::Bdv(combo) => self.b = self.a >> combo.apply(self),
            Instruction::Cdv(combo) => self.c = self.a >> combo.apply(self),

            Instruction::Bxl(op) => self.b ^= op as u64,
            Instruction::Bst(combo) => self.b = combo.apply(self) & 7,
            Instruction::Jnz(op) => {
                if self.a != 0 {
                    self.pc = op as usize
                }
            }
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out(combo) => return Status::Step(Some(combo.apply(self) & 7)),
        }

        Status::Step(None)
    }
}

impl Iterator for Vm {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            match self.step() {
                Status::Step(None) => continue,
                Status::Step(Some(x)) => return Some(x),
                Status::Done => return None,
            }
        }
    }
}

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);
    let vm = Vm::new(input);
    let program = vm.program.clone();

    println!("{}", vm.clone().join(","));

    let mut valid = (0..256).collect_vec();
    for len in 0..program.len() {
        valid = valid
            .into_iter()
            .flat_map(|a| {
                (0..8).map(move |x| {
                    let shift = 8 + 3 * len;
                    assert!(a >> shift == 0);
                    a | (x << shift)
                })
            })
            .filter(|&a| {
                Vm { a, ..vm.clone() }
                    .take(len)
                    .eq(program.iter().take(len).map(|n| *n as u64))
            })
            .collect();
        println!("{:?}", valid.first());
    }

    Ok(())
}
