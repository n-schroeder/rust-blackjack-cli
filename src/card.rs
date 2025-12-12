// Create Suit enum
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

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

// Create Card Struct
struct Card {
    suit: Suit,
    rank: Rank,
}

// Define Card implications
impl Card {
    pub fn value(&self) -> u8 {
        // match enum with Rank value
        match self {
            Rank::TWO => 2,
            Rank::THREE => 3,
            Rank::FOUR => 4,
            Rank::FIVE => 5,
            Rank::SIX => 6,
            Rank::EIGHT => 8,
            Rank::NINE => 9,
            Rank::TEN => 10,
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            Rank::ACE => 11,
        }
    }
}