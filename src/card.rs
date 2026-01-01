use std::fmt;

use colored::*;

// Create Suit enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {
    pub fn all() -> &'static [Suit] {
        &[
            Suit::HEARTS,
            Suit::DIAMONDS,
            Suit::CLUBS,
            Suit::SPADES]
    }
}

// format print for Suit
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::HEARTS => "♥".red(),
            Suit::DIAMONDS => "♦".blue(),
            Suit::CLUBS => "♣".green(),
            Suit::SPADES => "♠".black(),
        };
        write!(f, "{}", symbol)
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
    pub fn all() -> &'static [Rank] {
        &[
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

// format print for Rank
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Rank::TWO => "2",
            Rank::THREE => "3",
            Rank::FOUR => "4",
            Rank::FIVE => "5",
            Rank::SIX => "6",
            Rank::SEVEN => "7",
            Rank::EIGHT => "8",
            Rank::NINE => "9",
            Rank::TEN => "T",
            Rank::JACK => "J",
            Rank::QUEEN => "Q",
            Rank::KING => "K",
            Rank::ACE => "A",
        };
        write!(f, "{}", symbol)
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
    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}

// format print for Card
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}