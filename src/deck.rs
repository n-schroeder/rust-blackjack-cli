use crate::card::{Suit, Rank, Card};
use rand::Rng;

/// Deck struct
/// 
/// explain
#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Constructor
    /// explain
    pub fn new() -> Self {
        // create vector to store cards
        let mut cards = Vec::with_capacity(52);
        // loop through suits and ranks to populate cards vector
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(*suit, *rank));
            }
        }
        Deck { cards }
    }

    /// explain
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();

        for i in (1..self.cards.len()).rev() {
            let n = rng.random_range(0..=i);

            self.cards.swap(i,n);
        }
    }

    /// explain
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
 }

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_deck_initialization() {
        // setup
        let d = Deck::new();

        // test using assertion
        assert_eq!(d.cards.len(), 52);
    }

    /// explain
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

        // NOTE:
        //
        // must manually test using cargo test -- --nocapture
    }

    /// Test deal method
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