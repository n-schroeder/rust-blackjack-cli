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

    // calculate Hand value
    pub fn value(&self) -> u8 {
        // declare vars
        let mut total_value: u8 = 0;
        let mut ace_count: u8 = 0;

        // iterate through Cards in Hand
        for c in &self.cards {
            // add value of card to total_value
            total_value += c.value();
            
            // if ace, increment ace_count
            if c.rank() == &Rank::ACE {
                ace_count += 1;
            }
        }

        // adjust for bust
        if total_value > 21 && ace_count > 0 {
            // decrement total_value by 10
            total_value -= 10;
            // decrement ace count by 1
            ace_count -= 1;
        }

        // return total_value
        total_value
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