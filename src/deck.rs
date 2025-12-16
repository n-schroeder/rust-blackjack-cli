use crate::card::Card;
//#[derive(Debug, Clone, Copy, PartialEq)]
 pub struct Deck {
    deck: Vec<Card>,
 }

 impl Deck {
    pub fn new() -> Self {
        Deck {
            deck: Vec::new(),
        }
    }
    
 }