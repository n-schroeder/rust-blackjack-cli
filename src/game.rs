//! Game State Management
//! 
//! This module holds the core logic for my blackjack game,
//! including the `Game` struct, state tracking, and turn phases
//! 
//! It is meant to separate the actual blackjack rules from the main executable.

// imports
use crate::{deck::Deck, hand::*};

/// Game struct
/// 
/// The Game struct contains all data that needs to be managed
pub struct Game {
    deck: Deck,
    player: Hand,
    dealer: Hand,
    pub bankroll: u32,
    pub bet: u32,
    i: u16,
}

/// Game implementations
/// 
/// Contains constructor, 
impl Game {
    /// Constructor
    /// 
    /// Creates new game session.
    pub fn new(starting_bankroll: u32) -> Self {
        // create new deck and shuffle
        let mut deck = Deck::new();
        deck.shuffle();

        // create both player hands
        let player = Hand::new();
        let dealer = Hand::new();

        // return Game
        Game {
            deck,
            player,
            dealer,
            bankroll: starting_bankroll,
            bet: 0,
            i: 0,
        }
    }

    /// Initial Deal
    /// 
    /// First verifies valid bet, then starts round
    /// by dealing cards to `player` and `dealer`
    pub fn initial_deal(&mut self, bet: u32) -> Result<(), String> {
        // verify valid bet
        if bet == 0 {
            return Err("You can't play for free ;)".to_string())
        }
        if bet > self.bankroll {
            return Err("Bad Bet. Insufficient Funds".to_string())
        }
        
        // set bet
        self.bet = bet;

        // clear hands
        self.player.clear();
        self.dealer.clear();

        // deal cards
        self.player.add_card(self.deck.deal().unwrap());
        self.dealer.add_card(self.deck.deal().unwrap());
        self.player.add_card(self.deck.deal().unwrap());
        
        // return success
        Ok(())
    }
}
