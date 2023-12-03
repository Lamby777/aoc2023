//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

const NUMBERS: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let input = inputfile!("3.txt");

    let lines: Vec<_> = std::iter::once(None)
        .chain(input.lines().map(Some))
        .chain(std::iter::once(None))
        .collect();

    // println!("{}", sum);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //
}
