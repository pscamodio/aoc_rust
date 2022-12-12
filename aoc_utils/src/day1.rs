
pub fn compute_elf_snack_weights(lines: &Vec<String>) -> Vec<i32> {
    // Parse all numbers to int
    let contents = lines.iter().map(|x| match x.is_empty() {
        true => None,
        false => Some(x.trim().parse::<i32>().expect("not a number")),
    });

    // Reduce to list of sums
    contents.fold(Vec::from([0]), |mut acc, x| match x {
        Some(x) => {
            let val = acc.pop().expect("Can't be empty");
            acc.push(val + x);
            return acc;
        }
        None => {
            acc.push(0);
            return acc;
        }
    })
}
