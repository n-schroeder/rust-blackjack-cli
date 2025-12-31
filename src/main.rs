// Declare existing files
mod card;
mod deck;
mod hand;
mod game;
mod user_interface;

// Imports
use game::Game;
use user_interface as ui;

use crate::game::RoundResult;

fn main() {
    // Create new game
    let mut game = Game::new(1000);

    // Session loop to simulate rounds
    'session: loop {
        // Welcome user
        ui::display_header(game.i, game.bankroll);

        // create temporary bet and assign to game.bet
        let temp_bet: u32 = ui::get_bet(game.bankroll);
        game.bet = temp_bet;

        // initial deal
        game.initial_deal();

        // loop to handle player actions
        'gameplay: loop {
            // show hands
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // Check for blackjacks
            if game.player_hand.is_blackjack() { break 'gameplay }

            'player_turn: loop {
                // check player bust
                if game.player_bust() { break 'gameplay }

                // Get user decision and show hands upon user hit
                if ui::player_hits() {
                    game.deal_to_player();
                    ui::show_hands(&game.player_hand, &game.dealer_hand);
                    continue;
                }
                else { break 'player_turn }
            }

            // add dealer's downcard
            game.deal_to_dealer();
            
            // show dealer turn
            println!("\n==== Dealer's Turn ====");

            // dealer turn loop
            loop {
                // show hands
                ui::show_hands(&game.player_hand, &game.dealer_hand);

                // check for blackjack
                if game.dealer_hand.is_blackjack() { break 'gameplay }

                // check if >= 17
                if game.dealer_hand.value() >= 17 { break 'gameplay }

                // deal card to dealer
                println!("\n==== Dealer Hits ====");
                game.deal_to_dealer();

                // check bust
                if game.dealer_bust() { break 'gameplay }
            }
        }

        // Determine winner and handle payout logic
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

        // prompt play again
        if !ui::play_again(game.bankroll) { break 'session }
    }
}