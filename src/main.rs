//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

// could be one vec since the width is the same for all lines
// ... but who gives a shit, we're playing for fun, not milliseconds :)
type Grid<T> = Vec<Vec<T>>;

fn find_part_numbers(input: Grid<char>) -> Vec<u32> {
    let res = vec![];

    for (i, v) in input.iter().enumerate() {
        //
    }

    res
}

fn main() {
    let input = inputfile!("3.txt");

    // split into a Vec<Vec<char>>
    let lines = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
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
