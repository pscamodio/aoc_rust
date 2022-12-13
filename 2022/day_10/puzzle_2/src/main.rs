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

struct Cpu {
    x: i64,
    cycle: i64,
    executing: Option<Processing>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
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
}

struct Screen {
    width: usize,
    row: usize,
    col: usize,
    res: String,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            width: 40,
            row: 0,
            col: 0,
            res: String::new(),
        }
    }

    fn clock(&mut self, cpu: &Cpu) {
        let lit = self.col.abs_diff(cpu.x as usize) < 2;
        self.res.push(match lit {
            true => '#',
            false => '.',
        });
        self.col += 1;
        if self.col == self.width {
            self.row += 1;
            self.col = 0;
            self.res.push('\n');
        }
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

    let mut cpu = Cpu::new();
    let mut screen = Screen::new();

    while cpu.clock(&mut program) {
        screen.clock(&cpu);
    }

    println!("{}", screen.res);

    Ok(())
}
