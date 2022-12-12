use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug)]
pub struct FSEntry {
    pub name: String,
    pub size: u32,
    pub children: Vec<Rc<RefCell<FSEntry>>>,
    pub parent: Option<Rc<RefCell<FSEntry>>>,
    pub is_dir: bool,
}

impl FSEntry {
    pub fn new(
        is_dir: bool,
        name: String,
        size: u32,
        parent: Option<Rc<RefCell<FSEntry>>>,
    ) -> Rc<RefCell<FSEntry>> {
        Rc::new(RefCell::new(FSEntry {
            name,
            size,
            children: Vec::new(),
            parent: parent,
            is_dir,
        }))
    }

    pub fn update_size(&mut self) -> u32 {
        if !self.children.is_empty() {
            self.size = self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.borrow_mut().update_size())
        }

        self.size
    }

    pub fn append(&mut self, entry: Rc<RefCell<FSEntry>>) -> &mut Rc<RefCell<FSEntry>> {
        self.children.push(entry);
        self.children.last_mut().unwrap()
    }

    pub fn full_path(&self) -> String {
        let mut tokens = vec![self.name.to_string()];
        let mut current = self.parent.clone();
        while current.is_some() {
            if let Some(x) = &current {
                tokens.push(x.borrow().name.to_string());
            }
            let parent = current.unwrap().borrow().parent.clone();
            current = parent;
        }
        if tokens.len() > 1 {
            tokens.pop();
            tokens.push("".to_string());
        }
        tokens.reverse();
        tokens.join("/")
    }
}

fn find_dir_rec_impl(
    node: &Rc<RefCell<FSEntry>>,
    predicate: &Box<dyn Fn(u32) -> bool>,
    found: &mut Vec<Rc<RefCell<FSEntry>>>,
) {
    if node.borrow().is_dir && predicate(node.borrow().size) {
        found.push(node.clone())
    }

    for child in &node.borrow().children {
        find_dir_rec_impl(child, predicate, found);
    }
}

pub fn find_dir_rec(
    node: &Rc<RefCell<FSEntry>>,
    predicate: Box<dyn Fn(u32) -> bool>,
    found: &mut Vec<Rc<RefCell<FSEntry>>>,
) {
    if node.borrow().is_dir && predicate(node.borrow().size) {
        found.push(node.clone())
    }

    for child in &node.borrow().children {
        find_dir_rec_impl(child, &predicate, found);
    }
}

impl fmt::Display for FSEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}\n", self.full_path(), self.size)?;
        for child in &self.children {
            child.borrow().fmt(f)?;
        }
        Ok(())
    }
}

pub fn parse_fs(input: String) -> Rc<RefCell<FSEntry>> {
    let root = FSEntry::new(true, "/".to_string(), 0, None);
    let mut current = root.clone();

    for line in input.lines() {
        // Ignore root cd and all dir lines, we don't care about them
        if line.starts_with("$ cd /") || line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        }
        // cd .. go up one level
        if line.starts_with("$ cd ..") {
            let parent = current.borrow().parent.clone().unwrap();
            current = parent;
            continue;
        }
        // bare cd create a new folder in our tree representation
        if line.starts_with("$ cd") {
            let folder = &line[5..];

            let child = FSEntry::new(true, folder.to_string(), 0, Some(current.clone()));

            current.borrow_mut().append(child.clone());

            current = child;

            continue;
        }
        // the rest are files
        let mut tokens = line.split(" ");
        let size: u32 = tokens.next().unwrap().parse().unwrap();
        let name: &str = tokens.next().unwrap();
        let child = FSEntry::new(false, name.to_string(), size, Some(current.clone()));
        current.borrow_mut().append(child);
    }

    root
}
