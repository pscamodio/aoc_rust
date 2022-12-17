use std::{
    error::Error,
    fs, collections::HashMap
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord {x, y}
    }

    fn parse(s: &str) -> Coord {
        let mut tokens = s.split(",");
        let x : i32 = tokens.next().unwrap().parse().unwrap();
        let y : i32 = tokens.next().unwrap().parse().unwrap();
        Coord {x, y}
    }

    fn line(from: Coord, to: Coord) -> Vec<Coord> {
        let mut step = Coord::new(to.x - from.x, to.y - from.y);
        if step.x != 0 {
            step.x /= step.x.abs();
        }
        if step.y != 0 {
            step.y /= step.y.abs();
        }
        let mut curr = from;
        let mut res = Vec::new();
        res.push(curr);
        loop {
            curr.x += step.x;
            curr.y += step.y;
            res.push(curr);
            if curr == to {
                break;
            }
        } 

        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    SandSource,
    Air,
    Rock,
    Sand
}

struct Cave {
    map: HashMap<Coord, Cell>,
    min: Coord,
    max: Coord,
    source: Coord
}

enum SandMovement {
    Stop,
    Move(Coord)
}

impl Cave {
    fn new() -> Cave {
        let mut cave = Cave {
            map: HashMap::new(),
            min: Coord::new(500, 0),
            max: Coord::new(500, 0),
            source: Coord::new(500, 0)
        };
        cave.map.insert(cave.source, Cell::SandSource);

        cave
    }

    fn add(&mut self, coord: Coord, cell: Cell) {
        self.map.insert(coord, cell);
        self.min.x = std::cmp::min(self.min.x, coord.x);
        self.max.x = std::cmp::max(self.max.x, coord.x);
        if cell != Cell::Sand {
            self.min.y = std::cmp::min(self.min.y, coord.y);
            self.max.y = std::cmp::max(self.max.y, coord.y);
        }
    }

    fn get(&self, coord: Coord) -> Cell {
        let default = if coord.y == self.max.y + 2 { Cell::Rock } else { Cell::Air };
        self.map.get(&coord).unwrap_or(&default).clone()
    }

    fn add_row_of_rocks(&mut self, from: Coord, to: Coord) {
        for point in Coord::line(from, to) {
            self.add(point, Cell::Rock);
        }
    }

    fn check_sand_movement(&self, sand: Coord) -> SandMovement {
        let test = Coord::new(sand.x, sand.y+1);
        if self.get(test) == Cell::Air {
            return SandMovement::Move(test);
        }
        let test = Coord::new(sand.x-1, sand.y+1);
        if self.get(test) == Cell::Air {
            return SandMovement::Move(test);
        }
        let test = Coord::new(sand.x+1, sand.y+1);
        if self.get(test) == Cell::Air {
            return SandMovement::Move(test);
        }

        SandMovement::Stop
    }

    fn drop_sand(&mut self) -> Coord {
        let mut sand = self.source.clone();
        loop {
            match self.check_sand_movement(sand) {
                SandMovement::Stop => {
                    self.add(sand, Cell::Sand);
                    break;
                },
                SandMovement::Move(x) => {
                    sand = x;
                }
            }
        }
        sand
    }


    fn print(&self) {
        let mut str = String::new();
        for row in self.min.y..=self.max.y+2 {
            for col in self.min.x..=self.max.x {
                match self.get(Coord::new(col, row)) {
                    Cell::Air => str.push('.'),
                    Cell::Rock => str.push('#'),
                    Cell::Sand => str.push('o'),
                    Cell::SandSource => str.push('+'),
                }
            }
            str.push('\n');
        }
        println!("{str}");
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_14/input.txt")?;
    let mut cave = Cave::new();

    for line in input.lines() {
        let points : Vec<Coord> = line.split(" -> ").map(|x| Coord::parse(x)).collect();
        for idx in 0..points.len() - 1 {
            let from = points[idx];
            let to = points[idx + 1];
            cave.add_row_of_rocks(from, to);
        }
    }

    let mut cnt = 0;
    while cave.drop_sand() != cave.source {
        cnt += 1;
    }

    cave.print();

    println!("Sand: {}", cnt + 1);

    Ok(())
}
