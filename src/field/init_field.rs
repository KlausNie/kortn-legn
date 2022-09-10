use std::collections::VecDeque;
use crate::Card;
use crate::field::field::Field;

pub fn init_field(cards: Vec<Card>) -> Field {
    // Field {
    //     top_stack1: cards.clone(),
    //     top_stack2: cards.clone(),
    //     bottom_stack1: cards.clone(),
    //     bottom_stack2: cards.clone(),
    //     bottom_stack3: cards.clone(),
    //     not_played_cards: VecDeque::from(cards)
    // }
    Field {
        top_stack1: vec![],
        top_stack2: vec![],
        bottom_stack1: vec![],
        bottom_stack2: vec![],
        bottom_stack3: vec![],
        not_played_cards: VecDeque::from(cards)
    }
}