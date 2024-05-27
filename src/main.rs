mod card;

use std::io;
use crate::card::Card;

fn main() {
    println!("Let's make love letter!");

    let mut user_input: String = String::new();
    println!("Input player name...");
    io::stdin().read_line(&mut user_input).unwrap();
    println!("User input: {}", user_input);    

    let card: Card = Card {
        name: String::from("Princess"),
        description: String::from("If you discard this card, you are out of the round."),
        value: 8,
        is_face_down: false,
    };
    println!("{:?}", card);
}
