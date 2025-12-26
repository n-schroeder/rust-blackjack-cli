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
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub bankroll: u32,
    pub bet: u32,
    pub i: u16,
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
        let player_hand = Hand::new();
        let dealer_hand = Hand::new();

        // return Game
        Game {
            deck,
            player_hand,
            dealer_hand,
            bankroll: starting_bankroll,
            bet: 0,
            i: 0,
        }
    }

    /// Initial Deal
    /// 
    /// Deals cards to hands
    pub fn initial_deal(&mut self) {
        // clear hands
        self.player_hand.clear();
        self.dealer_hand.clear();

        // deal cards
        self.player_hand.add_card(self.deck.deal().unwrap());
        self.dealer_hand.add_card(self.deck.deal().unwrap());
        self.player_hand.add_card(self.deck.deal().unwrap());
    }
}
