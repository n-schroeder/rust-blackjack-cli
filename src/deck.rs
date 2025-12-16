// import crates
use crate::card::{Suit, Rank, Card};

 // define Deck struct
 pub struct Deck {
    pub cards: Vec<Card>,
 }

 // define Deck implications
 impl Deck {
    // constructor method
    pub fn new() -> Self {
        // create vector to store cards
        let mut cards = Vec::with_capacity(52);
        // loop through suits and ranks to populate cards vector
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(suit, rank));
            }
        }
        // return cards as Deck using field init shorthand
        Deck { cards }
    }
 }

// TESTING
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
}