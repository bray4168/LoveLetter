#[derive(Debug)]
pub struct Card {
    name: String,
    description: String,
    value: u32,
    pub is_face_down: bool,
}

impl Card {
    fn new(name: String, description: String, value: u32) -> Self {
        Self {
            name,
            description,
            value,
            is_face_down: false,
        }
    }

    pub fn new_guard() -> Self {
        Self::new(
            String::from("Guard"), 
            String::from("Name a number other than 1 and choose another player. If they have that number in their hand, they are knocked out of the round."),
            1
        )
    }

    pub fn new_priest() -> Self {
        Self::new(
            String::from("Priest"),
            String::from("Choose another player. Look at their hand."),
            2
        )
    }

    pub fn new_baron() -> Self {
        Self::new(
            String::from("Baron"),
            String::from("Choose another player. You secretly compare hands with them. The player with the lower number is out of the round."),
            3
        )
    }
    
    pub fn new_handmaid() -> Self {
        Self::new(
            String::from("Handmaid"),
            String::from("You cannot be chosen as part of teh effects of other players' cards until the start of your next turn."),
            4
        )
    }

    pub fn new_prince() -> Self {
        Self::new(
            String::from("Prince"),
            String::from("Choose any player. They must discard their hand and draw a new card."),
            5
        )
    }

    pub fn new_king() -> Self {
        Self::new(
            String::from("King"),
            String::from("Choose another player. You trade hands with them."),
            6
        )
    }
    
    pub fn new_countess() -> Self {
        Self::new(
            String::from("Countess"), 
            String::from("If you have this card and the King or Prince in your hand, you must discard this card."), 
            7
        )
    }
    
    pub fn new_princess() -> Self {
        Self::new(
            String::from("Princess"), 
            String::from("If you discard this card, you are immediately knocked out of the round."), 
            8
        )
    }
}