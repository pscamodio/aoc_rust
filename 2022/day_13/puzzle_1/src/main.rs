use std::{
    error::Error,
    fs, str::Chars, fmt::Display
};

#[derive(Debug, PartialEq)]
enum Value {
    Value(i32),
    List(Vec<Value>)
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Value(a), Value::Value(b)) => a.partial_cmp(b),
            (Value::List(a), Value::List(b)) => a.partial_cmp(b),
            (Value::Value(a), Value::List(b)) => vec![Value::Value(*a)].partial_cmp(&b),
            (Value::List(a), Value::Value(b)) => a.partial_cmp(&vec![Value::Value(*b)]),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(x) => write!(f, "{x}"),
            Value::List(x) => {
                let res : Vec<String> = x.iter().map(|x| x.to_string()).collect();
                let res = res.join(",");
                write!(f, "[{res}]")
            }
        }   
    }
}

impl Value {
    fn parse_number(first: char, itr: &mut Chars) -> (i32, bool) {
        let mut str = first.to_string();
        let mut array_end = false;
        for c in itr {
            match c {
                ']' => {
                    array_end = true;
                    break;
                },
                ',' => {
                    array_end = false;
                    break;
                }
                x => str.push(x),
            }
        }
        (str.parse().unwrap(), array_end)
    }

    fn parse_list(itr: &mut Chars) -> Vec<Value> {

        let mut value: Vec<Value> = Vec::new();

        while let Some(c) = itr.next() {
            match c {
                '[' => {
                    value.push(Value::List(Value::parse_list(itr)));
                },
                ',' => {},
                ']' => {
                    break;
                }
                _ => {
                    let (num, end) = Value::parse_number(c, itr);
                    value.push(Value::Value(num));
                    if end {
                        break;
                    }
                }
            }
        }

        value
    }

    fn parse(itr: &mut Chars) -> Value {
        let first = itr.next().unwrap();

        if first != '[' {
            Value::Value(Value::parse_number(first, itr).0)
        } else {
            Value::List(Value::parse_list(itr))
        }

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_13/input_simple.txt")?;

    let mut sum = 0;

    for (idx, block) in input.split("\n\n").enumerate() {
        let mut lines = block.lines();
        let val1 = lines.next().map(|x| Value::parse(&mut x.chars())).unwrap();
        let val2 = lines.next().map(|x| Value::parse(&mut x.chars())).unwrap();
        println!("{}", val1);
        println!("{}", val2);
        println!("a < b = {}", val1 < val2);
        if val1 < val2 {
            sum += idx + 1;
        }
    }


    Ok(())
}
