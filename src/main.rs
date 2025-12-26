use crate::game::Game;

mod card;
mod deck;
mod hand;
mod game;
mod user_interface;

fn main() {
    // Create new game
    let game = Game::new(1000);

    // Welcome user
    user_interface::display_header(game.i, game.bankroll);
}