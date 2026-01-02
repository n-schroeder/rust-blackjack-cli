//! Game State Management
//! 
//! This module holds the core logic for my blackjack game,
//! including the `Game` struct, state tracking, and turn phases
//! 
//! It is meant to separate the actual blackjack rules from the main executable.

use crate::{deck::Deck, hand::*};

/// Round Result enum
/// 
/// Contains all possible round results
#[derive(Debug, PartialEq)]
pub enum RoundResult {
    PlayerWin,
    PlayerBlackjack,
    DealerWin,
    Push,
}

/// Game struct
/// 
/// The Game struct contains all data that needs to be 
/// managed and tracked throughout a blackjack game session
#[derive(Debug, PartialEq)]
pub struct Game {
    deck: Deck,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub bankroll: u32,
    pub bet: u32,
    pub i: u16,
}

impl Game {
    /// Constructor
    /// 
    /// Creates new game session with starting bankroll and initializes deck and hands
    pub fn new(starting_bankroll: u32) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let player_hand = Hand::new();
        let dealer_hand = Hand::new();

        Game {
            deck,
            player_hand,
            dealer_hand,
            bankroll: starting_bankroll,
            bet: 0,
            i: 0,
        }
    }

    /// Initial deal that deals two cards to player and one to dealer to represent dealer's face-up card
    pub fn initial_deal(&mut self) {
        self.player_hand.clear();
        self.dealer_hand.clear();

        self.player_hand.add_card(self.deck.deal().unwrap());
        self.dealer_hand.add_card(self.deck.deal().unwrap());
        self.player_hand.add_card(self.deck.deal().unwrap());
    }

    /// Deal a card to the player
    pub fn deal_to_player(&mut self) {
        self.player_hand.add_card(self.deck.deal().unwrap());
    }

    /// Deal a card to the dealer
    pub fn deal_to_dealer(&mut self) {
        self.dealer_hand.add_card(self.deck.deal().unwrap());
    }

    /// Check if player has busted
    pub fn player_bust(&self) -> bool { self.player_hand.value() > 21 }

    /// Check if dealer has busted
    pub fn dealer_bust(&self) -> bool { self.dealer_hand.value() > 21 }

    /// Win Decision Logic
    /// 
    /// Determines the winner of the round based on blackjack rules
    /// 
    /// Follows correct priority:
    /// 1. Busts
    /// 2. Blackjacks
    /// 3. Hand Value Comparison
    /// 
    /// Returns `RoundResult` enum indicating outcome
    pub fn determine_winner(&self) -> RoundResult {
        if self.player_bust() {
            return RoundResult::DealerWin
        } else if self.dealer_bust() {
            return RoundResult::PlayerWin
        } 
        
        else if self.player_hand.is_blackjack() && self.dealer_hand.is_blackjack() {
            return RoundResult::Push
        } else if self.player_hand.is_blackjack() {
            return RoundResult::PlayerBlackjack
        } else if self.dealer_hand.is_blackjack() {
            return RoundResult::DealerWin
        }

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

    /// Test player blackjack win scenario
    /// 
    /// Gives player a blackjack and dealer a non-blackjack 21, asserts player win
    #[test]
    fn test_player_blackjack_win() {
        let mut game = Game::new(1000);

        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));

        assert_eq!(game.determine_winner(), RoundResult::PlayerBlackjack);
    }

    /// Test dealer blackjack win scenario
    /// 
    /// Gives dealer a blackjack and player a non-blackjack 21, asserts dealer win
    #[test]
    fn test_dealer_blackjack_win() {
        let mut game = Game::new(1000);

        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::ACE));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));

        assert_eq!(game.determine_winner(), RoundResult::DealerWin);
    }

    /// Test push scenario
    /// 
    /// Gives both player and dealer hands of equal value, asserts push result
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

    /// Test player bust scenario
    /// 
    /// Gives player a busting hand and dealer a valid hand, asserts dealer win
    #[test]
    fn test_player_bust() {
        let mut game = Game::new(1000);

        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::KING));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TWO));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::NINE));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::SEVEN));

        assert_eq!(game.determine_winner(), RoundResult::DealerWin);
    }

    /// Test dealer bust scenario
    /// 
    /// Gives dealer a busting hand and player a valid hand, asserts player win
    #[test]
    fn test_dealer_bust() {
        let mut game = Game::new(1000);

        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::NINE));
        game.player_hand.add_card(Card::new(Suit::DIAMONDS, Rank::SEVEN));

        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TEN));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::KING));
        game.dealer_hand.add_card(Card::new(Suit::DIAMONDS, Rank::TWO));

        assert_eq!(game.determine_winner(), RoundResult::PlayerWin);
    }

    /// Test blackjack push scenario
    /// 
    /// Gives both player and dealer blackjacks, asserts push result
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