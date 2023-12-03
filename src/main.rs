//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

const WIDTH: usize = 140;

fn is_symbol(ch: char) -> bool {
    !(ch.is_digit(10) || ch == '.')
}

fn num_prefix(input: &str) -> String {
    input
        .chars()
        .take_while(|v| v.to_digit(10).is_some())
        .collect::<String>()
}

fn find_part_numbers(input: Vec<&str>) -> Vec<u32> {
    let mut part_numbers = vec![];

    // empty line iter used to make start/end follow
    // the same rules as the rest of the lines
    let literally_nothing = ".".repeat(WIDTH);

    for i in 0..input.len() {
        let line = input[i];

        let line_factory = |i: usize| {
            input
                .get(i)
                .map(|v| v.chars())
                .unwrap_or(literally_nothing.chars())
                .map(is_symbol)
                .collect::<Vec<_>>()
        };

        let previous = line_factory(i - 1);
        let next = line_factory(i + 1);

        // the last remembered index of where a
        // number started
        let mut is_part = false;
        let mut num_index = 0;

        let line_chars = line.chars().chain(std::iter::once('.'));
        for (i, ch) in line_chars.enumerate() {
            if !ch.is_digit(10) && is_part {
                // parse the number
                let num = num_prefix(&line[num_index..]);
                let num = num.parse::<u32>().unwrap();
                part_numbers.push(num);
            }

            match ch {
                '.' => is_part = false,
                _ if is_symbol(ch) => is_part = true,
                _ => num_index = i,
            }
        }
    }

    part_numbers
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
