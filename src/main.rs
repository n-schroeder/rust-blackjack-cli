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

            // check player bust
            if game.player_bust() { break }

            // Get user decision
            if ui::player_hits() {
                game.deal_to_player(); 
                continue;
            }

            // add dealer's downcard
            game.deal_to_dealer();

            // show dealer hitting
            println!("\n==== Dealer hits ====");
            // show hands
            ui::show_hands(&game.player_hand, &game.dealer_hand);

            // check for blackjack
            if game.dealer_hand.is_blackjack() { break }

            // deal card to dealer
            game.deal_to_dealer();

            // check bust
            if game.dealer_bust() { break }

            // check if >= 17
            if game.dealer_hand.value() >= 17 { break }
        }

        // win decision
        // player bust
        if game.player_bust() || game.dealer_hand.value() > game.player_hand.value() {
            ui::print_outcome(false, game.bet);
            game.bankroll -= game.bet;
        }

        // prompt play again
        if !ui::play_again(game.bankroll) { break 'session }
    }
}