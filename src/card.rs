use std::fmt;
use colored::*;

/// Suit enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {
    /// explain
    pub fn all() -> &'static [Suit] {
        &[
            Suit::HEARTS,
            Suit::DIAMONDS,
            Suit::CLUBS,
            Suit::SPADES]
    }
}

/// format print for Suit
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

/// Create Rank enum
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Rank {
    /// explain
    pub fn value(&self) -> u8 {
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

    /// explain
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

impl fmt::Display for Rank {
    /// explain
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

/// Card struct
/// 
/// explain
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    /// Constructor
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
        }
    }

    /// explain
    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    /// explain
    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}

impl fmt::Display for Card {
    /// explain
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}