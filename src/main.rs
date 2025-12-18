mod card;
mod deck;
mod hand;

// imports
use std::io::{self, stdin, Write};

use crate::{deck::Deck, hand::Hand};

fn main() {
    // declare vars
    let mut play_again: bool = true;
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
        loop {
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
            if bet == 1 {
                println!("chud");
            }
            // exit betting loop
            break
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
        println!("\nDealer: {}    Player: {}", dealer, player);

        iteration += 1;
        println!();
        println!("iteration: {}", iteration);
        println!();
        play_again = true
    }
}
