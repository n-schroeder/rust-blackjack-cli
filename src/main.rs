//! # Blackjack CLI
//! 
//! A terminal-based implementation of Blackjack written in Rust.
//! 
//! ## Architecture
//! * **Game Loop:** The `main` function handles the game flow (rounds, betting, dealing, turns, and round result).
//! * **Modules:** Relies on `card`, `deck`, `hand`, `game`, and `user_interface` for game logic.
//! 
//! ## Usage
//! Run with `cargo run`. Follow the prompts to bet and play.

mod card;
mod deck;
mod hand;
mod game;
mod user_interface;

use game::{Game, RoundResult};
use user_interface as ui;

/// Initializes the game, handles the main loop, and manages user input
/// until the player runs out of money or chooses to quit.
fn main() {
    let mut game = Game::new(1000);

    // Session loop:
    //
    // Simulates rounds until user quits or bankroll is 0
    'session: loop {
        ui::display_header(game.i, game.bankroll);

        let temp_bet: u32 = ui::get_bet(game.bankroll);
        game.bet = temp_bet;

        game.initial_deal();

        // Gameplay loop:
        //
        // Handles player and dealer turns, checks for busts and blackjacks
        'gameplay: loop {
            println!();
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            if game.player_hand.is_blackjack() { break 'gameplay }

            // --- Player turn loop ---
            'player_turn: loop {
                if game.player_bust() { break 'gameplay }

                if ui::player_hits() {
                    game.deal_to_player();
                    println!();
                    ui::show_hands(&game.player_hand, &game.dealer_hand);
                    continue;
                }
                else { break 'player_turn }
            }

            game.deal_to_dealer();
            
            println!("\n\n=== Dealer's Turn ===\n\n");
            print!("    ");
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // --- Dealer turn loop ---
            loop {
                if game.dealer_hand.is_blackjack() { break 'gameplay }

                if game.dealer_hand.value() >= 17 { break 'gameplay }

                println!("\n    Dealer hits...");
                game.deal_to_dealer();
                print!("    ");
                ui::show_hands(&game.player_hand, &game.dealer_hand);

                if game.dealer_bust() { break 'gameplay }
            }
        }

        println!();

        // --- Winner determination ---
        //
        // Determines winner, updates bankroll, and displays outcome
        let result = game.determine_winner();

        match result {
            RoundResult::PlayerWin => {
                // print player win
                ui::print_outcome(true, game.bet);
                game.bankroll += game.bet;
            },
            RoundResult::DealerWin => {
                // print player loss
                ui::print_outcome(false, game.bet);
                game.bankroll -= game.bet
            },
            RoundResult::PlayerBlackjack => {
                // change win amount
                let payout: u32 = (game.bet * 3) / 2;
                // print blackjack message
                ui::print_blackjack(payout);
            },
            RoundResult::Push => {
                // print push message
                ui::print_push();
            },
        }

        if !ui::play_again(game.bankroll) { break 'session }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    /// Test game initialization
    /// 
    /// Creates new game with bankroll of 500, asserts bankroll, bet, and hand values
    #[test]
    fn test_game_initialization() {
        let game = Game::new(500);
        assert_eq!(game.bankroll, 500);
        assert_eq!(game.bet, 0);
        assert_eq!(game.player_hand.value(), 0);
        assert_eq!(game.dealer_hand.value(), 0);
    }

    /// Test blackjack payout calculation
    /// 
    /// Creates new game, sets bet to 200, simulates blackjack result, and asserts payout amount
    #[test]
    fn test_blackjack_payout() {
        let mut game = Game::new(1000);
        game.bet = 200;
        let result = RoundResult::PlayerBlackjack;
        match result {
            RoundResult::PlayerBlackjack => {
                let payout: u32 = (game.bet * 3) / 2;
                assert_eq!(payout, 300);
            },
            _ => panic!("Expected PlayerBlackjack result"),
        }
    }
}