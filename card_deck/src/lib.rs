use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1..5);  // Fixed genrange to gen_range
        Self::translate(val)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => unreachable!(),  // Use _ to catch any unmatched values
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1..14);  // Fixed genrange to gen_range
        Self::translate(val)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            i @ 2..=10 => Rank::Number(i),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),  // Use _ to catch any unmatched values
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}
