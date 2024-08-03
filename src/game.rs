use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::process::exit;

use rand::seq::SliceRandom;
use rand::thread_rng;

enum Color {
    Green,
    Yellow,
    Grey,
}

pub struct Wordle {
    solution: String,
    pub guesses_left: i32,
    guessed_words: Vec<String>,
}

impl Wordle {
    pub fn new() -> Self {
        let solution = Wordle::generate_solution();
        println!("The solution is: {}", solution);
        Wordle {
            solution,
            guesses_left: 5,
            guessed_words: Vec::new(),
        }
    }

    fn generate_solution() -> String {
        let solutions = read_file("./solutions.txt");
        let mut rng = thread_rng();
        let word = solutions.choose(&mut rng).unwrap();

        return word.clone().trim().to_lowercase();
    }
    fn is_input_valid(&self, input: String) -> bool {
        let words: Vec<String> = read_file("./words.txt")
            .iter()
            .map(|word| word.trim().to_lowercase())
            .collect();
        let solutions: Vec<String> = read_file("./solutions.txt")
            .iter()
            .map(|word| word.trim().to_lowercase())
            .collect();

        return words.contains(&input) || solutions.contains(&input);
    }
    pub fn get_input(&mut self) {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if self.is_input_valid(input.trim().to_lowercase()) {
            self.guessed_words.push(input.trim().to_lowercase());
            if input.trim().to_lowercase() == self.solution {
                self.print();
                exit(0);
            }
            self.guesses_left -= 1;
            self.print();
            if self.guesses_left == 0 {
                println!("The solution was {}", self.solution);
                exit(0);
            }
        } else {
            println!("Invalid word..");
        }
    }

    pub fn get_color(c: char) -> Color {
        return Color::Grey;
    }
    pub fn print(&self) {}
}

pub fn read_file(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let contents = contents.lines().map(|line| line.to_string()).collect();
    contents
}
