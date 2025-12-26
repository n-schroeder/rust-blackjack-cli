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

    // Welcome user
    ui::display_header(game.i, game.bankroll);

    // create temporary bet and assign to game.bet
    let temp_bet: u32 = ui::get_bet(game.bankroll);
    game.bet = temp_bet;

    // initial deal
    game.initial_deal();
}