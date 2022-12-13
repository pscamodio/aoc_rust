use core::panic;
use std::{error::Error, fs};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

fn instruction_cycles(instruction: Instruction) -> i64 {
    match instruction {
        Instruction::Noop => 1,
        Instruction::Addx(_) => 2,
    }
}

#[derive(Debug, Clone, Copy)]
struct Processing {
    instruction: Instruction,
    left_cycles: i64,
}

fn process(instruction: Instruction, x: i64) -> i64 {
    match instruction {
        Instruction::Noop => x,
        Instruction::Addx(y) => x + y,
    }
}

struct Screen {
    x: i64,
    cycle: i64,
    executing: Option<Processing>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            x: 1,
            cycle: 0,
            executing: None,
        }
    }

    fn clock(&mut self, input: &mut Vec<Instruction>) -> bool {
        self.cycle += 1;
        if let Some(cmd) = self.executing {
            if cmd.left_cycles == 0 {
                self.x = process(cmd.instruction, self.x);
                self.executing = None;
            }
        }
        if self.executing.is_none() {
            if input.is_empty() {
                return false;
            }
            let next = input.drain(0..1).next().unwrap();
            self.executing = Some(Processing {
                instruction: next,
                left_cycles: instruction_cycles(next),
            })
        }
        let mut cmd = self.executing.unwrap();
        cmd.left_cycles -= 1;
        self.executing = Some(cmd);
        return true;
    }

    fn signal_strength(&self) -> i64 {
        self.cycle * self.x
    }
}

fn read_program(input: String) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split(" ");
        let cmd = tokens.next().unwrap();
        match cmd {
            "noop" => program.push(Instruction::Noop),
            "addx" => program.push(Instruction::Addx(tokens.next().unwrap().parse().unwrap())),
            _ => panic!("Invalid program"),
        }
    }
    program
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_10/input.txt")?;

    let mut program = read_program(input);

    let mut screen = Screen::new();

    let mut s: Vec<i64> = Vec::new();

    while screen.clock(&mut program) {
        let interesting = (screen.cycle - 20) % 40 == 0;
        if interesting {
            println!("Cycle {} - X {}", screen.cycle, screen.x);
            s.push(screen.signal_strength());
        }
    }

    let sum: i64 = s.iter().sum();

    println!("Sum {sum}");

    Ok(())
}
