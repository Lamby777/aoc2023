//!
//! Day 3
//! Part 1
//!
//!  - &Cherry
//!

mod macros;

const WIDTH: usize = 140;

// as it turns out, using a Vec<Vec<char>> is actually
// harder to work with than just Vec<char> in this case

fn find_part_numbers(input: Vec<char>) -> Vec<u32> {
    let res = vec![];

    for (i, v) in input.iter().enumerate() {
        //
    }

    res
}

fn main() {
    let input = inputfile!("3.txt");

    // split into a Vec<Vec<char>>
    let lines = input.lines().flat_map(str::chars).collect();

    let sum = find_part_numbers(lines).iter().sum::<u32>();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //
}
