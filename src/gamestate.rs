use crate::deck::Deck;
use crate::player::Player;
use crate::card::Card;

#[derive(Debug)]
pub struct Gamestate {
    pub num_players: u32,
    pub deck: Deck,
    pub players: Vec<Player>,
    pub discard: Vec<Card>,
}

impl Gamestate {
    pub fn new(num_players: u32) -> Self {
        Self {
            num_players,
            deck: Deck::new(),
            players: vec![],
            discard: vec![],
        }
    }
}