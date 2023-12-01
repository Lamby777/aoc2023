use std::{collections::VecDeque, process::exit};

type NumberKVPair = (&'static str, u32);
const NUMBERS: [NumberKVPair; 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn run(input: &str) {
    let sum = input.lines().map(parse_line).sum::<u32>();
    println!("{}", sum);
}

fn mem_is_number<'a, T>(it: T) -> Option<u32>
where
    T: Iterator<Item = &'a char>,
{
    let word = it.collect::<String>();
    for (k, v) in NUMBERS {
        if word.starts_with(k) {
            println!("Found: {}", word);
            return Some(v);
        }
    }

    println!("{} is not a number", word);

    None
}

/// consume iterator items until a number is found
fn consume_num<T>(forward: bool, chars: &mut T) -> Option<u32>
where
    T: Iterator<Item = char>,
{
    let mut mem = VecDeque::new();

    while let Some(ch) = chars.next() {
        // parse the digit, or None if can't yet
        let res = ch.to_digit(10).or_else(|| {
            // if not a digit...
            mem.push_front(ch);

            mem.make_contiguous();
            mem_is_number(mem.iter())
        });

        if let Some(n) = res {
            println!("Clearing with: {}!", n);
            mem.clear();
            return res;
        }
    }

    mem.clear();
    None
}

fn parse_line(line: &str) -> u32 {
    // for testing, quit at ---
    if line == "---" {
        exit(0)
    }

    let mut chars = line.chars();

    let n_first = consume_num(true, &mut chars).unwrap();
    let n_last = consume_num(false, &mut chars).unwrap_or(n_first);

    (n_first * 10) + n_last
}
