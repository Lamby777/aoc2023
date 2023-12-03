//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

const WIDTH: usize = 140;

fn is_symbol(ch: char) -> bool {
    ch.is_digit(10) && ch != '.'
}

fn find_part_numbers(input: Vec<&str>) -> Vec<u32> {
    let res = vec![];

    // empty line iter used to make start/end follow
    // the same rules as the rest of the lines
    let literally_nothing = ".".repeat(WIDTH);

    for i in 0..input.len() {
        let current = input[i];

        let line_iter = |i: usize| {
            input
                .get(i)
                .map(|v| v.chars())
                .unwrap_or(literally_nothing.chars())
        };

        let previous = line_iter(i - 1);
        let next = line_iter(i + 1);

        for ch in current.chars() {
            //
        }
    }

    res
}

fn main() {
    let input = inputfile!("3.txt");

    let lines = input.lines().collect();
    let sum = find_part_numbers(lines).iter().sum::<u32>();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //
}
