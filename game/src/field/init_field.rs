use std::collections::LinkedList;
use crate::deck::card::Card;
use crate::field::field::Field;
use crate::field::stats::{Stats};

pub fn init_field(cards: Vec<Card>) -> Field {
    Field {
        top_stack1: vec![],
        top_stack2: vec![],
        bottom_stack1: vec![],
        bottom_stack2: vec![],
        bottom_stack3: vec![],
        not_played_cards: cards,
        stats: Stats {
            steps: LinkedList::new()
        }
    }
}