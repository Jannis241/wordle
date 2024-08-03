use crate::game::Wordle;

mod game;

fn main() {
    let mut wordle = Wordle::new();
    loop {
        wordle.get_input();
    }
}
