//! # User Interface
//!
//! Methods from this module prompt, gain information from, and display information to the user.
//! This module is meant to process and validate inputs, then pass them off.

use crate::hand::Hand;
use colored::Colorize;
use std::io::{self, Write};

/// Displays the header with current bankroll
pub fn display_header(i: u16, bankroll: u32) {
    // \x1B[2J clears screen, \x1B[1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    let title = if i == 0 {
        "WELCOME TO BLACKJACK"
    } else {
        "WELCOME BACK"
    };

    println!("{}", "┌──────────────────────────────┐".cyan());
    print!("{}", "│".cyan().bold());
    print!(" {:^28} ", title.cyan().bold());
    println!("{}", "│".cyan().bold());
    println!("{}", "├──────────────────────────────┤".cyan());
    print!("{}", "│".cyan().bold());
    print!(
        "   Current Bankroll: ${:<7} ",
        bankroll.to_string().green().bold()
    );
    println!("{}", "│".cyan().bold());
    println!("{}", "└──────────────────────────────┘".cyan());
    println!();
}

/// Prompts user for bet, then obtains and validates the bet
pub fn get_bet(bankroll: u32) -> u32 {
    loop {
        let mut input: String = String::new();
        let bet: u32;

        print!("Enter your bet amount: $");

        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read bet");

        bet = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number!\n");
                continue;
            }
        };

        if bet > bankroll {
            println!("Bad bet. Insufficient funds\n");
            continue;
        }
        if bet == 0 {
            println!("You can't play for free ;)\n");
            continue;
        }

        return bet;
    }
}

/// Show player and dealer hands
pub fn show_hands(player_hand: &Hand, dealer_hand: &Hand) {
    println!(
        "Dealer: {} ({})    Player: {} ({})\n",
        dealer_hand,
        dealer_hand.value(),
        player_hand,
        player_hand.value()
    );
}

/// Prompt user for hit or stand decision. Returns true for hit, false for stand
pub fn player_hits() -> bool {
    loop {
        let mut input = String::new();

        print!("Would you like to (h)it or (s)tand?: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "h" | "hit" => return true,
            "s" | "stand" => return false,
            _ => println!("\n**Invalid Input**\n"),
        }
    }
}

/// Print outcome message, either win or loss, with amount
pub fn print_outcome(won: bool, amount: u32) {
    let msg = if won { " YOU WIN!" } else { "YOU LOSE" };

    let colorize = |s: String| -> String {
        if won {
            s.green().to_string()
        } else {
            s.red().to_string()
        }
    };

    println!(
        "{}",
        colorize("┌──────────────────────────────┐".to_string())
    );

    let msg_line = format!("│          {:<12}        │", msg);
    println!("{}", colorize(msg_line));

    let label = if won { "Payout" } else { "  Loss" };
    let money_line = format!("│        {}: ${:<5}        │", label, amount);
    println!("{}", colorize(money_line));

    println!(
        "{}",
        colorize("└──────────────────────────────┘".to_string())
    );
}

/// Print push message
pub fn print_push() {
    let msg: &str = "   PUSH   ";

    let colorize = |s: String| -> String { s.blue().to_string() };

    println!(
        "{}",
        colorize("┌──────────────────────────────┐".to_string())
    );

    let msg_line = format!("│          {:<12}        │", msg);
    println!("{}", colorize(msg_line));

    let money_line = format!("│          MONEY BACK          │");
    println!("{}", colorize(money_line));

    println!(
        "{}",
        colorize("└──────────────────────────────┘".to_string())
    );
}

/// Blackjack message. Print blackjack message with payout amount
pub fn print_blackjack(amount: u32) {
    let msg = "BLACKJACK!";

    let colorize = |s: String| -> String { s.green().to_string() };

    println!(
        "{}",
        colorize("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$".to_string())
    );

    let msg_line = format!("$          {:<12}        $", msg);
    println!("{}", colorize(msg_line));

    let label = "Payout";
    let money_line = format!("$        {}: ${:<5}        $", label, amount);
    println!("{}", colorize(money_line));

    println!(
        "{}",
        colorize("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$".to_string())
    );
}

/// Prompt user to play again, returns true for yes, false for no
pub fn play_again(bankroll: u32) -> bool {
    loop {
        if bankroll == 0 {
            println!(
                "{}",
                "\n\nYou are out of money! You are not useful to us anymore.\n\n"
                    .red()
                    .bold()
            );
            return false;
        }

        let mut input = String::new();

        print!("\n\nPlay again? (y/n): ");

        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user decision");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid Input"),
        }
    }
}
