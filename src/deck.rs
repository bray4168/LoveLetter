use crate::card::Card;

use rand::Rng;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {

        let mut ret = Self {
            cards: vec![],
        };

        Self::init_deck(&mut ret);
        Self::shuffle_deck(&mut ret);
        ret
    }

    fn init_deck(&mut self) {
        // Now that I know rust enums this needs to be a better enum
        let mut num_guards = 5;
        let mut num_priest = 2;
        let mut num_baron = 2;
        let mut num_handmaid = 2;
        let mut num_prince = 2;
        let mut num_king = 1;
        let mut num_countess = 1;
        let mut num_princess = 1;

        // Setup cards
        for _i in 0..num_guards {
            self.cards.push(Card::new_guard());

            if num_priest > 0 {
                self.cards.push(Card::new_priest());
                num_priest -= 1;
            }

            if num_baron > 0 {
                self.cards.push(Card::new_baron());
                num_baron -= 1;
            }

            if num_handmaid > 0 {
                self.cards.push(Card::new_handmaid());
                num_handmaid -= 1;
            }

            if num_prince > 0 {
                self.cards.push(Card::new_prince());
                num_prince -= 1;
            }

            if num_king > 0 {
                self.cards.push(Card::new_king());
                num_king -= 1;
            }

            if num_countess > 0 {
                self.cards.push(Card::new_countess());
                num_countess -= 1;
            }

            if num_princess > 0 {
                self.cards.push(Card::new_princess());
                num_princess -= 1;
            }
        }
    }

    pub fn shuffle_deck(&mut self) {
        let mut new_index: usize;

        for index in 0..self.cards.len() {
            new_index = rand::thread_rng().gen_range(0..self.cards.len()-1);
            self.cards.swap(index, new_index);
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}