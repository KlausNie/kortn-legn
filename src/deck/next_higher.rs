use CardNumber::Ochta;
use crate::CardNumber::{Ass, Kinig, Neina, Obo, Siebma, Unto, Zehna};
use crate::deck::card::Card;
use crate::deck::card_number::CardNumber;

pub trait NextHigher<T> {
    fn is_next_higher(&self, other: T) -> bool;
}

impl NextHigher<Card> for Card {
    fn is_next_higher(&self, other: Card) -> bool {
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
}