use std::{error::Error, fs};

use regex::Regex;

enum OperationInput {
    Old,
    Value(i32),
}

enum OperationOp {
    Sum,
    Mul,
}

struct Operation {
    left: OperationInput,
    right: OperationInput,
    op: OperationOp,
}

struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: Operation,
    divisible_by: i32,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

struct MonkeyThrow {
    item: i32,
    target: usize,
}

impl Monkey {
    fn parse_id(line: &str) -> usize {
        let rx = Regex::new("Monkey (.*):").unwrap();
        let captures = rx.captures(line).unwrap();
        let id_str = &captures[1];
        id_str.parse().unwrap()
    }

    fn parse_items(line: &str) -> Vec<i32> {
        let rx = Regex::new("Starting items: (.*)").unwrap();
        rx.captures(line).unwrap()[1]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn parse_operation(line: &str) -> Operation {
        let rx = Regex::new("Operation: new = (.*) (.*) (.*)").unwrap();
        let captures = rx.captures(line).unwrap();
        let left = match &captures[1] {
            "old" => OperationInput::Old,
            s => OperationInput::Value(s.parse().unwrap()),
        };
        let op = match &captures[2] {
            "+" => OperationOp::Sum,
            "*" => OperationOp::Mul,
            _ => panic!("Invalid Op"),
        };
        let right = match &captures[3] {
            "old" => OperationInput::Old,
            s => OperationInput::Value(s.parse().unwrap()),
        };
        Operation { left, right, op }
    }

    fn parse_divisible_by(line: &str) -> i32 {
        let rx = Regex::new("Test: divisible by (.*)").unwrap();
        rx.captures(line).unwrap()[1].parse().unwrap()
    }

    fn parse_target(line: &str) -> usize {
        let rx = Regex::new("If .*: throw to monkey (.*)").unwrap();
        rx.captures(line).unwrap()[1].parse().unwrap()
    }

    fn new(str: &str) -> Monkey {
        let mut lines = str.lines();
        Monkey {
            id: Monkey::parse_id(lines.next().unwrap()),
            items: Monkey::parse_items(lines.next().unwrap()),
            operation: Monkey::parse_operation(lines.next().unwrap()),
            divisible_by: Monkey::parse_divisible_by(lines.next().unwrap()),
            true_target: Monkey::parse_target(lines.next().unwrap()),
            false_target: Monkey::parse_target(lines.next().unwrap()),
            inspection_count: 0,
        }
    }

    fn inspect_and_throw(&mut self) -> Option<MonkeyThrow> {
        if self.items.is_empty() {
            return None;
        }
        self.inspection_count += 1;
        let item = self.items.drain(0..1).next().unwrap();
        println!("  Monkey inspects an item with worry level of {item}");
        let item = self.apply_operation(item);
        println!("    Worry increase to {item}");
        let item = item / 3;
        println!("    Monkey is bored, reduce worry to {item}");
        let test = item % self.divisible_by == 0;
        println!(
            "    Current level is divisible by {} - {}",
            self.divisible_by, test
        );
        let res = MonkeyThrow {
            item,
            target: if test {
                self.true_target
            } else {
                self.false_target
            },
        };
        println!(
            "    Item with worry level {} is thrown to monkey {}",
            res.item, res.target
        );
        return Some(res);
    }

    fn apply_operation(&self, item: i32) -> i32 {
        let left = match self.operation.left {
            OperationInput::Old => item,
            OperationInput::Value(x) => x,
        };
        let right = match self.operation.right {
            OperationInput::Old => item,
            OperationInput::Value(x) => x,
        };
        match self.operation.op {
            OperationOp::Mul => left * right,
            OperationOp::Sum => left + right,
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for monkey_id in 0..monkeys.len() {
        println!("Monkey {monkey_id}");
        while let Some(throw) = monkeys[monkey_id].inspect_and_throw() {
            monkeys[throw.target].items.push(throw.item)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_11/input.txt")?;
    let monkey_strings: Vec<&str> = input.split("\n\n").collect();

    let mut monkeys: Vec<Monkey> = monkey_strings.iter().map(|x| Monkey::new(x)).collect();

    for _ in 0..20 {
        round(&mut monkeys);
    }
    for monkey in monkeys.iter() {
        println!(
            "Monkey {} inspected {} times",
            monkey.id, monkey.inspection_count
        );
    }

    let mut counts: Vec<usize> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort_unstable();
    let best = dbg!(&counts[counts.len() - 2..]);

    println!("Monkey Business {}", best[0] * best[1]);
    Ok(())
}
