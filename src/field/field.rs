use std::collections::VecDeque;
use crate::Card;

#[derive(Debug)]
pub struct Field {
    pub top_stack1: Vec<Card>,
    pub top_stack2: Vec<Card>,
    pub bottom_stack1: Vec<Card>,
    pub bottom_stack2: Vec<Card>,
    pub bottom_stack3: Vec<Card>,
    pub not_played_cards: VecDeque<Card>
}