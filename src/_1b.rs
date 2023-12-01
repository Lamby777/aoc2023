use std::collections::VecDeque;

pub fn run(input: &str) {
    let sum = input.lines().map(parse_line).sum::<u32>();
    println!("{}", sum);
}

fn mem_is_number(deque: &[char]) -> Option<u32> {
    None
}

fn parse_line(line: &str) -> u32 {
    let mut chars = line.chars();

    let mut mem = VecDeque::new();
    let n_first = chars.take_while(|ch| {
        // parse the digit, or None if can't yet
        let res = ch.to_digit(10).or_else(|| {
            // if not a digit...
            mem.push_front(*ch);

            mem.make_contiguous();
            mem_is_number(mem.as_slices().0)
        });

        if let Some(n) = res {
            mem.clear();
            true
        } else {
            false
        }
    });

    let n_last = chars.next_back().map(char_to_digit);
    let n_last = n_last.unwrap_or(n_first);

    (n_first * 10) + n_last
}
