//! # Deck module
//! 
//! This module contains the `Deck` struct and its associated methods
//! for managing a deck of playing cards in blackjack.

use crate::card::{Suit, Rank, Card};
use rand::Rng;

/// Represents a deck containing playing cards
#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Constructor: Loops through all suits and ranks to create a new standard deck of 52 playing cards
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(*suit, *rank));
            }
        }
        Deck { cards }
    }

    /// Shuffle the deck using the Fisher-Yates algorithm
    /// 
    /// Fisher-Yates algorithm randomly shuffles the deck in place
    /// by iterating backwards through the deck and swapping each card
    /// with another card at a random index less than or equal to the current index.
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();

        for i in (1..self.cards.len()).rev() {
            let n = rng.random_range(0..=i);

            self.cards.swap(i,n);
        }
    }

    /// Deal a card from the deck. Removes and returns the top card if available.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
 }

#[cfg(test)]
mod tests {
    use super::*;

    /// Test `Deck` initialization
    /// 
    /// Creates new deck and checks that it contains 52 cards and asserts length == 52
    # [test]
    fn test_deck_initialization() {
        let d = Deck::new();

        assert_eq!(d.cards.len(), 52);
    }

    /// Test `shuffle()` method
    /// 
    /// Creates new deck, shuffles it, and prints both pre- and post-shuffle decks
    /// 
    /// **NOTE:** must manually verify that the order has changed using `cargo test -- --nocapture`
    # [test]
    fn test_deck_shuffle() {
        let mut d = Deck::new();

        for i in 0..52 {
            println!("Item {}: {:?}", i + 1, d.cards[i]);
        }
        println!();
        
        d.shuffle();

        for i in 0..52 {
            println!("Item {}: {:?}", i + 1, d.cards[i]);
        }
    }

    /// Test `deal()` method
    /// 
    /// Creates new deck, deals one card, and compares to expected card.
    # [test]
    fn test_deal() {
        let mut d = Deck::new();
        let card = d.cards[51];
        let return_card = d.deal();
        
        assert_eq!(card, return_card.unwrap())
    }
}