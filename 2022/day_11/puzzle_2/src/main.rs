use std::{error::Error, fs};

use regex::Regex;

type ItemType = u64;

#[derive(Debug, Clone, Copy)]
enum OperationInput {
    Old,
    Value(u16),
}

#[derive(Debug, Clone, Copy)]
enum OperationOp {
    Sum,
    Mul,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    left: OperationInput,
    right: OperationInput,
    op: OperationOp,
}

struct Monkey {
    id: usize,
    items: Vec<ItemType>,
    operation: Operation,
    divisible_by: u64,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

#[derive(Debug, Clone)]
struct MonkeyThrow {
    item: ItemType,
    target: usize,
}

impl Monkey {
    fn parse_id(line: &str) -> usize {
        let rx = Regex::new("Monkey (.*):").unwrap();
        let captures = rx.captures(line).unwrap();
        let id_str = &captures[1];
        id_str.parse().unwrap()
    }

    fn parse_items(line: &str) -> Vec<ItemType> {
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

    fn parse_divisible_by(line: &str) -> u64 {
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

    fn inspect_and_throw(&mut self, lcm: u64) -> Option<MonkeyThrow> {
        if self.items.is_empty() {
            return None;
        }
        self.inspection_count += 1;
        let item = self.items.drain(0..1).next().unwrap();
        let item = self.apply_operation(&item);
        let test = item % self.divisible_by == 0;
        let item = item % lcm;
        let res = MonkeyThrow {
            item: item,
            target: if test {
                self.true_target
            } else {
                self.false_target
            },
        };
        return Some(res);
    }

    fn apply_operation(&self, item: &ItemType) -> ItemType {
        let left = match self.operation.left {
            OperationInput::Old => item.clone(),
            OperationInput::Value(x) => ItemType::from(x),
        };
        let right = match self.operation.right {
            OperationInput::Old => item.clone(),
            OperationInput::Value(x) => ItemType::from(x),
        };
        match self.operation.op {
            OperationOp::Mul => left * right,
            OperationOp::Sum => left + right,
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>, lcm: u64) {
    for monkey_id in 0..monkeys.len() {
        while let Some(throw) = monkeys[monkey_id].inspect_and_throw(lcm) {
            monkeys[throw.target].items.push(throw.item)
        }
    }
}

fn gcd(first: u64, second: u64) -> u64 {
    let (mut max, mut min) = (first, second);
    if min < max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_11/input.txt")?;
    let monkey_strings: Vec<&str> = input.split("\n\n").collect();

    let mut monkeys: Vec<Monkey> = monkey_strings.iter().map(|x| Monkey::new(x)).collect();

    let lcm = monkeys.iter().fold(1, |prev, m| lcm(prev, m.divisible_by));

    for r in 1..10001 {
        round(&mut monkeys, lcm);
        if r == 1 || r == 20 || r % 1000 == 0 {
            println!("== After round {} ==", r);
            for monkey in monkeys.iter() {
                println!(
                    "Monkey {} inspected items {} times",
                    monkey.id, monkey.inspection_count
                );
            }
        }
    }
    for monkey in monkeys.iter() {
        println!(
            "Monkey {} inspected {} times",
            monkey.id, monkey.inspection_count
        );
    }

    let mut counts: Vec<usize> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort_unstable();
    let best = &counts[counts.len() - 2..];

    println!("Monkey Business {}", best[0] * best[1]);
    Ok(())
}
