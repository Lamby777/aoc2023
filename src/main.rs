//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn find_part_numbers(input: Vec<Option<&str>>) -> Vec<u32> {
    todo!()
}

fn main() {
    let input = inputfile!("3.txt");

    let lines: Vec<_> = std::iter::once(None)
        .chain(input.lines().map(Some))
        .chain(std::iter::once(None))
        .collect();

    let sum = find_part_numbers(lines).iter().sum::<u32>();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //
}
