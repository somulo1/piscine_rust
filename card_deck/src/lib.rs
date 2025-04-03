use rand::Rng; // Import rand crate for random generation

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    // Associated function to generate a random Suit
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng(); // Random number generator
        match rng.gen_range(1..=4) { // Generate a number between 1 and 4
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(), // We are sure that the number will be between 1 and 4
        }
    }

    // Associated function to translate an integer value to a Suit
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

#[derive(Debug)]
pub enum Rank {
    Ace,
    Number(u8), // For 2 to 10
    Jack,
    Queen,
    King,
}

impl Rank {
    // Associated function to generate a random Rank
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=13) { // Generate a number between 1 and 13
            1 => Rank::Ace,
            2..=10 => Rank::Number(rng.gen_range(2..=10)), // Number between 2 and 10
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }

    // Associated function to translate an integer value to a Rank
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    // Check if the card is the Ace of Spades
    match (&card.rank, &card.suit) {
        (Rank::Ace, Suit::Spade) => true,
        _ => false,
    }
}

