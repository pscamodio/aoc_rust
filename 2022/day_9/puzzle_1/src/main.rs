use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug},
    fs,
};

#[derive(Clone, Copy)]
struct Cell {
    row: i64,
    col: i64,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.row, self.col)
    }
}

impl Cell {
    fn key(&self) -> String {
        format!("{}-{}", self.row, self.col)
    }
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Movement {
    fn parse(s: &str, res: &mut Vec<Movement>) {
        let mut tokens = s.split(" ");
        let dir = tokens.next().unwrap();
        let amount: usize = tokens.next().unwrap().parse().unwrap();
        let dir = match dir {
            "L" => Movement::Left,
            "R" => Movement::Right,
            "U" => Movement::Up,
            "D" => Movement::Down,
            _ => panic!("Invalid movement {dir}"),
        };
        for _ in 0..amount {
            res.push(dir);
        }
    }
}

fn parse_input(str: &str) -> Vec<Movement> {
    let mut res: Vec<Movement> = Vec::new();
    for line in str.lines() {
        Movement::parse(line, &mut res);
    }
    res
}

fn need_catch_up(head: Cell, tail: Cell) -> bool {
    head.col.abs_diff(tail.col) > 1 || head.row.abs_diff(tail.row) > 1
}

fn catch_up(head: Cell, prev: Cell, tail: Cell) -> Cell {
    let need = need_catch_up(dbg!(head), dbg!(tail));
    let prev = dbg!(prev);
    let new_tail = if dbg!(need) { prev } else { tail };

    dbg!(new_tail)
}

fn go(head: &mut Cell, tail: &mut Cell, movement: Movement) {
    let prev = head.clone();
    match movement {
        Movement::Down => head.col -= 1,
        Movement::Up => head.col += 1,
        Movement::Left => head.row -= 1,
        Movement::Right => head.row += 1,
    }
    *tail = catch_up(*head, prev, *tail);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_9/input.txt")?;

    let movements = parse_input(&input);

    let mut head = Cell { col: 0, row: 0 };
    let mut tail = Cell { col: 0, row: 0 };

    let mut set = HashSet::new();

    for movement in movements {
        go(&mut head, &mut tail, movement);

        set.insert(tail.key());
    }

    println!("Visited: {}", set.len());

    Ok(())
}
