//! Game State Management
//! 
//! This module holds the core logic for my blackjack game,
//! including the `Game` struct, state tracking, and turn phases
//! 
//! It is meant to separate the actual blackjack rules from the main executable.

// imports
use crate::{deck::Deck, hand::*};

/// Round Result enum
/// 
/// Contains all possible round results
#[derive(Debug, PartialEq)]
pub enum RoundResult {
    PlayerWin,
    PlayerBlackjack, // Distinct because it pays 3:2
    DealerWin,
    Push,
}

/// Game struct
/// 
/// The Game struct contains all data that needs to be managed
#[derive(Debug, PartialEq)]
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

    pub fn deal_to_player(&mut self) {
        self.player_hand.add_card(self.deck.deal().unwrap());
    }

    pub fn deal_to_dealer(&mut self) {
        self.dealer_hand.add_card(self.deck.deal().unwrap());
    }

    pub fn player_bust(&self) -> bool { self.player_hand.value() > 21 }

    pub fn dealer_bust(&self) -> bool { self.dealer_hand.value() > 21 }

    // Win Decision Logic
    pub fn determine_winner(&self) -> RoundResult {
        // Check for bust
        if self.player_bust() {
            return RoundResult::DealerWin
        } else if self.dealer_bust() {
            return RoundResult::PlayerWin
        } 
        
        // Check for blackjacks
        else if self.player_hand.is_blackjack() && self.dealer_hand.is_blackjack() {
            return RoundResult::Push
        } else if self.player_hand.is_blackjack() {
            return RoundResult::PlayerBlackjack
        } else if self.dealer_hand.is_blackjack() {
            return RoundResult::DealerWin
        }

        // Compare hand values
        else if self.player_hand.value() > self.dealer_hand.value() {
            return RoundResult::PlayerWin
        } else if self.player_hand.value() < self.dealer_hand.value() {
            return RoundResult::DealerWin
        } else {
            return RoundResult::Push
        }
    }
}

#[cfg(test)]

mod test {
    use crate::card::*;

    use super::*;

    #[test]
    fn test_player_blackjack_win() {
        // create new game
        let mut game = Game::new(1000);

        // give player blackjack
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        // give dealer 21, but !blackjack
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));

        // verify player win
        assert_eq!(game.determine_winner(), RoundResult::PlayerBlackjack);
    }

    #[test]
    fn test_dealer_blackjack_win() {
        // create new game
        let mut game = Game::new(1000);

        // give player 21, but !blackjack
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));

        // give dealer blackjack
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        // verify dealer win
        assert_eq!(game.determine_winner(), RoundResult::DealerWin);
    }

    #[test]
    fn test_push() {
        let mut game = Game::new(1000);
        
        // Both have 20
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        let result = game.determine_winner();
        assert!(matches!(result, RoundResult::Push));
    }

    #[test]
    fn test_player_bust() {
        let mut game = Game::new(1000);
        // give player bust hand
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::KING));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TWO));

        // give dealer valid hand
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::NINE));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::SEVEN));

        // verify dealer win
        assert_eq!(game.determine_winner(), RoundResult::DealerWin);
    }

    #[test]
    fn test_dealer_bust() {
        let mut game = Game::new(1000);
        // give player valid hand
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::NINE));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::SEVEN));

        // give dealer bust hand
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::KING));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TWO));

        // verify player win
        assert_eq!(game.determine_winner(), RoundResult::PlayerWin);
    }

    #[test]
    fn test_backjack_push() {
        let mut game = Game::new(1000);
        
        // Both have blackjack
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::KING));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::QUEEN));

        let result = game.determine_winner();
        assert!(matches!(result, RoundResult::Push));
    }
}