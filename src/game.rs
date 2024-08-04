use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
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
    pub fn get_input(&mut self, prompt: &str) -> Option<String> {
        io::stdout().flush().unwrap();
        println!();
        print!("{}", prompt);

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if self.is_input_valid(input.trim().to_lowercase()) {
            Some(input)
        } else {
            None
        }
    }
    pub fn check_if_game_over(&self) -> bool {
        return self.guessed_words.last().unwrap() == &self.solution || self.guesses_left == 0;
    }

    pub fn set_input(&mut self, input: &String) {
        self.guessed_words.push(input.clone().trim().to_lowercase());
        self.guesses_left -= 1;
    }

    pub fn get_color(&self, inp: String) -> Vec<Color> {
        let mut result = Vec::new();
        let solution_chars: Vec<char> = self.solution.chars().collect();

        for (i, char) in inp.chars().enumerate() {
            if solution_chars[i] == char {
                result.push(Color::Green)
            } else if self.solution.contains(char) {
                // Todo: handle double yellow letters -> if there are 2 e in the guess and 1 e in the solution only 1 e should be yellow, if there are 2 then both are yellow
                result.push(Color::Yellow)
            } else {
                result.push(Color::Grey)
            }
        }
        result
    }
    pub fn print(&self) {
        for word in self.guessed_words.clone() {
            let colors = self.get_color(word);
            println!("Colors: {:?}", colors);
        }
    }
}

pub fn read_file(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let contents = contents.lines().map(|line| line.to_string()).collect();
    contents
}
