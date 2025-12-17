// imports
use crate::card::{Suit, Rank, Card};

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    // constructor
    pub fn new() -> Self {
        // create vector to hold Card objects
        let mut cards = Vec::new();

        // Hand is an object containing a vector that holds Card objects
        Hand { cards }
    }

    // method to add Card to Hand
    pub fn add_card(&mut self, card: Card) {
        // push Card to cards
        self.cards.push(card);
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    # [test]
    fn test_add_card() {
        // create new Hand and Card
        let mut h = Hand::new();
        let c = Card::new(Suit::SPADES, Rank::ACE);

        // add Card to Hand
        h.add_card(c);

        // affirm that h: Hand has length of 1
        assert_eq!(h.cards.len(), 1);

        // affirm that Hand contains ace of spades
        assert_eq!(h.cards[0], c);
    }
}