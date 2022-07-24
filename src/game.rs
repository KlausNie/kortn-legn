use std::collections::VecDeque;
use crate::Card;

#[derive(Debug)]
pub struct Field {
    topStack1: Vec<Card>,
    topStack2: Vec<Card>,
    bottomStack1: Vec<Card>,
    bottomStack2: Vec<Card>,
    bottomStack3: Vec<Card>,
    notPlayedCards: VecDeque<Card>
}

pub fn initField(cards: Vec<Card>) -> Field {
    Field {
        topStack1: vec![],
        topStack2: vec![],
        bottomStack1: vec![],
        bottomStack2: vec![],
        bottomStack3: vec![],
        notPlayedCards: VecDeque::from(cards)
    }
}

trait Playable {
    fn finished(&self) -> bool;
}

impl Playable for Field {
    fn finished(&self) -> bool {
        return self.topStack1.len() + self.topStack2.len() == 32;
    }
}