mod card;
mod deck;
mod hand;

// imports
use std::io::{self, stdin};

fn main() {
    // declare vars
    let mut play_again: bool = true;
    let mut input: String = String::new();
    let mut bet: u32 = 0;
    let mut bankroll: u32 = 1000;
    let mut iteration: u16 = 0;
    // greet player
    println!("Welcome! You have a starting bankroll of $1000");

    // create play-loop
    while play_again {
        // decide if player should be greeted
        if iteration != 0 {
            println!("Welcome! Current bankroll: ${}", bankroll);
        }

        // betting loop
        // prompt player
        loop {
            print!("Enter your bet amount: $");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read bet");

            // attempt parse
            bet = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That was not a number!");
                    continue
                }
            }
        }
    }
}
