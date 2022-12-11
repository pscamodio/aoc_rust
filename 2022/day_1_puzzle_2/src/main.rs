use std::fs;

fn main() {
    let file_path = "./input.txt";
    // --snip--
    println!("In file {}", file_path);

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Split by lines
    let contents = contents.split("\n");

    // Parse all numbers to int
    let contents = contents.map(|x| match x.is_empty() {
        true => None,
        false => Some(x.trim().parse::<i32>().expect("not a number"))
    });

    // Reduce to list of sums
    let mut contents = contents.fold(Vec::from([0]), |mut acc, x| match x {
        Some(x) => {
            let val = acc.pop().expect("Can't be empty");
            acc.push(val + x);
            return acc;
        },
        None => {
            acc.push(0);
            return acc;
        }
    });

    // Sort the array
    contents.sort_unstable_by(|a, b|  b.cmp(a));

    // Pick top three
    let best3 = &contents[0..3];

    // Sum them
    let best : i32 = Vec::from(best3).iter().sum();

    println!("With text:\n{best:?}");
}
