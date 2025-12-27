// Declare existing files
mod card;
mod deck;
mod hand;
mod game;
mod user_interface;

// Imports
use game::Game;
use user_interface as ui;

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
        loop {
            // show hands
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // Check for blackjacks
            if game.player_hand.is_blackjack() { break }

            // Get user decision
            if ui::user_hit() { game.deal_to_player(); }
            if !ui::user_hit() { break }
            if game.user_bust() { break }
        }
    }
}