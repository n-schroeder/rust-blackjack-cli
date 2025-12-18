mod card;
mod deck;
mod hand;

// imports
use std::io::{self, stdin, Write};

use crate::{deck::Deck, hand::Hand};

fn main() {
    // declare vars
    let mut bet: u32 = 0;
    let mut bankroll: u32 = 1000;
    let iteration: u16 = 0;
    // greet player
    println!("\n\n\n----------------------------------------------");
    println!("Welcome! You have a starting bankroll of $1000");

    // create play-loop
    'game_session: loop {
        // decide if player should be greeted
        if iteration != 0 {
            println!("Welcome! Current bankroll: ${}", bankroll);
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
            if bet == 1 {
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
        println!("\nDealer: {}    Player: {}", dealer, player);

        // check for player blackjack
         if player.is_blackjack() {
            // player blackjack && !dealer blackjack
            if !dealer.is_blackjack() {
                println!("Blackjack! You win!");
                bankroll += bet;
                
                // game loop decision
                if !ask_play_again(bankroll) { break 'game_session }
                continue 'game_session
            }
            // player blackjack && dealer blackjack
            else {
                println!("Unlucky! Dealer has blackjack as well. Push");

                // game loop decision
                if !ask_play_again(bankroll) { break 'game_session }
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
                    println!("\nDealer: {}    Player: {}", dealer, player);

                    // check for bust
                    if player.value() > 21 {
                        println!("Bust! You lose.");
                        bankroll -= bet;
                        if !ask_play_again(bankroll) { break 'game_session }
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
        println!("Dealer's Turn");
        // show hands
        println!("\nDealer: {}    Player: {}", dealer, player);

        // check for dealer blackjack
        if dealer.is_blackjack() {
            println!("Dealer Blackjack! You lose.");
            bankroll -= bet;

            // play again?
            if !ask_play_again(bankroll) { break 'game_session }
            continue 'game_session
        }

        // dealer hit loop
        while dealer.value() < 17 {
            // deal card to dealer
            dealer.add_card(Option::expect(deck.deal(), "No more cards in deck! This will never happen"));
            
            // print
            println!("Dealer hits...");
            println!("\nDealer: {}    Player: {}", dealer, player);
            // handle dealer bust
            if dealer.value() < 21 {
                println!("Conratulations! You win.");
                bankroll += bet;

                // prompt to play again
                if !ask_play_again(bankroll) { break 'game_session }
                continue 'game_session
            }
        }

        // print hand value
        println!("Dealer: {}    Player: {}", dealer.value(), player.value());

        // determine winner
        if player.value() > dealer.value() {
            println!("Conratulations! You win.");
            bankroll += bet;

            // prompt to play again
            if !ask_play_again(bankroll) { break 'game_session }
            continue 'game_session
        }
        if player.value() < dealer.value() {
            println!("You lose!");
            bankroll -= bet;

            // prompt to play again
            if !ask_play_again(bankroll) { break 'game_session }
            continue 'game_session
        }
        if player.value() == dealer.value() {
            println!("It's a push at {}", player.value());

            // prompt to play again
            if !ask_play_again(bankroll) { break 'game_session }
            continue 'game_session
        }
    }
}

// play-again helper function
fn ask_play_again(bankroll: u32) -> bool {
    loop {
        // check balance
        if bankroll == 0 {
            println!("You are out of money! You are not useful to us anymore");
            return false;
        }
        // declare input var
        let mut input = String::new();
        // prompt user

        print!("Play again? (y/n): ");

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
