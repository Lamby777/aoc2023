//!
//! Day 2
//! Part 1
//!
//!  - &Cherry, 12/1/2023
//!

mod macros;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,

    picks: Vec<RGB>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        Self {
            id: todo!(),
            picks: todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RGB(u32, u32, u32);

fn main() {
    let input = inputfile!("2.txt");

    let lines = input.lines();
}

mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = "Game 1: 4 blue; 1 green, 2 red; 4 blue, 1 green, 6 red";
        let game = Game::from_line(line);

        assert_eq!(
            game,
            Game {
                id: 1,
                picks: vec![RGB(2, 1, 4), RGB(6, 1, 4)],
            }
        )
    }
}
