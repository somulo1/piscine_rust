use rand::Rng;

<<<<<<< HEAD
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
=======
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamonds,
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    Spade,
    Club,
}

<<<<<<< HEAD
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
=======
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
<<<<<<< HEAD
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
=======
        let value = rng.gen_range(1..5);
        Suit::translate(value)
    }

    //  translate converts an integer value (u8) to a 
    //suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamonds,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
<<<<<<< HEAD
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
=======
        let value = rng.gen_range(1..14);
        Rank::translate(value)
    }

    // translate converts an integer value (u8) to a 
    // rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
        }
    }
}

<<<<<<< HEAD
#[derive(Debug, PartialEq, Clone, Copy)]
=======
#[derive(Debug, Clone, Copy, PartialEq)]
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
<<<<<<< HEAD
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
=======
    card.rank == Rank::Ace && card.suit == Suit::Spade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner_card() {
        let your_card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        let result = winner_card(your_card);
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_winner_card() {
        let your_card2 = Card {
            rank: Rank::Two, // Not Ace
            suit: Suit::Spade, // Not Club
        };
        let result = winner_card(your_card2);
        assert_eq!(result, false);
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    }
}
