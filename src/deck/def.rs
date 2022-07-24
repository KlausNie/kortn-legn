use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardColor {
    Herz,
    Schell,
    Laab,
    Oachl
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardNumber {
    Siebma = 7,
    Ochta = 8,
    Neina = 9,
    Zehna = 10,
    Unto = 11,
    Obo = 12,
    Kinig = 13,
    Ass = 14
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}

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