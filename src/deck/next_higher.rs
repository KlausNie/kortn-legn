use std::cmp::Ordering;
use crate::deck::card::Card;
use crate::deck::card_number::CardNumber;

pub trait NextHigher<T> {
    fn is_next_higher(&self, other: T) -> bool;
}

impl NextHigher<Card> for Card {
    fn is_next_higher(&self, other: Card) -> bool {
        if self.number == CardNumber::Ass && other.number == CardNumber::Siebma {
            return true
        }
        return self.number.cmp(&other.number) == Ordering::Less
    }
}