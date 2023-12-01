use std::collections::VecDeque;

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

fn mem_is_number(slice: &[char]) -> Option<u32> {
    let word = slice.iter().collect::<String>();
    for (k, v) in NUMBERS {
        if word.starts_with(k) {
            return Some(v);
        }
    }

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
            let vec_operation = if forward {
                VecDeque::push_front
            } else {
                VecDeque::push_back
            };

            vec_operation(&mut mem, ch);

            mem.make_contiguous();
            mem_is_number(mem.as_slices().0)
        });

        if let Some(n) = res {
            mem.clear();
            return res;
        }
    }

    None
}

fn parse_line(line: &str) -> u32 {
    let mut chars = line.chars();

    let n_first = consume_num(true, &mut chars).unwrap();
    let n_last = consume_num(false, &mut chars.rev()).unwrap_or(n_first);

    (n_first * 10) + n_last
}
