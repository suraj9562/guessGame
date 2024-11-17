use rand::Rng;
use std::io;

fn begin_banner() {
    println!("============================================================");
    println!("Hello, Folks! 👋 Welcome to the Guess Game 🎮 !!!");
    println!("Steps to play the game:");
    println!("1. Start the game.");
    println!("2. You will see a predefined list of fruits.");
    println!("3. The game will randomly select a fruit from the list.");
    println!("4. Your task is to guess the randomly selected fruit.");
    println!("============================================================");
}

fn game_choice_banner() {
    println!("============================================================");
    println!("Choose an option:");
    println!("1. Press 1 to Play 🎮");
    println!("2. Press 2 to Exit ❌");
    println!("============================================================");
}

fn game() {
    let fruits: Vec<&str> = vec!["apple", "orange", "banana", "papaya", "pear", "avocado"];
    println!("fruits: {:?}", fruits);

    let guess_index: usize = rand::thread_rng().gen_range(0..=5);
    println!("fruit guessed by system: {}", fruits[guess_index]);
    let mut turns: usize = 0;

    loop {
        let mut input: String = String::new();
        let mut fruit_name: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                fruit_name = input.trim().to_lowercase().to_string(); //input.trim() returns &str
                println!("Selected Fruit: {}", fruit_name);

                if !fruits.contains(&fruit_name.as_str()) {
                    println!("Invalid fruit 🔴! Please enter a valid fruit");
                    continue;
                }
            }
            Err(error) => {
                eprintln!("Error:{}", error);
            }
        }

        if &fruit_name == fruits[guess_index] {
            println!("Congratulations 💐!, you have successfully guessed the fruit");
            println!("You did it in {turns} turns.");
            break;
        }

        println!("Oops! You missed the guess. Please try again.");
        turns += 1;
    }
}

fn main() {
    begin_banner();

    loop {
        game_choice_banner();

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read");

        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };

        match guess {
            1 => game(),
            2 => {
                println!("Terminating the game...");
                return;
            }
            _ => {
                println!("INVALID INPUT 🔴, Terminating the game...");
                return;
            }
        }
    }
}
