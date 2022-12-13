use std::{error::Error, fs};

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
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

    fn get(&self, row: usize, col: usize) -> char {
        assert!(col < self.width);
        assert!(row < self.height);
        self.data[col + row * self.width]
    }
}

fn print(map: &Map) {
    let mut screen = String::new();
    for row in 0..map.height {
        for col in 0..map.width {
            screen.push(map.get(row, col));
        }
        screen.push('\n');
    }
    println!("{}", screen);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_12/input.txt")?;

    let map = Map::load(&input);

    print(&map);

    Ok(())
}
