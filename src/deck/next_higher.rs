use CardNumber::Ochta;
use crate::CardNumber::{Ass, Kinig, Neina, Obo, Siebma, Unto, Zehna};
use crate::deck::card::Card;
use crate::deck::card_number::CardNumber;

pub trait NextHigher<T> {
    fn is_next_higher(&self, other: &T) -> bool;

    fn fits_onto_stack(&self, stack: &Vec<T>) -> bool;
}

/// shitty implementation to check if this card is the next higher one to the other
impl NextHigher<Card> for Card {
    fn is_next_higher(&self, other: &Card) -> bool {
        if self.number == Siebma && other.number == Ass {
            return true
        }
        return match self.number {
            Siebma => {
                other.number == Ass
            }
            Ochta => {
                other.number == Siebma
            }
            Neina => {
                other.number == Ochta
            }
            Zehna => {
                other.number == Neina
            }
            Unto => {
                other.number == Zehna
            }
            Obo => {
                other.number == Unto
            }
            Kinig => {
                other.number == Obo
            }
            Ass => {
                other.number == Kinig
            }
        }
    }

    fn fits_onto_stack(&self, stack: &Vec<Card>) -> bool {
        let last_on_stack = stack.last();
        if last_on_stack.is_none() {
            return true;
        }
        return self.is_next_higher(last_on_stack.unwrap())
    }
}