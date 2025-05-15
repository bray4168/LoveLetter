use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub is_human: bool,
}

impl Player {
    pub fn new(name: String, is_human: bool) -> Self {
        Self {
            name,
            hand: vec![],
            is_human,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}