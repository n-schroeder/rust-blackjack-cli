mod card;
mod deck;
mod hand;

// imports
use std::io::{self, stdin, Write};

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

        // create Hand to test Display trait
        let mut player_hand = hand::Hand { cards: Vec::new() };
        // add some Cards to Hand
        player_hand.add_card(card::Card::new(card::Suit::HEARTS, card::Rank::ACE));
        player_hand.add_card(card::Card::new(card::Suit::SPADES, card::Rank::TEN));
        // print Hand using Display trait
        println!("Your hand: {}", player_hand); 

        iteration += 1;
        println!();
        println!("iteration: {}", iteration);
        println!();
        play_again = true
    }
}
