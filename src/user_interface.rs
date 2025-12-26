//! User Interface
//! 
//! I will call methods from this module to prompt, gain information from, and display information to the user.
//! This module is meant to process and validate inputs and pass it off.

// Imports
use colored::Colorize;
use std::io::{self};

/// Clears screen and welcomes user based on iteration and bankroll
pub fn display_header(i: u16, bankroll: u32) {
    // Clear screen
    // \x1B[2J clears screen, \x1B[1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    // Choose the title based on iteration
    let title = if i == 0 { "WELCOME TO BLACKJACK" } else { "WELCOME BACK" };

    // Print the Box
    // formatting: {:^28} centers the text in a 28-char wide space
    println!("{}", "┌──────────────────────────────┐".cyan());
    print!("{}", "│".cyan().bold());
    print!(" {:^28} ", title.cyan().bold());
    println!("{}", "│".cyan().bold());
    println!("{}", "├──────────────────────────────┤".cyan());
    print!("{}", "│".cyan().bold());
    print!("   Current Bankroll: ${:<7} ", bankroll.to_string().green().bold());
    println!("{}", "│".cyan().bold());
    println!("{}", "└──────────────────────────────┘".cyan());
    println!();
}

/// Prompts user for bet, then obtains and validates the bet
pub fn get_bet(bankroll:u32) -> u32 {
    // start betting loop
    loop {
        // declare input var
        let mut input: String = String::new();
        let bet: u32;
        // prompt player
        print!("Enter your bet amount: $");
        // real input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read bet");

        // attempt parse
        bet = match input.trim().parse() {
            // if result is good, return num
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number!");
                continue
            }
        };

        // verify valid bet
        if bet > bankroll {
            println!("Bad bet. Insufficient funds");
            continue
        }
        if bet == 0 {
            println!("You can't play for free ;)");
            continue
        }
        if bet == 67 {
            println!("chud");
        }
        
        // exit betting loop
        return bet;
    }
}