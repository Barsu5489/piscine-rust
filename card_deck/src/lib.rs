use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)] // Implement Copy and Clone here
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Copy, Clone)] // Implement Copy and Clone here
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=4); // 1 to 4 inclusive
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value! Must be 1..=4."),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=13); // 1 to 13 inclusive
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value! Must be 1..=13."),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)] // Implement Copy and Clone here
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// winner_card takes ownership of the card, so the Copy trait is needed
pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}