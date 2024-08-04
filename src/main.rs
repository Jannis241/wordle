use std::process::exit;

use crate::game::*;

mod engine;
mod game;

fn normal_game() {
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

fn solver() {
    let mut wordle = Wordle::new();

    loop {
        let word = engine::get_best_word(&wordle, 5);
        wordle.set_input(&word);
        wordle.print();
        if wordle.check_if_game_over() {
            exit(0);
        }
    }
}

fn main() {
    //normal_game();
    solver();
}
