# wordle

Wordle in the terminal, written in Rust, with a small bot that can solve the puzzles.

## Bot results

Over 10,000 simulated games:

| Wins | Loses | Average guesses |
|---|---|---|
| **9724 (97.2%)** | **276 (2.8%)** | **3.91** |

## How the bot works

1. The first guess is always `crane`.
2. After each guess, the bot looks at the colors: green letters (right letter, right place), yellow letters (right letter, wrong place) and grey letters (not in the word).
3. It goes through all 2,314 possible solutions and removes every word that doesn't fit these colors.
4. From the words that are left, it picks one randomly as the next guess.

The bot is simple. A better version would not pick randomly, but choose the word that removes the most other words.

## Run

Start it from the project folder, because the word lists (`words.txt`, `solutions.txt`) are loaded from there.

```sh
cargo run --release
```

By default this simulates 10,000 games with the bot and prints statistics. If you want to play yourself, call `play_normal_game()` instead of `start_bot()` in `main()` (`src/main.rs`).

## Files

```
src/game.rs      game logic (checking guesses, colors, board)
src/bot.rs       the bot
src/main.rs      game loop and bot simulation with statistics
words.txt        all 14,855 allowed guesses
solutions.txt    all 2,314 possible solutions
```

## What I learned
- How to structure a small Rust project with modules
- How to filter a big word list with a few simple rules
- How to test a bot by simulating thousands of games

## Future improvements
There are a few things I would like to improve in the future.
I want to make the user experience a little bit better. Currently you have to change the code to switch between playing the game and running the bot simulation.
The bot could also be improved by choosing guesses based on information theory instead of picking randomly. This would allow it to get more information from each guess and solve puzzles in fewer guesses.

I learned about this from an interesting video:

https://www.youtube.com/watch?v=v68zYyaEmEA
