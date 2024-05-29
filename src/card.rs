#[derive(Debug)]
pub struct Card {
    name: String,
    description: String,
    value: u32,
    pub is_face_down: bool,
}

impl Card {
    pub fn new(name: String, description: String, value: u32) -> Self {
        Self {
            name,
            description,
            value,
            is_face_down: false,
        }
    }
}