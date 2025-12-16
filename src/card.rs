#[derive(Debug, Clone, Copy, PartialEq)]
// Create Suit enum
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {
    pub fn all() -> Vec<Suit> {
        vec![
            Suit::HEARTS,
            Suit::DIAMONDS,
            Suit::CLUBS,
            Suit::SPADES]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// Create Rank enum
pub enum Rank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

// Define Rank implications
impl Rank {
    pub fn value(&self) -> u8 {
    // match enum with Rank value
        match self {
            Rank::TWO => 2,
            Rank::THREE => 3,
            Rank::FOUR => 4,
            Rank::FIVE => 5,
            Rank::SIX => 6,
            Rank::SEVEN => 7,
            Rank::EIGHT => 8,
            Rank::NINE => 9,
            Rank::TEN => 10,
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            Rank::ACE => 11,
        }
    }
    pub fn all() -> Vec<Rank> {
        vec![
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE,
            Rank::SIX,
            Rank::SEVEN,
            Rank::EIGHT,
            Rank::NINE,
            Rank::TEN,
            Rank::JACK,
            Rank::QUEEN,
            Rank::KING,
            Rank::ACE
        ] 
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// Create Card Struct
pub struct Card {
    suit: Suit,
    rank: Rank,
}

// Define Card implications
impl Card {
    // Constructor
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
        }
    }
    // getter methods
    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}