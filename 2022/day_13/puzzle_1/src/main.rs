use std::{
    error::Error,
    fs, str::Chars, iter::Peekable
};

#[derive(Debug)]
enum Value {
    Value(i32),
    List(Vec<Value>)
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

    fn parse(itr: &mut Chars) -> Vec<Value> {

        let mut value = Vec::new();

        while let Some(c) = itr.next() {
            match c {
                '[' => {
                    value.push(Value::List(Value::parse(itr)));
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_13/input_simple.txt")?;

    for line in input.lines() {
        println!("");
        if line.len() < 3 {
            continue;
        }
        let mut chars = line.chars();
        chars.next();
        let val = Value::parse(&mut chars);
        println!("{val:?}");
    }


    Ok(())
}
