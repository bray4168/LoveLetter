use std::io;

fn main() {
    println!("Let's make love letter!");

    let mut user_input: String = String::new();
    println!("Input player name...");
    io::stdin().read_line(&mut user_input).unwrap();
    println!("User input: {}", user_input);    
}
