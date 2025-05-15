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

    pub fn print_gametstate(self) {
        for player in self.players {
            println!("{}", player.name);
            if player.is_human {
                for card in player.hand {
                    println!("Card in hand: {}", card.name);
                }
            } else {
                println!("Num cards: {}", player.hand.len());
            }
        }
    }
}