use crate::card::Card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: vec![],
        }
    }
}