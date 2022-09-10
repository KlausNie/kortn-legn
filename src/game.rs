use std::collections::VecDeque;
use crate::Card;

#[derive(Debug)]
pub struct Field {
    top_stack1: Vec<Card>,
    top_stack2: Vec<Card>,
    bottom_stack1: Vec<Card>,
    bottom_stack2: Vec<Card>,
    bottom_stack3: Vec<Card>,
    pub not_played_cards: VecDeque<Card>
}

pub fn init_field(cards: Vec<Card>) -> Field {
    Field {
        top_stack1: vec![],
        top_stack2: vec![],
        bottom_stack1: vec![],
        bottom_stack2: vec![],
        bottom_stack3: vec![],
        not_played_cards: VecDeque::from(cards)
    }
}

trait Playable {
    fn finished(&self) -> bool;
}

impl Playable for Field {
    fn finished(&self) -> bool {
        return self.top_stack1.len() + self.top_stack2.len() == 32;
    }
}