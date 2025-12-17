// imports
use crate::card::Card;

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