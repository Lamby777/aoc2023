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
        let line_factory = |offset: i64| {
            let i = i as i64 - offset;
            // what the actual fuck am i doing with my life
            if i >= 0 {
                input[i as usize]
            } else {
                &literally_nothing
            }
            .chars()
            .map(is_symbol)
            .collect::<Vec<_>>()
        };

        // stream of booleans to be processed along the line
        // to see which chars are "adjacent" to symbols
        let tape: Vec<bool> = {
            // we pwayin siwwy hewe :3
            let lines = (-1..=1).map(line_factory).collect::<Vec<_>>();

            // goofy ahh code
            lines[0]
                .iter()
                .zip(lines[1].iter())
                .zip(lines[2].iter())
                .map(|((&a, &b), &c)| a || b || c)
                .collect()
        };

        // the last remembered index of where a
        // number started
        let mut is_part = false;
        let mut num_index = 0;
        let mut last_ch = ' ';

        let line = input[i];
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

                // start of new number
                _ if !last_ch.is_digit(10) => num_index = i,
                _ => (),
            }

            last_ch = ch;
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
