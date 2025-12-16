// import crates
use crate::card::{Suit, Rank, Card};
use rand::Rng;

#[derive(Debug)]
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
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        for i in (1..self.cards.len()).rev() {
            let n = rng.random_range(0..=i);
            self.cards.swap(i,n);
        }
    }
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
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

    # [test]
    fn test_deal() {
        let mut d = Deck::new();
        let card = d.cards[51];
        let return_card = d.deal();
        
        assert_eq!(card, return_card.unwrap())
    }
}