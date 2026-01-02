# RUST CLI BLACKJACK

> A basic command-line Blackjack game written in Rust

[![Watch the Demo](https://img.youtube.com/vi/[VIDEO_ID]/0.jpg)](https://www.youtube.com/watch?v=[VIDEO_ID])

^ Placeholder, dont click!

## About

I built this project as a way to expose myself to Rust and force myself to learn the basics through practice,
with a 'just-in-time' learning approach. However, more than that, I made an effort to abide by many professional
practices and conventions such as

- Dedicated module, struct, and method/function documentation (check it out with `cargo doc --open`!)
- Daily [log-style project documentation](./dev-logs), with an atttempt to film video logs daily
- New branches for various tasks like new features, refactors, or testing
- Specific and concise commit messages

## Features

- **Dynamic Ace Calculation:** Aces are correctly valued as either 1 or 11
- **Betting System:** Tracks player's bankroll and ensures valid bets are places
- **Consistent Shuffling:** One standard 52-card deck is consistently shuffled in place using the Fisher-Yates algorithm
- **UI:** Clean user-interface, with a dedicated [UI module](./src/user_interface.rs)

## Technical Overview

I focussed on writing clear, legible, and safe rust code to form good programming habits

[![Watch the Code Overview](https://img.youtube.com/vi/[VIDEO_ID]/0.jpg)](https://www.youtube.com/watch?v=[VIDEO_ID])

^ Placeholder, dont click!

## Key Concepts

- **Modules:** Separation of responsibilities between modules for maximum modularity and legibility
- **Ownership:** Models real-world physics by having `Game` struct own the `Deck` and `Hand`s, while individual `Card`s are
  moved (not copied) from the deck to the hand, guaranteeing that a specific card can never exist in two places at once.

## Play My Game!

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/n-schroeder/rust-blackjack-cli.git
    cd rust-blackjack-cli
    ```

2.  **Run the game:**
    ```bash
    cargo run
    ```

3.  **Run the tests:**
    ```bash
    cargo test
    ```
