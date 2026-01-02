//! Card module
//! 
//! This module contains the `Card`, `Suit`, and `Rank` structs/enums
//! for representing playing cards in blackjack.

use std::fmt;
use colored::*;

/// Suit enum
/// 
/// Define the four suits in a standard deck of cards
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {
    /// Returns reference to all possible suits
    pub fn all() -> &'static [Suit] {
        &[
            Suit::HEARTS,
            Suit::DIAMONDS,
            Suit::CLUBS,
            Suit::SPADES]
    }
}

impl fmt::Display for Suit {
    /// Format print for Suit
    /// 
    /// Matches suit to corresponding symbol
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

/// Rank enum
/// 
/// Define the ranks in a standard deck of cards
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
    /// Match rank to its corresponding integer value
    /// 
    /// Note: `Ace` is valued at 11 here; `Hand` logic will adjust as needed
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

    /// Returns reference to all possible ranks
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
/// Represents a single card in a standard deck, each with a unique suit and rank
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    /// Constructor
    /// 
    /// Creates a new `Card` with specified suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
        }
    }

    /// Returns a reference to the rank of the card
    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    /// Returns the integer value of the card
    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}