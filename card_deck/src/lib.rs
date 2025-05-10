use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::rng();
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade][rng.random_range(0..4)]
    }

    pub fn translate(value: u8) -> Suit {
        [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club][(value - 1) as usize]
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::rng();
        let index = rng.random_range(2..=10);
        [
            Self::Ace,
            Self::King,
            Self::Queen,
            Self::Jack,
            Self::Number(index),
        ][rng.random_range(0..=4)]
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => Self::Number(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        true
    } else {
        false
    }
}
