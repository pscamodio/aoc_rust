use std::str::Split;

#[derive(Debug)]
pub struct Section {
    from: i32,
    to: i32,
}

impl Section {
    pub fn new(mut tokens: Split<&str>) -> Section {
        Section {
            from: tokens.next().unwrap().parse().unwrap(),
            to: tokens.next().unwrap().parse().unwrap(),
        }
    }

    pub fn contains(&self, other: &Section) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn overlap(&self, other: &Section) -> bool {
        self.from <= other.to && other.from <= self.to
    }
}

pub fn parse_sections(line: &str) -> (Section, Section) {
    let mut tokens = line.split(",");
    let first = tokens.next().unwrap().split("-");
    let second = tokens.next().unwrap().split("-");

    (Section::new(first), Section::new(second))
}
