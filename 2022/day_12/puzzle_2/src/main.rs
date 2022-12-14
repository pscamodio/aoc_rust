use std::{
    collections::{HashMap, LinkedList},
    error::Error,
    fmt::Display,
    fs, ops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: i64,
    col: i64,
}

impl Coord {
    fn new(row: i64, col: i64) -> Coord {
        Coord { row, col }
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, offset: Coord) -> Coord {
        Coord {
            row: self.row + offset.row,
            col: self.col + offset.col,
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.row, self.col)
    }
}

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}

fn cell_height(cell: char) -> u8 {
    match cell {
        'S' => 0,
        'a'..='z' => cell as u8 - 'a' as u8,
        'E' => 'z' as u8 - 'a' as u8 + 1u8,
        _ => panic!("Invalid cell"),
    }
}

impl Map {
    fn load(str: &str) -> Map {
        let width = str.lines().next().unwrap().len();
        let data: Vec<char> = str.chars().filter(|c| c.is_alphabetic()).collect();
        Map::new(data, width)
    }

    fn new(data: Vec<char>, width: usize) -> Map {
        assert_eq!(data.len() % width, 0);
        let height = data.len() / width;
        Map {
            data,
            width,
            height,
        }
    }

    fn get_letter(&self, coord: Coord) -> char {
        self.data[self.coord_to_idx(coord)]
    }

    fn get_height(&self, coord: Coord) -> u8 {
        cell_height(self.data[self.coord_to_idx(coord)])
    }

    fn starting_pos(&self) -> Coord {
        self.idx_to_coord(self.data.iter().position(|c| *c == 'S').unwrap())
    }

    fn target_pos(&self) -> Coord {
        self.idx_to_coord(self.data.iter().position(|c| *c == 'E').unwrap())
    }

    fn idx_to_coord(&self, idx: usize) -> Coord {
        let row = idx / self.width;
        let col = idx % self.width;
        Coord {
            row: row as i64,
            col: col as i64,
        }
    }

    fn all_coord_with_letter(&self, letter: char) -> Vec<Coord> {
        let mut ret = Vec::new();
        for (idx, l) in self.data.iter().enumerate() {
            if letter == *l {
                ret.push(self.idx_to_coord(idx))
            }
        }
        ret
    }

    fn coord_to_idx(&self, coord: Coord) -> usize {
        let row = coord.row as usize;
        let col = coord.col as usize;
        assert!(col < self.width);
        assert!(row < self.height);
        col + row * self.width
    }

    fn inside_map(&self, coord: Coord) -> bool {
        if coord.row < 0 || coord.col < 0 {
            return false;
        }
        let row = coord.row as usize;
        let col = coord.col as usize;
        row < self.height && col < self.width
    }

    fn reachable_cells(&self, from: Coord) -> Vec<Coord> {
        let offsets = [
            Coord::new(-1, 0),
            Coord::new(1, 0),
            Coord::new(0, -1),
            Coord::new(0, 1),
        ];
        let mut ret = Vec::new();
        let from_height = self.get_height(from);
        for offset in offsets {
            let target = from + offset;
            if !self.inside_map(target) {
                continue;
            }
            let target_height = self.get_height(target);
            if target_height > from_height + 1 {
                continue;
            }
            ret.push(target);
        }

        ret
    }
}

fn print(map: &Map, current: Coord) {
    let mut screen = String::new();
    for row in 0..map.height {
        for col in 0..map.width {
            let pos = Coord::new(row as i64, col as i64);
            if pos == current {
                screen.push('X');
            } else {
                screen.push(map.get_letter(pos));
            }
        }
        screen.push('\n');
    }
    println!("{}", screen);
}

fn bfs(map: &Map, start: Coord, target: Coord) -> Option<usize> {
    let mut distances: HashMap<Coord, usize> = HashMap::new();
    let mut visited = Vec::new();
    let mut queue = LinkedList::from([start]);
    distances.insert(start, 0);

    while let Some(coord) = queue.pop_front() {
        visited.push(coord);
        println!("Checked {} of {}", visited.len(), map.width * map.height);
        let dist = *distances.get(&coord).unwrap();
        if coord == target {
            return Some(dist);
        }
        let paths = map.reachable_cells(coord);
        for path in paths {
            // Ignore paths that we already visited
            if visited.contains(&path) {
                continue;
            }
            // Update distances of nodes already checked but not visited
            if let Some(path_dist) = distances.get(&path) {
                if dist < *path_dist {
                    distances.insert(path, dist + 1);
                }
            } else {
                distances.insert(path, dist + 1);
                queue.push_back(path);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_12/input.txt")?;

    let map = Map::load(&input);

    let candidates = map.all_coord_with_letter('a');

    let target = map.target_pos();

    let min = candidates
        .iter()
        .map(|s| bfs(&map, *s, target))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min()
        .unwrap();

    println!("Distance {min}");

    Ok(())
}
