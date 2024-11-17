# Guess Game 🎮

Welcome to the **Guess Game**! This interactive CLI-based game challenges you to guess a randomly selected fruit from a predefined list. It's simple, fun, and a great way to learn Rust while engaging with user input and random number generation.

---

## How to Play 🚀

1. **Start the Game**: Run the program, and you'll see a menu with options.
2. **Choose an Option**:
   - Press `1` to start playing.
   - Press `2` to exit the game.
3. **Game Rules**:
   - You'll see a list of fruits displayed.
   - The game will randomly pick one fruit from the list.
   - You need to guess the selected fruit by typing its name.
   - If you guess wrong, you’ll get a chance to try again.
   - Keep guessing until you get it right!
4. **Win the Game**: When you correctly guess the fruit, the game will congratulate you and display the number of attempts you took.

---

## Example Output

```plaintext
============================================================
Hello, Folks! 👋 Welcome to the Guess Game 🎮 !!!
Steps to play the game:
1. Start the game.
2. You will see a predefined list of fruits.
3. The game will randomly select a fruit from the list.
4. Your task is to guess the randomly selected fruit.
============================================================
============================================================
Choose an option:
1. Press 1 to Play 🎮
2. Press 2 to Exit ❌
============================================================
fruits: ["apple", "orange", "banana", "papaya", "pear", "avocado"]
Selected Fruit: orange
Invalid fruit 🔴! Please enter a valid fruit
Oops! You missed the guess. Please try again.
Congratulations 💐! You successfully guessed the fruit.
You did it in 3 turns.
```

---

## Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) programming language (stable version recommended).

---

## How to Run the Program

1. Clone the repository or copy the source code into a file named `main.rs`.
2. Open a terminal and navigate to the directory containing the `main.rs` file.
3. Compile and run the program:
   ```bash
   cargo build
   cargo run
   ```

---

## Features

- **Random Selection**: Uses the `rand` crate for generating random indices.
- **User Interaction**: Accepts and validates user input in a loop.
- **Custom Banners**: Adds flair to the game experience with friendly banners and separators.

---

## Dependencies

- `rand`: For generating random indices.

Add it to your project by including the following in your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

---

## Acknowledgments

This program is a beginner-friendly project to explore:

- User input handling
- Loops and conditionals
- String manipulation
- Basic Rust programming concepts

Have fun playing the **Guess Game**! 🎉
