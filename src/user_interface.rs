//! User Interface
//! 
//! I will call methods from this module to prompt, gain information from, and display information to the user.
//! This module is meant to process and validate inputs and pass it off.

// Imports
use crate::hand::Hand;
use colored::Colorize;
use std::io::{self, Write};

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
        // read input
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read bet");
        // attempt parse
        bet = match input.trim().parse() {
            // if result is good, return num
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number!\n");
                continue
            }
        };

        // verify valid bet
        if bet > bankroll {
            println!("Bad bet. Insufficient funds\n");
            continue
        }
        if bet == 0 {
            println!("You can't play for free ;)\n");
            continue
        }

        // exit betting loop
        return bet;
    }
}

pub fn show_hands(player_hand: &Hand, dealer_hand: &Hand) {
    println!("\nDealer: {} ({})    Player: {} ({})\n",
        dealer_hand, dealer_hand.value(),
        player_hand, player_hand.value());
}

pub fn player_hits() -> bool {
    loop {
        // declare input var
        let mut input = String::new();
        // prompt user
        print!("Would you like to (h)it or (s)tand?: ");
        // flush and read user decision
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        // match input with user decision
        match input.trim().to_lowercase().as_str() {
            "h" | "hit" => return true,
            "s" | "stand" => return false,
            _ => println!("\n**Invalid Input**\n"),
        }
    }
}

// show outcome message
pub fn print_outcome(won: bool, amount: u32) {
    // define the text and color based on the result
    let msg = 
    if won { "YOU WIN!" }
    else { "YOU LOSE" };

    // helper closure to apply the color dynamically
    // allows us to apply "red" or "green" to the whole string at once
    let colorize = |s: String| -> String {
        if won { s.green().to_string() } else { s.red().to_string() }
    };

    // print the Box
    // We format the string FIRST, then colorize the whole thing.
    println!("{}", colorize("┌──────────────────────────────┐".to_string()));
    
    // The Message Row
    let msg_line = format!("│          {:<12}        │", msg);
    println!("{}", colorize(msg_line));

    // The Money Row (Using your {:<5} spacing!)
    // If won, we show "Payout", if lost, we show "Loss"
    let label = if won { "Payout" } else { "  Loss" }; // padded to match length
    let money_line = format!("│        {}: ${:<5}        │", label, amount);
    println!("{}", colorize(money_line));

    println!("{}", colorize("└──────────────────────────────┘".to_string()));
}

// print push message
pub fn print_push_message() {

}

// prompt user to play again
pub fn play_again(bankroll: u32) -> bool {
    loop {
        // check balance
        if bankroll == 0 {
            println!("{}", "\n\nYou are out of money! You are not useful to us anymore.\n\n".red().bold());
            return false;
        }
        // declare input var
        let mut input = String::new();

        // prompt user
        print!("\n\nPlay again? (y/n): ");

        // flush stdout
        io::stdout().flush().expect("Failed to flush stdout");
        // read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user decision");

        // match input with answer and decision
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid Input"),
        }
    }
}