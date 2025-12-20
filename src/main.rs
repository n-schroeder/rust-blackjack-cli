mod card;
mod deck;
mod hand;

// imports
use std::io::{self, Write};
use crate::{deck::Deck, hand::Hand};
use colored::Colorize;

fn main() {
    // declare vars
    let mut bet: u32;
    let mut bankroll: u32 = 1000;
    let mut iteration: u16 = 0;
    // greet player
    display_header(bankroll, iteration);

    // create play-loop
    'game_session: loop {
        // decide if player should be greeted
        if iteration != 0 {
            display_header(bankroll, iteration);
        }

        // betting loop
        'betting: loop {
            // declare input var
            let mut input: String = String::new();
            // prompt player
            print!("Enter your bet amount: $");
            // flush and read input
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read bet");

            // attempt parse
            bet = match input.trim().parse() {
                // if result is good, return num
                Ok(num) => num,
                Err(_) => {
                    println!("That was not a number!");
                    continue 'betting
                }
            };

            // verify valid bet
            if bet > bankroll {
                println!("Bad bet. Insufficient funds");
                continue 'betting
            }
            if bet == 0 {
                println!("You can't play for free ;)");
                continue 'betting
            }
            if bet == 67 {
                println!("chud");
            }
            // exit betting loop
            break 'betting
        }

        // game logic
        // create two player hands
        let mut player = Hand::new();
        let mut dealer = Hand::new();

        // create + shuffle deck
        let mut deck = Deck::new();
        deck.shuffle();

        // deal cards to player
        player.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
        player.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
        // deal upcard to dealer
        dealer.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
        let downcard = deck.deal();

        // show hands
        println!("\nDealer: {} ({})    Player: {} ({})\n", dealer, dealer.value(), player, player.value());

        // check for player blackjack
        if player.is_blackjack() {
            // player blackjack && !dealer blackjack
            if !dealer.is_blackjack() {
                println!();
                let blackjack_payout: u32 = (1.5 * bet as f64) as u32;
                print_outcome(true, blackjack_payout);
                bankroll += blackjack_payout;
                
                // game loop decision
                if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
                continue 'game_session
            }
            // player blackjack && dealer blackjack
            else {
                println!("Unlucky! Dealer has blackjack as well. Push");

                // game loop decision
                if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
                continue 'game_session
            }
        }

        // player hit/stand loop
        'player_turn: loop {
            // declare input var
            let mut input = String::new();
            // prompt user
            print!("Would you like to (h)it or (s)tand?: ");

            // flush and read user decision
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .unwrap();

            // match decision with hit or stand
            match input.trim().to_lowercase().as_str() {
                "h" | "hit" => {
                    // add card and show hands
                    player.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
                    println!("\nDealer: {} ({})    Player: {} ({})\n", dealer, dealer.value(), player, player.value());

                    // check for bust
                    if player.value() > 21 {
                        println!();
                        print_outcome(false, bet);
                        bankroll -= bet;
                        if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
                        continue 'game_session
                    }
                    // continue hit/stand loop
                    if player.value() <= 21 {
                        continue 'player_turn
                    }
                },
                // if player stands, continue game_session
                "s" | "stand" => { break 'player_turn },
                // invalid input
                _ => println!("Invalid Input"),
            }
        }
       
        // add downcard
        dealer.add_card(downcard.unwrap());
        
        // dealer turn
        println!("\n\n=== Dealer's Turn ===\n");
        // show hands
        println!("\n    Dealer: {} ({})    Player: {} ({})\n", dealer, dealer.value(), player, player.value());

        // check for dealer blackjack
        if dealer.is_blackjack() {
            println!();
            print_outcome(false, bet);
            bankroll -= bet;

            // play again?
            if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
            continue 'game_session
        }

        // dealer hit loop
        while dealer.value() < 17 {
            // deal card to dealer
            dealer.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
            
            // print
            println!("    Dealer hits...");
            println!("    Dealer: {} ({})    Player: {} ({})\n", dealer, dealer.value(), player, player.value());

            // handle dealer bust
            if dealer.value() > 21 {
                println!();
                print_outcome(true, bet);
                bankroll += bet;

                // prompt to play again
                if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
                continue 'game_session
            }
        }

        // print new line
        println!();

        // determine winner
        if player.value() > dealer.value() {
            print_outcome(true, bet);
            bankroll += bet;

            // prompt to play again
            if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
            continue 'game_session
        }
        if player.value() < dealer.value() {
            print_outcome(false, bet);
            bankroll -= bet;

            // prompt to play again
            if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
            continue 'game_session
        }
        if player.value() == dealer.value() {
            println!("{}", format!("It's a push at {}", player.value()).blue().bold());

            // prompt to play again
            if !ask_play_again(bankroll, &mut iteration) { break 'game_session }
            continue 'game_session
        }
    }
}

// play-again helper function
fn ask_play_again(bankroll: u32, i: &mut u16) -> bool {
    loop {
        // increment iteration
        *i += 1;

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

// print outcome loop
fn print_outcome(won: bool, amount: u32) {
    // 1. Define the text and color based on the result
    let msg = 
    if won { "YOU WIN!" }
    else { "YOU LOSE" };

    // 2. Helper closure to apply the color dynamically
    // This allows us to apply "red" or "green" to the whole string at once
    let colorize = |s: String| -> String {
        if won { s.green().to_string() } else { s.red().to_string() }
    };

    // 3. Print the Box
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

// display header
fn display_header(bankroll: u32, iteration: u16) {
    // 1. Clear the screen (optional, but makes it look like a real game app)
    // \x1B[2J clears screen, \x1B[1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    // 2. Choose the Title based on iteration
    let title = if iteration == 0 { "WELCOME TO BLACKJACK" } else { "WELCOME BACK" };

    // 3. Print the Box
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
    println!(); // Add a newline for breathing room
}