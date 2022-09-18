use std::collections::LinkedList;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;
use crate::deck::card::Card;

#[derive(Debug)]
pub struct Stats {
    pub steps: LinkedList<Step>
}

impl Stats {
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Step {
    pub source: PlaySource,
    pub target: PlayTarget,
    /// should be a reference to a card, but I'm too lazy to deal with the lifetimes
    pub card: Card
}

impl Step {
    pub fn from(source: PlaySource, target: PlayTarget, card: Card) -> Step {
        Step {
            source,
            target,
            card
        }
    }
}