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
        'user_action: loop {
            // show hands
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // Check for blackjacks
            if game.player_hand.is_blackjack() { break 'user_action }

            // Get user decision
            if ui::player_hit() { game.deal_to_player(); }
            else { break 'user_action }

            // check player bust
            if game.player_bust() { break 'user_action }
        }

        // add dealer's downcard
        game.deal_to_dealer();

        // dealer action loop
        'dealer_action: loop {
            // show hands
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // check for blackjack
            if game.dealer_hand.is_blackjack() { break 'dealer_action }

            // deal card to dealer
            game.deal_to_dealer();

            // check bust
            if game.dealer_bust() { break 'dealer_action }

            // check if >= 17
            if game.dealer_hand.value() >= 17 { break 'dealer_action }
        }
    }
}