use std::collections::HashMap;
use std::iter::zip;

use crate::game::*;

// Adjust import based on your actual module setup

pub fn get_best_word(wordle: &Wordle, anzahl: i32) -> String {
    let mut greens: HashMap<char, Vec<usize>> = HashMap::new(); // char | vec(index)
    let mut yellows: HashMap<char, Vec<usize>> = HashMap::new(); // char | vec(index)
    let mut greys: HashMap<char, usize> = HashMap::new(); // char | count

    for word in &wordle.guessed_words {
        let colors = wordle.get_color(&word);
        for (index, (ch, col)) in zip(word.chars(), colors).enumerate() {
            match col {
                Color::Grey => {
                    let counter = greys.entry(ch).or_insert(0);
                    *counter += 1;
                }
                Color::Green => {
                    let indeces = greens.entry(ch).or_insert(Vec::new());
                    indeces.push(index);
                }
                Color::Yellow => {
                    let indeces = yellows.entry(ch).or_insert(Vec::new());
                    indeces.push(index);
                }
            }
        }
    }
    println!("greens: {:?}", greens);
    println!("yellows: {:?}", yellows);
    println!("greys: {:?}", greys);

    let mut solutions = read_file("./solutions.txt");
    let mut possible_solutions = Vec::new();

    for solution in &solutions {
        // Skip if it's one of the guessed words
        if wordle.guessed_words.contains(solution) {
            continue;
        }

        let mut solution_chars: Vec<char> = solution.chars().collect();

        // Check green constraints
        let mut has_greens = true;
        for (ch, indeces) in &greens {
            for &index in indeces {
                if solution_chars.get(index) != Some(&ch) {
                    has_greens = false;
                    break;
                }
            }
            if !has_greens {
                break;
            }
        }

        // Check yellow constraints
        let mut has_yellows = true;
        for (ch, indeces) in &yellows {
            // Check if the character is not in any of the positions where it was yellow
            if !solution.contains(*ch) {
                has_yellows = false;
                break;
            }
            for &index in indeces {
                if solution_chars.get(index) == Some(&ch) {
                    has_yellows = false;
                    break;
                }
            }
            if !has_yellows {
                break;
            }
        }

        // Check grey constraints
        let mut contains_greys = true;
        let mut grey_count = HashMap::new();
        for ch in solution_chars.iter() {
            let counter = grey_count.entry(*ch).or_insert(0);
            *counter += 1;
        }
        for (ch, count) in &greys {
            if grey_count.get(ch).unwrap_or(&0) >= count {
                contains_greys = false;
                break;
            }
        }

        if has_greens && has_yellows && contains_greys {
            possible_solutions.push(solution.to_string());
        }
    }
    // Todo: grey letters funktioniert noch nicht so ganz mit den double letters
    // Todo: handle yellow letters falls sie zu grünen wurden
    println!("got {} possible solutions", possible_solutions.len());

    if wordle.guessed_words.is_empty() {
        return String::from("crane");
    }

    possible_solutions.first().unwrap().to_string()
}
