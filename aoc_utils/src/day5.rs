pub type Stack = Vec<char>;

pub type Stacks = [Stack; 9];

pub fn init_stacks() -> Stacks {
    [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ]
}

#[derive(Debug)]
pub struct Movement {
    amount: u8,
    from: u8,
    to: u8,
}

impl Movement {
    pub fn new(amount: u8, from: u8, to: u8) -> Movement {
        Movement { amount, from, to }
    }
}

pub fn parse_stack_line(line: &str, stacks: &mut Stacks) {
    if !line.contains("[") {
        return;
    }
    let mut s = line.chars().peekable();
    let mut stack_id = 0;
    while s.peek().is_some() {
        // Consume the first 4 chars of the string
        let chunk: Vec<char> = s.by_ref().take(4).collect();

        // Get out the crate letter
        let char = &chunk[1];

        // If it's not empty push on the stack
        if *char != ' ' {
            stacks[stack_id].push(*char);
        }

        // Check next stack
        stack_id += 1;
    }
}

pub fn parse_initial_stacks<'a, I>(mut input: I) -> Stacks
where
    I: Iterator<Item = &'a str>,
{
    // Init empty stacks
    let mut stacks = init_stacks();

    // Parse all initial file lines to fill the stacks
    loop {
        match input.next() {
            Some("") => break,
            Some(x) => parse_stack_line(x, &mut stacks),
            None => break,
        }
    }

    for stack in stacks.as_mut() {
        stack.reverse();
    }

    stacks
}

pub fn parse_movements<'a, I>(input: I) -> Vec<Movement>
where
    I: Iterator<Item = &'a str>,
{
    let mut movements: Vec<Movement> = Vec::new();

    for line in input {
        let mut tokens = line.split(" ");
        movements.push(Movement::new(
            tokens.nth(1).unwrap().parse().unwrap(),
            tokens.nth(1).unwrap().parse::<u8>().unwrap() - 1,
            tokens.nth(1).unwrap().parse::<u8>().unwrap() - 1,
        ))
    }

    movements
}

pub fn apply_movement_9000(stacks: &mut Stacks, movement: &Movement) {
    for _ in 0..movement.amount {
        let from = &mut stacks[movement.from as usize];
        let c = from.pop().unwrap();

        let to = &mut stacks[movement.to as usize];
        to.push(c);
    }
}

pub fn apply_movements_9000(stacks: &mut Stacks, movements: &Vec<Movement>) {
    for movement in movements {
        apply_movement_9000(stacks, movement)
    }
}

pub fn apply_movement_9001(stacks: &mut Stacks, movement: &Movement) {
    let from = &mut stacks[movement.from as usize];

    let start = from.len() - movement.amount as usize;

    let mut to_move: Vec<char> = from.splice(start..from.len() as usize, []).collect();

    let to = &mut stacks[movement.to as usize];

    to.append(&mut to_move);
}

pub fn apply_movements_9001(stacks: &mut Stacks, movements: &Vec<Movement>) {
    for movement in movements {
        apply_movement_9001(stacks, movement)
    }
}
