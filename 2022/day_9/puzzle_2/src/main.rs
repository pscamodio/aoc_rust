use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug},
    fs,
};

#[derive(Clone, Copy)]
struct Knot {
    row: i64,
    col: i64,
}

impl Debug for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.row, self.col)
    }
}

impl Knot {
    fn key(&self) -> String {
        format!("[{},{}]", self.row, self.col)
    }

    fn need_catch_up(&self, other: Knot) -> bool {
        self.col.abs_diff(other.col) > 1 || self.row.abs_diff(other.row) > 1
    }

    fn catch_up(&self, other: Knot) -> Knot {
        let mut next = *self;
        if !self.need_catch_up(other) {
            next
        } else {
            let row_move = other.row - self.row;
            let col_move = other.col - self.col;
            if row_move == 0 {
                let col_move = col_move / col_move.abs();
                next.col += col_move;
            } else if col_move == 0 {
                let row_move = row_move / row_move.abs();
                next.row += row_move;
            } else {
                let col_move = col_move / col_move.abs();
                let row_move = row_move / row_move.abs();
                next.col += col_move;
                next.row += row_move;
            }
            next
        }
    }

    fn do_move(&self, movement: Movement) -> Knot {
        let mut next = self.clone();
        match movement {
            Movement::Down => next.row += 1,
            Movement::Up => next.row -= 1,
            Movement::Left => next.col -= 1,
            Movement::Right => next.col += 1,
        }
        next
    }
}

#[derive(Clone, Copy)]
struct Rope {
    knots: [Knot; 10],
}

impl Rope {
    fn new() -> Rope {
        Rope {
            knots: [Knot { row: 0, col: 0 }; 10],
        }
    }

    fn move_head(self, movement: Movement) -> Rope {
        let mut new_rope = Rope::new();
        new_rope.knots[0] = self.knots[0].do_move(movement);
        for i in 1..self.knots.len() {
            let other = new_rope.knots[i - 1];
            new_rope.knots[i] = self.knots[i].catch_up(other)
        }

        new_rope
    }

    fn tail(&self) -> Knot {
        self.knots[9]
    }

    fn cell_label(&self, row: i64, col: i64) -> Option<String> {
        for (idx, knot) in self.knots.iter().enumerate() {
            if knot.row == row && knot.col == col {
                return Some(idx.to_string());
            }
        }
        None
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

fn print_rope(rope: &Rope, set: &HashSet<String>) {
    const WIDTH: i64 = 50;
    const HEIGHT: i64 = 50;
    let mut screen = String::new();

    for row in -HEIGHT / 2..HEIGHT / 2 {
        for col in -WIDTH / 2..WIDTH / 2 {
            if let Some(knot) = &rope.cell_label(row, col) {
                screen += knot;
            } else if row == 0 && col == 0 {
                screen += "s"
            } else if set.contains(&Knot { row, col }.key()) {
                screen += "#"
            } else {
                screen += "."
            }
        }
        screen += "\n";
    }
    println!("{}", screen);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_9/input.txt")?;

    let movements = parse_input(&input);

    let mut rope = Rope::new();

    let mut set = HashSet::new();

    for movement in movements {
        rope = rope.move_head(movement);

        set.insert(rope.tail().key());
        //print_rope(&rope, &set);
    }

    println!("Visited: {}", set.len());

    Ok(())
}
