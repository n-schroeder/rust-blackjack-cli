//! User Interface
//! 
//! I will call methods from this module to prompt, gain information from, and display information to the user.
//! This module is meant to process and validate inputs and pass it off.

// Imports
use colored::Colorize;

/// Clears screen and welcomes user based on iteration and bankroll
fn display_header(game: &Game) {
    // Clear screen
    // \x1B[2J clears screen, \x1B[1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    // Choose the title based on iteration
    let title = if game.iteration == 0 { "WELCOME TO BLACKJACK" } else { "WELCOME BACK" };

    // Print the Box
    // formatting: {:^28} centers the text in a 28-char wide space
    println!("{}", "┌──────────────────────────────┐".cyan());
    print!("{}", "│".cyan().bold());
    print!(" {:^28} ", title.cyan().bold());
    println!("{}", "│".cyan().bold());
    println!("{}", "├──────────────────────────────┤".cyan());
    print!("{}", "│".cyan().bold());
    print!("   Current Bankroll: ${:<7} ", game.bankroll.to_string().green().bold());
    println!("{}", "│".cyan().bold());
    println!("{}", "└──────────────────────────────┘".cyan());
    println!();
}
