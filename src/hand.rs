//! # Hand module
//! 
//! This module contains the `Hand` struct and its associated methods
//! for managing a player's or dealer's hand in blackjack.

use crate::card::{Rank, Card};
use std::fmt;

/// Uses vector of `Cards` a player's or dealer's hand containing multiple playing cards
#[derive(Debug, PartialEq, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Constructor: Creates a new, empty hand that can hold multiple cards
    pub fn new() -> Self {
        let cards = Vec::new();

        Hand { cards }
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Clear all cards from the hand
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Check if the hand is a blackjack (two cards totaling 21)
    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    /// Calculate the total value of the hand
    /// 
    /// Accounts for Ace being worth 1 or 11
    pub fn value(&self) -> u8 {
        let mut total_value: u8 = 0;
        let mut ace_count: u8 = 0;

        for c in &self.cards {
            total_value += c.value();
            
            if c.rank() == &Rank::ACE {
                ace_count += 1;
            }
        }

        while total_value > 21 && ace_count > 0 {
            total_value -= 10;
            ace_count -= 1;
        }

        total_value
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "[{}]", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::card::Suit;

    /// Test `add_card()` method
    /// 
    /// Adds a card to the hand and asserts that the hand length increases and the card is present
    # [test]
    fn test_add_card() {
        let mut h = Hand::new();
        let c = Card::new(Suit::SPADES, Rank::ACE);

        h.add_card(c);

        assert_eq!(h.cards.len(), 1);
        assert_eq!(h.cards[0], c);
    }

    /// Test `value()` calculation
    /// 
    /// Adds multiple cards, including Aces, and asserts that the hand value is calculated correctly
    # [test]
    fn test_value_calculation() {
        let mut h = Hand::new();

        let jack = Card::new(Suit::CLUBS, Rank::JACK);
        let ace = Card::new(Suit::HEARTS, Rank::ACE);
        let six = Card::new(Suit::DIAMONDS, Rank::SIX);

        h.add_card(jack);
        h.add_card(ace);

        assert_eq!(h.value(), 21);
        assert_eq!(h.is_blackjack(), true);

        h.add_card(six);

        assert_eq!(h.value(), 17);

        h.add_card(ace);

        assert_eq!(h.value(), 18);
    }
}