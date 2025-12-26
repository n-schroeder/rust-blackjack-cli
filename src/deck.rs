// import crates
use crate::card::{Suit, Rank, Card};
use rand::Rng;

#[derive(Debug, Clone)]
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
                cards.push(Card::new(*suit, *rank));
            }
        }
        // return cards as Deck using field init shorthand
        Deck { cards }
    }

    // shuffle deck method
    pub fn shuffle(&mut self) {
        // initialize rng
        let mut rng = rand::rng();

        // use for loop to reverse iterate through Deck.cards
        for i in (1..self.cards.len()).rev() {
            // n = random number in range 0 through current i iteration
            let n = rng.random_range(0..=i);

            // swap random index with current index
            self.cards.swap(i,n);
        }
    }

    // deal from deck method
    pub fn deal(&mut self) -> Option<Card> {
        // using pop() returns a wrapped top Card and removes that Card from cards
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

    // test shuffle method
    # [test]
    fn test_deck_shuffle() {
        // create new deck
        let mut d = Deck::new();
        // print new deck
        for i in 0..52 {
            println!("Item {}: {:?}", i + 1, d.cards[i]);
        }
        println!();
        
        // shuffle new deck
        d.shuffle();

        // print new deck
        for i in 0..52 {
            println!("Item {}: {:?}", i + 1, d.cards[i]);
        }

        // NOTE:

        // must manually test using cargo test -- --nocapture
    }

    // test deal card method
    # [test]
    fn test_deal() {
        // create two different Card objects,
        // one grabbing Card straight from index 51
        // and the other grabbing the wrapped return card from .deal()
        let mut d = Deck::new();
        let card = d.cards[51];
        let return_card = d.deal();
        
        // assert that cards are equivalent after unwrapping return card
        assert_eq!(card, return_card.unwrap())
    }
}