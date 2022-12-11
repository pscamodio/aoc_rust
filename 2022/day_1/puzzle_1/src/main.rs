use aoc_utils::read_input_lines;

fn main() {
    // Split by lines
    let lines = read_input_lines("2022/day_1/input.txt");

    // Parse all numbers to int
    let contents = lines.iter().map(|x| match x.is_empty() {
        true => None,
        false => Some(x.trim().parse::<i32>().expect("not a number"))
    });

    // Reduce to list of sums
    let contents = contents.fold(Vec::from([0]), |mut acc, x| match x {
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

    // Compute max value of array
    let max = contents.iter().max().expect("Should have a max");

    println!("With text:\n{max:?}");
}
