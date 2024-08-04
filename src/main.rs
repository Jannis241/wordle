use std::process::exit;

use crate::game::Wordle;

mod game;

fn main() {
    let mut wordle = Wordle::new();
    loop {
        let input = wordle.get_input("Your guess:");

        if let Some(input) = input {
            wordle.set_input(&input);
            wordle.print();
            if wordle.check_if_game_over() {
                exit(0);
            }
        } else {
            println!("Invalid word..")
        }
    }
}
