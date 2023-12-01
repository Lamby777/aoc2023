use std::collections::VecDeque;

pub fn run(input: &str) {
    let sum = input.lines().map(parse_line).sum::<u32>();
    println!("{}", sum);
}

fn mem_is_number(deque: &[char]) -> bool {
    true
}

fn parse_line(line: &str) -> u32 {
    let mut chars = line.chars();

    let mem = VecDeque::new();
    let n_first = chars.take_while(|ch| {
        let res = if let Some(n) = ch.to_digit(10) {
            n
        } else {
            mem.push_front(*ch);

            mem.make_contiguous();
            if mem_is_number(mem.as_slices().0) {
                //
            } else {
                return true;
            }
        };

        mem.clear();
        false;
    });

    let n_last = chars.next_back().map(char_to_digit);
    let n_last = n_last.unwrap_or(n_first);

    (n_first * 10) + n_last
}
