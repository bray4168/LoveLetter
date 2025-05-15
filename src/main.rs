mod card;
mod deck;
mod gamestate;
mod player;

use std::io;
use crate::player::Player;
use crate::gamestate::Gamestate;

fn main() {
    println!("Let's make love letter!");
    let mut num_players: String = String::new();
    println!("How many players? (2, 3, 4)...");
    io::stdin().read_line(&mut num_players).unwrap();

    let mut name: String = String::new();
    println!("Input player name...");
    io::stdin().read_line(&mut name).unwrap();

    let mut gamestate = init_gamestate(num_players.trim().parse().unwrap(), String::from(name.trim()));
    println!("{:#?}", gamestate);

    println!("Starting game with {} players.", gamestate.num_players);

    gamestate.print_gametstate();
}

fn init_gamestate(num_players: u32, player_name: String) -> Gamestate {
    let mut gamestate = Gamestate::new(num_players);
    gamestate.players.push(Player::new(player_name, true));
    gamestate.deck.shuffle_deck();

    for num in 1..num_players {
        gamestate.players.push(Player::new("cpu".to_string(), false));
    }

    for player in gamestate.players.iter_mut() {
        match gamestate.deck.draw_card() {
            Some(card) => player.add_card(card),
            None => println!("Deck is empty!")
        }
    }

    return gamestate;
}
